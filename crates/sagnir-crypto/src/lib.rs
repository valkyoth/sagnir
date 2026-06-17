#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::{SagnirError, constant_time_bytes_choice};
use sanitization::{SecureSanitize, sanitize_bytes};

pub const ED25519_SIGNATURE_BYTES: usize = 64;
pub const ML_DSA_44_SIGNATURE_BYTES: usize = 2_420;
pub const ML_DSA_65_SIGNATURE_BYTES: usize = 3_309;
pub const ML_DSA_87_SIGNATURE_BYTES: usize = 4_627;
pub const ML_DSA_SIGNATURE_BYTES_MIN: usize = ML_DSA_44_SIGNATURE_BYTES;
pub const ML_DSA_SIGNATURE_BYTES_MAX: usize = ML_DSA_87_SIGNATURE_BYTES;
pub const HYBRID_SIGNATURE_BYTES_MIN: usize = ED25519_SIGNATURE_BYTES + ML_DSA_SIGNATURE_BYTES_MIN;
pub const HYBRID_SIGNATURE_BYTES_MAX: usize = ED25519_SIGNATURE_BYTES + ML_DSA_SIGNATURE_BYTES_MAX;
pub const SIGNATURE_BYTES_MAX: usize = HYBRID_SIGNATURE_BYTES_MAX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    Ed25519,
    MlDsa,
    HybridClassicalPq,
}

#[derive(Clone, Eq)]
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
        if !valid_signature_len_for(algorithm, bytes.len()) {
            return Err(SagnirError::InvalidValue);
        }
        Ok(Self { algorithm, bytes })
    }

    #[must_use]
    pub const fn algorithm(&self) -> SignatureAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Timing-hardened signature byte equality for verification scaffolds.
    /// Before live signature verification relies on this path, Sagnir must
    /// admit a formally specified constant-time primitive.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        let algorithm_eq = constant_time_bytes_choice(
            &signature_algorithm_raw(self.algorithm).to_le_bytes(),
            &signature_algorithm_raw(other.algorithm).to_le_bytes(),
        );
        let bytes_eq = constant_time_bytes_choice(self.bytes, other.bytes);

        (algorithm_eq & bytes_eq).into()
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

#[must_use]
pub const fn max_signature_bytes_for(algorithm: SignatureAlgorithm) -> usize {
    match algorithm {
        SignatureAlgorithm::Ed25519 => ED25519_SIGNATURE_BYTES,
        SignatureAlgorithm::MlDsa => ML_DSA_SIGNATURE_BYTES_MAX,
        SignatureAlgorithm::HybridClassicalPq => HYBRID_SIGNATURE_BYTES_MAX,
    }
}

#[must_use]
pub const fn min_signature_bytes_for(algorithm: SignatureAlgorithm) -> usize {
    match algorithm {
        SignatureAlgorithm::Ed25519 => ED25519_SIGNATURE_BYTES,
        SignatureAlgorithm::MlDsa => ML_DSA_SIGNATURE_BYTES_MIN,
        SignatureAlgorithm::HybridClassicalPq => HYBRID_SIGNATURE_BYTES_MIN,
    }
}

#[must_use]
pub const fn valid_signature_len_for(algorithm: SignatureAlgorithm, len: usize) -> bool {
    match algorithm {
        SignatureAlgorithm::Ed25519 => len == ED25519_SIGNATURE_BYTES,
        SignatureAlgorithm::MlDsa => valid_ml_dsa_signature_len(len),
        SignatureAlgorithm::HybridClassicalPq => valid_hybrid_signature_len(len),
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

const fn signature_algorithm_raw(algorithm: SignatureAlgorithm) -> u16 {
    match algorithm {
        SignatureAlgorithm::Ed25519 => 1,
        SignatureAlgorithm::MlDsa => 2,
        SignatureAlgorithm::HybridClassicalPq => 3,
    }
}

const fn valid_ml_dsa_signature_len(len: usize) -> bool {
    matches!(
        len,
        ML_DSA_44_SIGNATURE_BYTES | ML_DSA_65_SIGNATURE_BYTES | ML_DSA_87_SIGNATURE_BYTES
    )
}

const fn valid_hybrid_signature_len(len: usize) -> bool {
    len == HYBRID_SIGNATURE_BYTES_MIN
        || len == ED25519_SIGNATURE_BYTES + ML_DSA_65_SIGNATURE_BYTES
        || len == HYBRID_SIGNATURE_BYTES_MAX
}

/// Owned, fixed-size signature buffer. Cleared on drop via `sanitization`.
///
/// Stack cost: `SIGNATURE_BYTES_MAX` bytes, currently 4691 bytes for
/// Ed25519 plus ML-DSA-87. Callers on constrained stacks must box this type or
/// use a future allocation-backed signature container.
pub struct OwnedSignature {
    algorithm: SignatureAlgorithm,
    len: usize,
    bytes: [u8; SIGNATURE_BYTES_MAX],
}

const _: () = assert!(
    core::mem::size_of::<OwnedSignature>() <= 4_800,
    "OwnedSignature stack budget changed; update security controls before release"
);

#[derive(Clone, Eq, PartialEq)]
pub struct HybridSignatureEnvelope<'a> {
    classical_bytes: &'a [u8],
    pq_bytes: &'a [u8],
}

impl core::fmt::Debug for HybridSignatureEnvelope<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HybridSignatureEnvelope")
            .field(
                "classical_bytes",
                &format_args!("[{} bytes redacted]", self.classical_bytes.len()),
            )
            .field(
                "pq_bytes",
                &format_args!("[{} bytes redacted]", self.pq_bytes.len()),
            )
            .finish()
    }
}

impl<'a> HybridSignatureEnvelope<'a> {
    /// Parses hybrid signature bytes into classical and post-quantum slices.
    ///
    /// Security: this is a length-admission parser only. It does not verify
    /// either signature component and must not be used as a security decision
    /// boundary without a live verifier that binds both components to the same
    /// message and domain.
    pub fn parse(raw: &'a [u8]) -> Result<Self, SagnirError> {
        if raw.len() < ED25519_SIGNATURE_BYTES {
            return Err(SagnirError::InvalidValue);
        }

        let (classical_bytes, pq_bytes) = raw.split_at(ED25519_SIGNATURE_BYTES);
        if !valid_ml_dsa_signature_len(pq_bytes.len()) {
            return Err(SagnirError::InvalidValue);
        }

        Ok(Self {
            classical_bytes,
            pq_bytes,
        })
    }

    #[must_use]
    pub const fn classical_bytes(&self) -> &'a [u8] {
        self.classical_bytes
    }

    #[must_use]
    pub const fn pq_bytes(&self) -> &'a [u8] {
        self.pq_bytes
    }
}

impl OwnedSignature {
    pub fn new(algorithm: SignatureAlgorithm, bytes: &[u8]) -> Result<Self, SagnirError> {
        if !valid_signature_len_for(algorithm, bytes.len()) {
            return Err(SagnirError::InvalidValue);
        }

        let mut owned = Self {
            algorithm,
            len: bytes.len(),
            bytes: [0; SIGNATURE_BYTES_MAX],
        };
        owned.bytes[..bytes.len()].copy_from_slice(bytes);
        Ok(owned)
    }

    #[must_use]
    pub const fn algorithm(&self) -> SignatureAlgorithm {
        self.algorithm
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_envelope(&self) -> Result<SignatureEnvelope<'_>, SagnirError> {
        SignatureEnvelope::new(self.algorithm, &self.bytes[..self.len])
    }
}

impl SecureSanitize for OwnedSignature {
    fn secure_sanitize(&mut self) {
        sanitize_bytes(&mut self.bytes);
        self.algorithm = SignatureAlgorithm::Ed25519;
        self.len = 0;
    }
}

impl Drop for OwnedSignature {
    fn drop(&mut self) {
        self.secure_sanitize();
    }
}

impl core::fmt::Debug for OwnedSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("OwnedSignature")
            .field("algorithm", &self.algorithm)
            .field("bytes", &format_args!("[{} bytes redacted]", self.len))
            .finish()
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
    fn signature_envelope_enforces_algorithm_specific_bounds() {
        let ed25519 = [0_u8; ED25519_SIGNATURE_BYTES];
        let too_small_ed25519 = [0_u8; ED25519_SIGNATURE_BYTES - 1];
        let too_large_ed25519 = [0_u8; ED25519_SIGNATURE_BYTES + 1];
        let too_small_ml_dsa = [0_u8; ML_DSA_SIGNATURE_BYTES_MIN - 1];
        let ml_dsa_44 = [0_u8; ML_DSA_SIGNATURE_BYTES_MIN];
        let invalid_ml_dsa_gap = [0_u8; ML_DSA_SIGNATURE_BYTES_MIN + 1];
        let ml_dsa_65 = [0_u8; ML_DSA_65_SIGNATURE_BYTES];
        let ml_dsa_87 = [0_u8; ML_DSA_SIGNATURE_BYTES_MAX];
        let too_small_hybrid = [0_u8; HYBRID_SIGNATURE_BYTES_MIN - 1];
        let hybrid_min = [0_u8; HYBRID_SIGNATURE_BYTES_MIN];
        let invalid_hybrid_gap = [0_u8; HYBRID_SIGNATURE_BYTES_MIN + 1];
        let hybrid_middle = [0_u8; ED25519_SIGNATURE_BYTES + ML_DSA_65_SIGNATURE_BYTES];
        let hybrid = [0_u8; HYBRID_SIGNATURE_BYTES_MAX];

        assert!(SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &ed25519).is_ok());
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &too_small_ed25519),
            Err(SagnirError::InvalidValue)
        );
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &too_large_ed25519),
            Err(SagnirError::InvalidValue)
        );
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &too_small_ml_dsa),
            Err(SagnirError::InvalidValue)
        );
        assert!(SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &ml_dsa_44).is_ok());
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &invalid_ml_dsa_gap),
            Err(SagnirError::InvalidValue)
        );
        assert!(SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &ml_dsa_65).is_ok());
        assert!(SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &ml_dsa_87).is_ok());
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &too_small_hybrid),
            Err(SagnirError::InvalidValue)
        );
        assert!(SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &hybrid_min).is_ok());
        assert_eq!(
            SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &invalid_hybrid_gap),
            Err(SagnirError::InvalidValue)
        );
        assert!(
            SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &hybrid_middle).is_ok()
        );
        assert!(SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &hybrid).is_ok());
    }

    #[test]
    fn signature_envelope_keeps_algorithm() {
        let bytes = [0_u8; HYBRID_SIGNATURE_BYTES_MIN];
        let envelope = SignatureEnvelope::new(SignatureAlgorithm::HybridClassicalPq, &bytes);
        assert_eq!(
            envelope.map(|value| value.algorithm()),
            Ok(SignatureAlgorithm::HybridClassicalPq)
        );
    }

    #[test]
    fn signature_envelope_has_constant_time_equality_api() {
        let bytes = [1_u8; ED25519_SIGNATURE_BYTES];
        let mut changed = bytes;
        changed[ED25519_SIGNATURE_BYTES - 1] = 2;
        let ml_dsa = [1_u8; ML_DSA_SIGNATURE_BYTES_MIN];
        let left = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &bytes);
        let right = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &bytes);
        let different_algorithm = SignatureEnvelope::new(SignatureAlgorithm::MlDsa, &ml_dsa);
        let different_bytes = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &changed);

        assert!(
            left.as_ref()
                .is_ok_and(|value| right.as_ref().is_ok_and(|right| value.ct_eq(right)))
        );
        assert!(left.as_ref().is_ok_and(|value| {
            different_algorithm
                .as_ref()
                .is_ok_and(|right| !value.ct_eq(right))
        }));
        assert!(left.as_ref().is_ok_and(|value| {
            different_bytes
                .as_ref()
                .is_ok_and(|right| !value.ct_eq(right))
        }));
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
        let bytes = [1_u8; ED25519_SIGNATURE_BYTES];
        let envelope = SignatureEnvelope::new(SignatureAlgorithm::Ed25519, &bytes);
        let debug = envelope.map(|value| format!("{value:?}"));
        assert_eq!(
            debug,
            Ok(String::from(
                "SignatureEnvelope { algorithm: Ed25519, bytes: [64 bytes redacted] }"
            ))
        );
    }

    #[test]
    fn owned_signature_redacts_and_sanitizes() {
        let bytes = [7_u8; ED25519_SIGNATURE_BYTES];
        let mut owned = OwnedSignature::new(SignatureAlgorithm::Ed25519, &bytes);

        assert!(owned.as_ref().is_ok_and(|value| {
            value
                .as_envelope()
                .is_ok_and(|envelope| envelope.len() == ED25519_SIGNATURE_BYTES)
        }));
        assert_eq!(
            owned.as_ref().map(|value| format!("{value:?}")),
            Ok(String::from(
                "OwnedSignature { algorithm: Ed25519, bytes: [64 bytes redacted] }"
            ))
        );

        if let Ok(value) = owned.as_mut() {
            value.secure_sanitize();
        }

        assert_eq!(
            owned.map(|value| (value.algorithm(), value.len())),
            Ok((SignatureAlgorithm::Ed25519, 0))
        );
    }

    #[test]
    fn owned_signature_sanitizes_algorithm_sentinel() {
        let bytes = [7_u8; ML_DSA_SIGNATURE_BYTES_MIN];
        let mut owned = OwnedSignature::new(SignatureAlgorithm::MlDsa, &bytes);

        if let Ok(value) = owned.as_mut() {
            value.secure_sanitize();
        }

        assert_eq!(
            owned.map(|value| (value.algorithm(), value.len())),
            Ok((SignatureAlgorithm::Ed25519, 0))
        );
    }

    #[test]
    fn sanitized_owned_signature_has_no_envelope() {
        let bytes = [7_u8; ED25519_SIGNATURE_BYTES];
        let mut owned = OwnedSignature::new(SignatureAlgorithm::Ed25519, &bytes);

        if let Ok(value) = owned.as_mut() {
            value.secure_sanitize();
        }

        assert!(owned.is_ok_and(|value| value.as_envelope().is_err()));
    }

    #[test]
    fn hybrid_signature_envelope_splits_bound_components() {
        let raw = [9_u8; HYBRID_SIGNATURE_BYTES_MIN];
        let parsed = HybridSignatureEnvelope::parse(&raw);

        assert_eq!(
            parsed.map(|value| (value.classical_bytes().len(), value.pq_bytes().len())),
            Ok((ED25519_SIGNATURE_BYTES, ML_DSA_SIGNATURE_BYTES_MIN))
        );
    }

    #[test]
    fn hybrid_signature_envelope_rejects_unbound_component_lengths() {
        let raw = [9_u8; ED25519_SIGNATURE_BYTES + ML_DSA_SIGNATURE_BYTES_MIN + 1];

        assert_eq!(
            HybridSignatureEnvelope::parse(&raw),
            Err(SagnirError::InvalidValue)
        );
    }

    #[test]
    fn hybrid_signature_envelope_debug_redacts_components() {
        let raw = [9_u8; HYBRID_SIGNATURE_BYTES_MIN];
        let debug = HybridSignatureEnvelope::parse(&raw).map(|value| format!("{value:?}"));

        assert_eq!(
            debug,
            Ok(String::from(
                "HybridSignatureEnvelope { classical_bytes: [64 bytes redacted], pq_bytes: [2420 bytes redacted] }"
            ))
        );
    }
}
