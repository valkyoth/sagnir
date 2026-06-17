#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::SagnirError;

pub const CONFIDENCE_MAX: u16 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FactKind {
    RealmCreated,
    WorldCreated,
    ChangeOpened,
    ChangeSealed,
    TestRecorded,
    ReviewApproved,
    WorldPromoted,
    TaintMarked,
    QuarantineApplied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConfidenceScore(u16);

impl ConfidenceScore {
    pub const fn new(value: u16) -> Result<Self, SagnirError> {
        if value > CONFIDENCE_MAX {
            return Err(SagnirError::InvalidValue);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> u16 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_accepts_maximum() {
        assert_eq!(
            ConfidenceScore::new(CONFIDENCE_MAX).map(ConfidenceScore::get),
            Ok(CONFIDENCE_MAX)
        );
    }

    #[test]
    fn confidence_rejects_overflow() {
        assert_eq!(
            ConfidenceScore::new(CONFIDENCE_MAX + 1),
            Err(SagnirError::InvalidValue)
        );
    }
}
