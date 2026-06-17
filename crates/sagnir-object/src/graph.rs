use sagnir_core::SagnirError;

use crate::{ObjectId, ObjectType};

/// Recommended maximum object graph entries for one in-memory admission pass.
pub const OBJECT_GRAPH_ENTRIES_MAX: usize = 256;
/// Recommended maximum object graph references for one in-memory admission pass.
pub const OBJECT_GRAPH_REFS_MAX: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectGraphEntry {
    id: ObjectId,
}

impl ObjectGraphEntry {
    #[must_use]
    pub const fn new(id: ObjectId) -> Self {
        Self { id }
    }

    #[must_use]
    pub const fn id(self) -> ObjectId {
        self.id
    }

    #[must_use]
    pub const fn object_type(self) -> ObjectType {
        self.id.object_type()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectReference {
    source: ObjectId,
    target: ObjectId,
    target_type: ObjectType,
}

impl ObjectReference {
    pub fn new(
        source: ObjectId,
        target: ObjectId,
        target_type: ObjectType,
    ) -> Result<Self, SagnirError> {
        if target.object_type() != target_type {
            return Err(SagnirError::InvalidValue);
        }

        Ok(Self {
            source,
            target,
            target_type,
        })
    }

    #[must_use]
    pub const fn source(self) -> ObjectId {
        self.source
    }

    #[must_use]
    pub const fn target(self) -> ObjectId {
        self.target
    }

    #[must_use]
    pub const fn target_type(self) -> ObjectType {
        self.target_type
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectGraphReport {
    Complete,
    MissingReference(ObjectReference),
    Cycle(ObjectId),
    InvalidEntry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitState {
    Unseen,
    Visiting,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitError {
    Cycle(ObjectId),
    InvalidEntry,
}

/// Fixed-capacity object relationship graph.
///
/// `N` and `R` are compile-time storage bounds. Production callers should stay
/// at or below `OBJECT_GRAPH_ENTRIES_MAX` and `OBJECT_GRAPH_REFS_MAX` unless a
/// release gate explicitly admits a larger stack and runtime budget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectGraph<const N: usize, const R: usize> {
    entries: [Option<ObjectGraphEntry>; N],
    references: [Option<ObjectReference>; R],
    entry_len: usize,
    reference_len: usize,
}

impl<const N: usize, const R: usize> ObjectGraph<N, R> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: [None; N],
            references: [None; R],
            entry_len: 0,
            reference_len: 0,
        }
    }

    pub fn insert_entry(&mut self, entry: ObjectGraphEntry) -> Result<(), SagnirError> {
        if self.entry_len >= N || self.node_index(entry.id()).is_some() {
            return Err(SagnirError::InvalidValue);
        }

        self.entries[self.entry_len] = Some(entry);
        self.entry_len += 1;
        Ok(())
    }

    pub fn insert_reference(&mut self, reference: ObjectReference) -> Result<(), SagnirError> {
        if self.reference_len >= R
            || self.node_index(reference.source()).is_none()
            || reference.source().object_type() == ObjectType::Blob
        {
            return Err(SagnirError::InvalidValue);
        }

        self.references[self.reference_len] = Some(reference);
        self.reference_len += 1;
        Ok(())
    }

    #[must_use]
    pub fn contains_entry(&self, id: ObjectId) -> bool {
        self.node_index(id).is_some()
    }

    #[must_use]
    pub const fn entry_len(&self) -> usize {
        self.entry_len
    }

    #[must_use]
    pub const fn reference_len(&self) -> usize {
        self.reference_len
    }

    #[must_use]
    pub fn verify(&self) -> ObjectGraphReport {
        let mut index = 0;
        while index < self.reference_len {
            if let Some(reference) = self.references[index]
                && self.node_index(reference.target()).is_none()
            {
                return ObjectGraphReport::MissingReference(reference);
            }
            index += 1;
        }

        let mut states = [VisitState::Unseen; N];
        let mut entry_index = 0;
        while entry_index < self.entry_len {
            if let Err(error) = self.visit(entry_index, &mut states) {
                return match error {
                    VisitError::Cycle(id) => ObjectGraphReport::Cycle(id),
                    VisitError::InvalidEntry => ObjectGraphReport::InvalidEntry,
                };
            }
            entry_index += 1;
        }

        ObjectGraphReport::Complete
    }

    pub fn contains_path(&self, source: ObjectId, target: ObjectId) -> Result<bool, SagnirError> {
        let source_index = self.node_index(source).ok_or(SagnirError::InvalidValue)?;
        if self.node_index(target).is_none() {
            return Err(SagnirError::InvalidValue);
        }

        let mut stack = [0_usize; N];
        let mut queued = [false; N];
        let mut seen = [false; N];
        let mut depth = 1;
        stack[0] = source_index;
        queued[source_index] = true;

        while depth > 0 {
            depth -= 1;
            let index = stack[depth];
            if seen[index] {
                continue;
            }
            seen[index] = true;

            let Some(source_id) = self.entry_id(index) else {
                continue;
            };
            if source_id == target {
                return Ok(true);
            }

            let mut reference_index = 0;
            while reference_index < self.reference_len {
                if let Some(reference) = self.references[reference_index]
                    && reference.source() == source_id
                    && let Some(target_index) = self.node_index(reference.target())
                    && !seen[target_index]
                    && !queued[target_index]
                    && depth < N
                {
                    stack[depth] = target_index;
                    queued[target_index] = true;
                    depth += 1;
                }
                reference_index += 1;
            }
        }

        Ok(false)
    }

    fn node_index(&self, id: ObjectId) -> Option<usize> {
        let mut index = 0;
        while index < self.entry_len {
            if let Some(entry) = self.entries[index]
                && entry.id() == id
            {
                return Some(index);
            }
            index += 1;
        }
        None
    }

    fn visit(&self, index: usize, states: &mut [VisitState; N]) -> Result<(), VisitError> {
        match states[index] {
            VisitState::Visiting => {
                let id = self.entry_id(index).ok_or(VisitError::InvalidEntry)?;
                return Err(VisitError::Cycle(id));
            }
            VisitState::Done => return Ok(()),
            VisitState::Unseen => {}
        }

        let start_id = self.entry_id(index).ok_or(VisitError::InvalidEntry)?;
        let mut stack = [(0_usize, 0_usize); N];
        let mut depth = 1;
        stack[0] = (index, 0);
        states[index] = VisitState::Visiting;

        while depth > 0 {
            let frame_index = depth - 1;
            let (node_index, mut reference_index) = stack[frame_index];
            let source = self
                .entry_id(node_index)
                .ok_or(VisitError::Cycle(start_id))?;
            let mut advanced = false;

            while reference_index < self.reference_len {
                stack[frame_index].1 = reference_index + 1;
                if let Some(reference) = self.references[reference_index]
                    && reference.source() == source
                    && let Some(target_index) = self.node_index(reference.target())
                {
                    match states[target_index] {
                        VisitState::Visiting => {
                            return Err(VisitError::Cycle(
                                self.entry_id(target_index).unwrap_or(source),
                            ));
                        }
                        VisitState::Done => {}
                        VisitState::Unseen => {
                            if depth >= N {
                                return Err(VisitError::Cycle(reference.target()));
                            }
                            states[target_index] = VisitState::Visiting;
                            stack[depth] = (target_index, 0);
                            depth += 1;
                            advanced = true;
                            break;
                        }
                    }
                }
                reference_index += 1;
            }

            if !advanced {
                states[node_index] = VisitState::Done;
                depth -= 1;
            }
        }

        Ok(())
    }

    fn entry_id(&self, index: usize) -> Option<ObjectId> {
        self.entries
            .get(index)
            .and_then(|entry| entry.map(|entry| entry.id()))
    }
}

impl<const N: usize, const R: usize> Default for ObjectGraph<N, R> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
