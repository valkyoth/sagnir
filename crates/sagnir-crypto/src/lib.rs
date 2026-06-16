#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::SagnirError;

pub const SIGNATURE_BYTES_MAX: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignatureAlgorithm {
    Ed25519,
    MlDsa,
    HybridClassicalPq,
    Future(u16),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignatureEnvelope<'a> {
    algorithm: SignatureAlgorithm,
    bytes: &'a [u8],
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
