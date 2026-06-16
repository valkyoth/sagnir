#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyResult {
    Allow,
    Deny,
    RequireProof,
    Quarantine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObligationSet(u16);

impl ObligationSet {
    pub const NONE: Self = Self(0);
    pub const REQUIRE_TEST_EVIDENCE: Self = Self(1 << 0);
    pub const REQUIRE_REVIEW: Self = Self(1 << 1);

    #[must_use]
    pub const fn bits(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn has(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PolicyDecision {
    result: PolicyResult,
    obligations: ObligationSet,
}

impl PolicyDecision {
    #[must_use]
    pub const fn new(result: PolicyResult, obligations: ObligationSet) -> Self {
        Self {
            result,
            obligations,
        }
    }

    #[must_use]
    pub const fn result(self) -> PolicyResult {
        self.result
    }

    #[must_use]
    pub const fn obligations(self) -> ObligationSet {
        self.obligations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_decision_exposes_aggregate_result() {
        let decision =
            PolicyDecision::new(PolicyResult::RequireProof, ObligationSet::REQUIRE_REVIEW);
        assert_eq!(decision.result(), PolicyResult::RequireProof);
        assert!(decision.obligations().has(ObligationSet::REQUIRE_REVIEW));
        assert_eq!(decision.obligations().bits(), 2);
    }
}
