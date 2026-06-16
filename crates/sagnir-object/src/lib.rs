#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::{ID_BYTES, TypedId, constant_time_bytes_eq};

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

#[derive(Clone, Copy, Debug, Eq)]
pub struct ObjectId {
    algorithm: HashAlgorithm,
    object_type: ObjectType,
    digest: [u8; ID_BYTES],
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

pub const fn parse_hash_algorithm(raw: u16) -> Result<HashAlgorithm, sagnir_core::SagnirError> {
    match raw {
        1 => Ok(HashAlgorithm::Sha256),
        _ => Err(sagnir_core::SagnirError::InvalidValue),
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

    /// Constant-time digest equality for signature verification, proof
    /// verification, and bundle acceptance checks.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        self.algorithm == other.algorithm
            && self.object_type == other.object_type
            && constant_time_bytes_eq(&self.digest, &other.digest)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateRootRef {
    state_id: TypedId,
    content_root: ObjectId,
    format_version: u16,
}

impl StateRootRef {
    #[must_use]
    pub const fn new(state_id: TypedId, content_root: ObjectId, format_version: u16) -> Self {
        Self {
            state_id,
            content_root,
            format_version,
        }
    }

    #[must_use]
    pub const fn format_version(self) -> u16 {
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
    use sagnir_core::{IdKind, TypedId};

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
        let root = StateRootRef::new(state_id, object_id, 1);
        assert_eq!(root.format_version(), 1);
    }

    #[test]
    fn unknown_hash_algorithm_fails_closed() {
        assert_eq!(
            parse_hash_algorithm(999),
            Err(sagnir_core::SagnirError::InvalidValue)
        );
    }
}
