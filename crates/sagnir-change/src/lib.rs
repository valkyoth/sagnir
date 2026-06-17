#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::{BoundedName, ChangeId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ChangeState {
    Open,
    Sealed,
    Quarantined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChangeRef<'a> {
    id: ChangeId,
    title: BoundedName<'a>,
    state: ChangeState,
}

impl<'a> ChangeRef<'a> {
    #[must_use]
    pub const fn new(id: ChangeId, title: BoundedName<'a>, state: ChangeState) -> Self {
        Self { id, title, state }
    }

    #[must_use]
    pub const fn id(self) -> ChangeId {
        self.id
    }

    #[must_use]
    pub const fn state(self) -> ChangeState {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sagnir_core::{BoundedName, ID_BYTES};

    #[test]
    fn change_ref_records_state() {
        let id = ChangeId::new([4; ID_BYTES]);
        let title = BoundedName::new("seal-object-format");
        let change = title.map(|name| ChangeRef::new(id, name, ChangeState::Open));
        assert_eq!(change.map(ChangeRef::state), Ok(ChangeState::Open));
    }
}
