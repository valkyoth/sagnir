#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub const FORMAT_VERSION: FormatVersion = FormatVersion::new(1);
pub const ID_BYTES: usize = 32;
pub const NAME_MAX_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatVersion(u16);

impl FormatVersion {
    #[must_use]
    pub const fn new(value: u16) -> Self {
        Self(value)
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

#[derive(Clone, Copy, Debug, Eq)]
pub struct TypedId {
    kind: IdKind,
    bytes: [u8; ID_BYTES],
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

    /// Constant-time byte comparison for security-sensitive verification.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        self.kind == other.kind && constant_time_bytes_eq(&self.bytes, &other.bytes)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SagnirError {
    EmptyName,
    NameTooLong,
    InvalidNameByte,
    BufferTooSmall,
    InvalidValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedName<'a> {
    value: &'a str,
}

impl<'a> BoundedName<'a> {
    /// Creates a bounded name from path-like ASCII-safe input.
    ///
    /// Platform note: `.saga` path segments are rejected with ASCII
    /// case-folding so case-insensitive filesystems cannot alias Sagnir
    /// control paths into source-state names.
    pub fn new(value: &'a str) -> Result<Self, SagnirError> {
        if value.is_empty() {
            return Err(SagnirError::EmptyName);
        }
        if value.len() > NAME_MAX_BYTES {
            return Err(SagnirError::NameTooLong);
        }
        if !value.bytes().all(valid_name_byte) {
            return Err(SagnirError::InvalidNameByte);
        }
        if value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == ".." || is_saga_segment(part))
        {
            return Err(SagnirError::InvalidNameByte);
        }
        Ok(Self { value })
    }

    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.value
    }
}

#[must_use]
pub const fn valid_name_byte(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'/' | b'.')
}

#[must_use]
pub fn constant_time_bytes_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut diff = 0_u8;
    let mut index = 0;
    while index < left.len() {
        diff |= left[index] ^ right[index];
        index += 1;
    }
    diff == 0
}

#[must_use]
pub fn is_saga_segment(segment: &str) -> bool {
    segment.eq_ignore_ascii_case(".saga")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_name_accepts_world_path() {
        let name = BoundedName::new("draft/wal-recovery");
        assert_eq!(name.map(BoundedName::as_str), Ok("draft/wal-recovery"));
    }

    #[test]
    fn bounded_name_rejects_empty() {
        assert_eq!(BoundedName::new(""), Err(SagnirError::EmptyName));
    }

    #[test]
    fn bounded_name_rejects_path_traversal_segments() {
        assert_eq!(
            BoundedName::new("../outside"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("draft/../main"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("draft//main"),
            Err(SagnirError::InvalidNameByte)
        );
    }

    #[test]
    fn bounded_name_rejects_case_folded_saga_control_path() {
        assert_eq!(
            BoundedName::new(".Saga/config"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("sub/.SAGA"),
            Err(SagnirError::InvalidNameByte)
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
}
