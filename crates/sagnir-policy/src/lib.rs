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
pub struct PolicyDecision {
    result: PolicyResult,
    obligations: u16,
}

impl PolicyDecision {
    #[must_use]
    pub const fn new(result: PolicyResult, obligations: u16) -> Self {
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
    pub const fn obligations(self) -> u16 {
        self.obligations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_decision_exposes_aggregate_result() {
        let decision = PolicyDecision::new(PolicyResult::RequireProof, 2);
        assert_eq!(decision.result(), PolicyResult::RequireProof);
        assert_eq!(decision.obligations(), 2);
    }
}
