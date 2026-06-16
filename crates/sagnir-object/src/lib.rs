#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::{ID_BYTES, TypedId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HashAlgorithm {
    Sha256,
    Future(u16),
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ObjectId {
    algorithm: HashAlgorithm,
    object_type: ObjectType,
    digest: [u8; ID_BYTES],
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
}
