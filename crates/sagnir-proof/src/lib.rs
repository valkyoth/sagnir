#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::SagnirError;
use sagnir_policy::PolicyResult;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProofStatus {
    /// Reserved for the live verifier.
    ///
    /// Production code cannot construct this status through `ProofReport::new`.
    /// Until the verifier is implemented, only tests can mint the token needed
    /// by `ProofReport::verified`.
    Verified,
    MissingEvidence,
    Invalid,
}

/// Opaque capability proving that the live verifier accepted a proof report.
///
/// This token has no production constructor in the current scaffold. It keeps
/// call sites honest by preventing ordinary code from marking a report as
/// verified before the verifier exists.
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

    /// Build a verified report after the live verifier has minted a token.
    ///
    /// In the current scaffold, this is reachable only from tests because
    /// `VerificationToken` has no production constructor.
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
