use sagnir_core::{FormatVersion, ID_BYTES, SagnirError, TypedId, constant_time_bytes_eq};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum HashAlgorithm {
    Sha256,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ObjectType {
    Blob,
    Tree,
    StateRoot,
    Change,
    ChangeRevision,
    World,
    Fact,
    Operation,
    Bundle,
}

#[derive(Clone, Copy, Eq)]
pub struct ObjectId {
    algorithm: HashAlgorithm,
    object_type: ObjectType,
    digest: [u8; ID_BYTES],
}

impl core::fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ObjectId")
            .field("algorithm", &self.algorithm)
            .field("object_type", &self.object_type)
            .field(
                "digest",
                &format_args!("[{} bytes redacted]", self.digest.len()),
            )
            .finish()
    }
}

impl core::hash::Hash for ObjectId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.algorithm, state);
        core::hash::Hash::hash(&self.object_type, state);
        core::hash::Hash::hash(&self.digest, state);
    }
}

impl PartialEq for ObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

pub const fn parse_hash_algorithm(raw: u16) -> Result<HashAlgorithm, SagnirError> {
    match raw {
        1 => Ok(HashAlgorithm::Sha256),
        _ => Err(SagnirError::InvalidValue),
    }
}

pub const fn parse_object_type(raw: u16) -> Result<ObjectType, SagnirError> {
    match raw {
        1 => Ok(ObjectType::Blob),
        2 => Ok(ObjectType::Tree),
        3 => Ok(ObjectType::StateRoot),
        4 => Ok(ObjectType::Change),
        5 => Ok(ObjectType::ChangeRevision),
        6 => Ok(ObjectType::World),
        7 => Ok(ObjectType::Fact),
        8 => Ok(ObjectType::Operation),
        9 => Ok(ObjectType::Bundle),
        _ => Err(SagnirError::InvalidValue),
    }
}

impl ObjectId {
    #[must_use]
    pub const fn new(
        algorithm: HashAlgorithm,
        object_type: ObjectType,
        digest: [u8; ID_BYTES],
    ) -> Self {
        Self {
            algorithm,
            object_type,
            digest,
        }
    }

    #[must_use]
    pub const fn object_type(self) -> ObjectType {
        self.object_type
    }

    #[must_use]
    pub const fn digest(self) -> [u8; ID_BYTES] {
        self.digest
    }

    /// Timing-hardened digest equality for verification scaffolds. Before live
    /// signature or HMAC verification relies on this path, Sagnir must admit a
    /// formally specified constant-time primitive.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        let algorithm_eq = constant_time_bytes_eq(
            &hash_algorithm_raw(self.algorithm).to_le_bytes(),
            &hash_algorithm_raw(other.algorithm).to_le_bytes(),
        ) as u8;
        let object_type_eq = constant_time_bytes_eq(
            &object_type_raw(self.object_type).to_le_bytes(),
            &object_type_raw(other.object_type).to_le_bytes(),
        ) as u8;
        let digest_eq = constant_time_bytes_eq(&self.digest, &other.digest) as u8;

        (algorithm_eq & object_type_eq & digest_eq) == 1
    }
}

const fn hash_algorithm_raw(algorithm: HashAlgorithm) -> u16 {
    match algorithm {
        HashAlgorithm::Sha256 => 1,
    }
}

pub(crate) const fn object_type_raw(object_type: ObjectType) -> u16 {
    match object_type {
        ObjectType::Blob => 1,
        ObjectType::Tree => 2,
        ObjectType::StateRoot => 3,
        ObjectType::Change => 4,
        ObjectType::ChangeRevision => 5,
        ObjectType::World => 6,
        ObjectType::Fact => 7,
        ObjectType::Operation => 8,
        ObjectType::Bundle => 9,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateRootRef {
    state_id: TypedId,
    content_root: ObjectId,
    format_version: FormatVersion,
}

impl StateRootRef {
    #[must_use]
    pub const fn new(
        state_id: TypedId,
        content_root: ObjectId,
        format_version: FormatVersion,
    ) -> Self {
        Self {
            state_id,
            content_root,
            format_version,
        }
    }

    #[must_use]
    pub const fn format_version(self) -> FormatVersion {
        self.format_version
    }
}

#[must_use]
pub const fn domain_tag(object_type: ObjectType) -> &'static [u8] {
    match object_type {
        ObjectType::Blob => b"sagnir.object.v1.blob",
        ObjectType::Tree => b"sagnir.object.v1.tree",
        ObjectType::StateRoot => b"sagnir.object.v1.state-root",
        ObjectType::Change => b"sagnir.object.v1.change",
        ObjectType::ChangeRevision => b"sagnir.object.v1.change-revision",
        ObjectType::World => b"sagnir.object.v1.world",
        ObjectType::Fact => b"sagnir.object.v1.fact",
        ObjectType::Operation => b"sagnir.object.v1.operation",
        ObjectType::Bundle => b"sagnir.object.v1.bundle",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sagnir_core::{FORMAT_VERSION, IdKind, TypedId};
    extern crate std;
    use std::{format, string::String};

    #[test]
    fn object_id_keeps_type_separate_from_digest() {
        let id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [1; ID_BYTES]);
        assert_eq!(id.object_type(), ObjectType::Tree);
        assert_eq!(id.digest(), [1; ID_BYTES]);
    }

    #[test]
    fn object_id_has_constant_time_equality_api() {
        let left = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [1; ID_BYTES]);
        let right = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [1; ID_BYTES]);
        let different_type = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Blob, [1; ID_BYTES]);
        let different_digest =
            ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [2; ID_BYTES]);

        assert!(left.ct_eq(&right));
        assert!(!left.ct_eq(&different_type));
        assert!(!left.ct_eq(&different_digest));
    }

    #[test]
    fn domain_tags_are_type_separated() {
        assert_ne!(domain_tag(ObjectType::Blob), domain_tag(ObjectType::Tree));
    }

    #[test]
    fn state_root_records_format_version() {
        let state_id = TypedId::new(IdKind::State, [2; ID_BYTES]);
        let object_id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [3; ID_BYTES]);
        let root = StateRootRef::new(state_id, object_id, FORMAT_VERSION);
        assert_eq!(root.format_version(), FORMAT_VERSION);
        assert_eq!(root.format_version().get(), 1);
    }

    #[test]
    fn unknown_hash_algorithm_fails_closed() {
        assert_eq!(parse_hash_algorithm(999), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_type_parser_fails_closed() {
        assert_eq!(parse_object_type(1), Ok(ObjectType::Blob));
        assert_eq!(parse_object_type(9), Ok(ObjectType::Bundle));
        assert_eq!(parse_object_type(0), Err(SagnirError::InvalidValue));
        assert_eq!(parse_object_type(10), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_id_debug_redacts_digest() {
        let id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Blob, [4; ID_BYTES]);
        assert_eq!(
            format!("{id:?}"),
            String::from(
                "ObjectId { algorithm: Sha256, object_type: Blob, digest: [32 bytes redacted] }"
            )
        );
    }
}
