use core::hint::black_box;

use crate::SagnirError;

pub const FORMAT_VERSION: FormatVersion = FormatVersion::current();
pub const ID_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatVersion(u16);

impl FormatVersion {
    pub const CURRENT_RAW: u16 = 1;

    #[must_use]
    pub const fn current() -> Self {
        Self(Self::CURRENT_RAW)
    }

    pub const fn try_new(value: u16) -> Result<Self, SagnirError> {
        match value {
            Self::CURRENT_RAW => Ok(Self(value)),
            _ => Err(SagnirError::InvalidValue),
        }
    }

    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IdKind {
    Realm,
    World,
    Change,
    Revision,
    State,
    Fact,
    Object,
    Operation,
    Bundle,
}

#[derive(Clone, Copy, Eq)]
pub struct TypedId {
    kind: IdKind,
    bytes: [u8; ID_BYTES],
}

impl core::fmt::Debug for TypedId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TypedId")
            .field("kind", &self.kind)
            .field(
                "bytes",
                &format_args!("[{} bytes redacted]", self.bytes.len()),
            )
            .finish()
    }
}

impl core::hash::Hash for TypedId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.kind, state);
        core::hash::Hash::hash(&self.bytes, state);
    }
}

impl PartialEq for TypedId {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl TypedId {
    #[must_use]
    pub const fn new(kind: IdKind, bytes: [u8; ID_BYTES]) -> Self {
        Self { kind, bytes }
    }

    #[must_use]
    pub const fn kind(self) -> IdKind {
        self.kind
    }

    #[must_use]
    pub const fn bytes(self) -> [u8; ID_BYTES] {
        self.bytes
    }

    /// Timing-hardened byte comparison for security-sensitive verification
    /// scaffolds. See [`constant_time_bytes_eq`] for the current guarantee.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        let kind_eq = (self.kind == other.kind) as u8;
        let bytes_eq = black_box(constant_time_bytes_eq(&self.bytes, &other.bytes)) as u8;

        black_box(kind_eq & bytes_eq) == 1
    }
}

macro_rules! define_id_wrapper {
    ($name:ident, $kind:expr) => {
        #[derive(Clone, Copy, Eq, PartialEq, Hash)]
        pub struct $name(TypedId);

        impl $name {
            pub const KIND: IdKind = $kind;

            #[must_use]
            pub const fn new(bytes: [u8; ID_BYTES]) -> Self {
                Self(TypedId::new(Self::KIND, bytes))
            }

            pub const fn from_typed(id: TypedId) -> Result<Self, SagnirError> {
                match id.kind() {
                    Self::KIND => Ok(Self(id)),
                    _ => Err(SagnirError::InvalidValue),
                }
            }

            #[must_use]
            pub const fn typed(self) -> TypedId {
                self.0
            }

            #[must_use]
            pub const fn bytes(self) -> [u8; ID_BYTES] {
                self.0.bytes()
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
    };
}

define_id_wrapper!(RealmId, IdKind::Realm);
define_id_wrapper!(WorldId, IdKind::World);
define_id_wrapper!(ChangeId, IdKind::Change);
define_id_wrapper!(RevisionId, IdKind::Revision);
define_id_wrapper!(StateId, IdKind::State);
define_id_wrapper!(FactId, IdKind::Fact);
define_id_wrapper!(ObjectIdRef, IdKind::Object);
define_id_wrapper!(OperationId, IdKind::Operation);
define_id_wrapper!(BundleId, IdKind::Bundle);

/// Accumulates XOR differences across two equal-length byte slices.
///
/// This uses [`core::hint::black_box`] to reduce compiler-inserted early exits.
/// It is not a formal constant-time guarantee. Before live signature or HMAC
/// verification relies on this path, Sagnir must admit `subtle` or an
/// equivalent formally specified primitive through the dependency policy.
#[must_use]
pub fn constant_time_bytes_eq(left: &[u8], right: &[u8]) -> bool {
    let len_eq = left.len() == right.len();
    let compare_len = left.len().max(right.len());
    let mut diff = 0_u8;
    let mut index = 0;
    while index < compare_len {
        let left_byte = if index < left.len() { left[index] } else { 0 };
        let right_byte = if index < right.len() { right[index] } else { 0 };
        diff |= left_byte ^ right_byte;
        index += 1;
    }
    black_box(diff) == 0 && len_eq
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::format;

    #[test]
    fn format_version_accepts_current_only() {
        assert_eq!(FORMAT_VERSION.get(), 1);
        assert_eq!(FormatVersion::try_new(1), Ok(FORMAT_VERSION));
        assert_eq!(FormatVersion::try_new(0), Err(SagnirError::InvalidValue));
        assert_eq!(
            FormatVersion::try_new(u16::MAX),
            Err(SagnirError::InvalidValue)
        );
    }

    #[test]
    fn constant_time_bytes_eq_checks_full_slice() {
        assert!(constant_time_bytes_eq(&[1, 2, 3], &[1, 2, 3]));
        assert!(!constant_time_bytes_eq(&[1, 2, 3], &[1, 2, 4]));
        assert!(!constant_time_bytes_eq(&[1, 2, 3], &[1, 2]));
    }

    #[test]
    fn typed_id_keeps_kind_and_bytes() {
        let id = TypedId::new(IdKind::World, [7; ID_BYTES]);
        assert_eq!(id.kind(), IdKind::World);
        assert_eq!(id.bytes(), [7; ID_BYTES]);
    }

    #[test]
    fn typed_id_has_constant_time_equality_api() {
        let left = TypedId::new(IdKind::World, [7; ID_BYTES]);
        let right = TypedId::new(IdKind::World, [7; ID_BYTES]);
        let different_kind = TypedId::new(IdKind::Change, [7; ID_BYTES]);
        let different_bytes = TypedId::new(IdKind::World, [8; ID_BYTES]);

        assert!(left.ct_eq(&right));
        assert!(!left.ct_eq(&different_kind));
        assert!(!left.ct_eq(&different_bytes));
    }

    #[test]
    fn typed_id_debug_redacts_bytes() {
        let id = TypedId::new(IdKind::Object, [9; ID_BYTES]);
        assert_eq!(
            format!("{id:?}"),
            "TypedId { kind: Object, bytes: [32 bytes redacted] }"
        );
    }

    #[test]
    fn id_wrappers_preserve_kind() {
        let realm = RealmId::new([1; ID_BYTES]);
        let world = WorldId::new([2; ID_BYTES]);
        let change = ChangeId::new([3; ID_BYTES]);
        let revision = RevisionId::new([4; ID_BYTES]);
        let state = StateId::new([5; ID_BYTES]);
        let fact = FactId::new([6; ID_BYTES]);
        let object = ObjectIdRef::new([7; ID_BYTES]);
        let operation = OperationId::new([8; ID_BYTES]);
        let bundle = BundleId::new([9; ID_BYTES]);

        assert_eq!(realm.typed().kind(), IdKind::Realm);
        assert_eq!(world.typed().kind(), IdKind::World);
        assert_eq!(change.typed().kind(), IdKind::Change);
        assert_eq!(revision.typed().kind(), IdKind::Revision);
        assert_eq!(state.typed().kind(), IdKind::State);
        assert_eq!(fact.typed().kind(), IdKind::Fact);
        assert_eq!(object.typed().kind(), IdKind::Object);
        assert_eq!(operation.typed().kind(), IdKind::Operation);
        assert_eq!(bundle.typed().kind(), IdKind::Bundle);
    }

    #[test]
    fn id_wrappers_reject_wrong_kind() {
        let world = TypedId::new(IdKind::World, [2; ID_BYTES]);

        assert_eq!(WorldId::from_typed(world).map(WorldId::typed), Ok(world));
        assert_eq!(RealmId::from_typed(world), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn id_wrapper_debug_redacts_bytes() {
        let id = RealmId::new([1; ID_BYTES]);
        assert_eq!(
            format!("{id:?}"),
            "RealmId(TypedId { kind: Realm, bytes: [32 bytes redacted] })"
        );
    }
}
