#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_policy::PolicyResult;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProofStatus {
    Verified,
    MissingEvidence,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProofReport {
    status: ProofStatus,
    policy: PolicyResult,
}

impl ProofReport {
    #[must_use]
    pub const fn new(status: ProofStatus, policy: PolicyResult) -> Self {
        Self { status, policy }
    }

    #[must_use]
    pub const fn status(self) -> ProofStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_report_records_status() {
        let report = ProofReport::new(ProofStatus::MissingEvidence, PolicyResult::RequireProof);
        assert_eq!(report.status(), ProofStatus::MissingEvidence);
    }
}
