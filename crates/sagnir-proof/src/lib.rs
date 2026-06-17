#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::SagnirError;
use sagnir_policy::PolicyResult;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProofStatus {
    Verified,
    MissingEvidence,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerificationToken {
    _private: (),
}

impl VerificationToken {
    #[cfg(test)]
    const fn for_test() -> Self {
        Self { _private: () }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProofReport {
    status: ProofStatus,
    policy: PolicyResult,
}

impl ProofReport {
    pub const fn new(status: ProofStatus, policy: PolicyResult) -> Result<Self, SagnirError> {
        match status {
            ProofStatus::Verified => Err(SagnirError::InvalidValue),
            ProofStatus::MissingEvidence | ProofStatus::Invalid => Ok(Self { status, policy }),
        }
    }

    #[must_use]
    pub const fn verified(_token: VerificationToken, policy: PolicyResult) -> Self {
        Self {
            status: ProofStatus::Verified,
            policy,
        }
    }

    #[must_use]
    pub const fn status(self) -> ProofStatus {
        self.status
    }

    #[must_use]
    pub const fn policy(self) -> PolicyResult {
        self.policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_report_records_status() {
        let report = ProofReport::new(ProofStatus::MissingEvidence, PolicyResult::RequireProof);
        assert_eq!(
            report.map(|value| (value.status(), value.policy())),
            Ok((ProofStatus::MissingEvidence, PolicyResult::RequireProof))
        );
    }

    #[test]
    fn verified_report_requires_token() {
        assert_eq!(
            ProofReport::new(ProofStatus::Verified, PolicyResult::Allow),
            Err(SagnirError::InvalidValue)
        );

        let report = ProofReport::verified(VerificationToken::for_test(), PolicyResult::Allow);
        assert_eq!(report.status(), ProofStatus::Verified);
        assert_eq!(report.policy(), PolicyResult::Allow);
    }
}
