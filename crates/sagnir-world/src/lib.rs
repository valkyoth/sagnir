#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::{BoundedName, WorldId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WorldKind {
    Local,
    Draft,
    Review,
    Staging,
    Production,
    Audit,
    Simulation,
    Agent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PromotionPreflight {
    Allowed,
    MissingProof,
    Conflict,
    PolicyDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorldRef<'a> {
    id: WorldId,
    name: BoundedName<'a>,
    kind: WorldKind,
}

impl<'a> WorldRef<'a> {
    #[must_use]
    pub const fn new(id: WorldId, name: BoundedName<'a>, kind: WorldKind) -> Self {
        Self { id, name, kind }
    }

    #[must_use]
    pub const fn id(self) -> WorldId {
        self.id
    }

    #[must_use]
    pub const fn kind(self) -> WorldKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sagnir_core::{BoundedName, ID_BYTES};

    #[test]
    fn world_ref_records_kind() {
        let id = WorldId::new([5; ID_BYTES]);
        let name = BoundedName::new("draft/object-format");
        let world = name.map(|value| WorldRef::new(id, value, WorldKind::Draft));
        assert_eq!(world.map(WorldRef::kind), Ok(WorldKind::Draft));
    }
}
