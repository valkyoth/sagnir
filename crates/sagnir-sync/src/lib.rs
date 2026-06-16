#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::SagnirError;

pub const MAX_OBJECTS_PER_BUNDLE: u64 = 1_000_000;
pub const MAX_FACTS_PER_BUNDLE: u64 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BundleKind {
    World,
    Change,
    FactRange,
    FullRealm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BundleManifest {
    kind: BundleKind,
    object_count: u64,
    fact_count: u64,
}

impl BundleManifest {
    pub const fn new(
        kind: BundleKind,
        object_count: u64,
        fact_count: u64,
    ) -> Result<Self, SagnirError> {
        if object_count > MAX_OBJECTS_PER_BUNDLE || fact_count > MAX_FACTS_PER_BUNDLE {
            return Err(SagnirError::InvalidValue);
        }

        Ok(Self {
            kind,
            object_count,
            fact_count,
        })
    }

    #[must_use]
    pub const fn total_items(self) -> u64 {
        self.object_count.saturating_add(self.fact_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_counts_total_items() {
        let manifest = BundleManifest::new(BundleKind::World, 3, 5);
        assert_eq!(manifest.map(BundleManifest::total_items), Ok(8));
    }

    #[test]
    fn manifest_rejects_oversized_counts() {
        assert_eq!(
            BundleManifest::new(BundleKind::World, MAX_OBJECTS_PER_BUNDLE + 1, 0),
            Err(SagnirError::InvalidValue)
        );
        assert_eq!(
            BundleManifest::new(BundleKind::World, 0, MAX_FACTS_PER_BUNDLE + 1),
            Err(SagnirError::InvalidValue)
        );
    }
}
