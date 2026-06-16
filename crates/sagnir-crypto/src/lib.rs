#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::{SagnirError, constant_time_bytes_eq};

pub const SIGNATURE_BYTES_MAX: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    Ed25519,
    MlDsa,
    HybridClassicalPq,
}

#[derive(Clone, Copy, Eq)]
pub struct SignatureEnvelope<'a> {
    algorithm: SignatureAlgorithm,
    bytes: &'a [u8],
}

impl PartialEq for SignatureEnvelope<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

impl<'a> SignatureEnvelope<'a> {
    pub fn new(algorithm: SignatureAlgorithm, bytes: &'a [u8]) -> Result<Self, SagnirError> {
        if bytes.is_empty() || bytes.len() > SIGNATURE_BYTES_MAX {
            return Err(SagnirError::InvalidValue);
        }
        Ok(Self { algorithm, bytes })
    }

    #[must_use]
    pub const fn algorithm(self) -> SignatureAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub const fn len(self) -> usize {
        self.bytes.len()
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }

    /// Timing-hardened signature byte equality for verification scaffolds.
    /// Before live signature verification relies on this path, Sagnir must
    /// admit a formally specified constant-time primitive.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        self.algorithm == other.algorithm && constant_time_bytes_eq(self.bytes, other.bytes)
    }
}

impl core::fmt::Debug for SignatureEnvelope<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SignatureEnvelope")
            .field("algorithm", &self.algorithm)
            .field(
                "bytes",
                &format_args!("[{} bytes redacted]", self.bytes.len()),
            )
            .finish()
    }
}

pub const fn parse_signature_algorithm(raw: u16) -> Result<SignatureAlgorithm, SagnirError> {
    match raw {
        1 => Ok(SignatureAlgorithm::Ed25519),
        2 => Ok(SignatureAlgorithm::MlDsa),
        3 => Ok(SignatureAlgorithm::HybridClassicalPq),
        _ => Err(SagnirError::InvalidValue),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::{format, string::String};

    #[test]
    fn signature_envelope_rejects_empty_bytes() {
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &[]),
            Err(SagnirError::InvalidValue)
        );
    }

    #[test]
    fn signature_envelope_keeps_algorithm() {
        let envelope = SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &[1, 2]);
        assert_eq!(
            envelope.map(SignatureEnvelope::algorithm),
            Ok(SignatureAlgorithm::HybridClassicalPq)
        );
    }

    #[test]
    fn signature_envelope_has_constant_time_equality_api() {
        let left = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &[1, 2, 3]);
        let right = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &[1, 2, 3]);
        let different_algorithm = SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &[1, 2, 3]);
        let different_bytes = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &[1, 2, 4]);

        assert!(left.is_ok_and(|value| right.is_ok_and(|right| value.ct_eq(&right))));
        assert!(
            left.is_ok_and(|value| { different_algorithm.is_ok_and(|right| !value.ct_eq(&right)) })
        );
        assert!(left.is_ok_and(|value| different_bytes.is_ok_and(|right| !value.ct_eq(&right))));
    }

    #[test]
    fn unknown_signature_algorithm_fails_closed() {
        assert_eq!(
            parse_signature_algorithm(999),
            Err(SagnirError::InvalidValue)
        );
    }

    #[test]
    fn debug_redacts_signature_bytes() {
        let envelope = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &[1, 2, 3]);
        let debug = envelope.map(|value| format!("{value:?}"));
        assert_eq!(
            debug,
            Ok(String::from(
                "SignatureEnvelope { algorithm: Ed25519, bytes: [3 bytes redacted] }"
            ))
        );
    }
}
