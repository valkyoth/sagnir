#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

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
    #[must_use]
    pub const fn new(kind: BundleKind, object_count: u64, fact_count: u64) -> Self {
        Self {
            kind,
            object_count,
            fact_count,
        }
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
        assert_eq!(manifest.total_items(), 8);
    }
}
