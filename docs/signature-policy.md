# Sagnir Signature Policy

Status: planning document

Signature algorithms are admitted explicitly. Unknown algorithm identifiers must
fail closed before signature bytes are trusted.

Current envelope algorithm identifiers:

- Ed25519;
- ML-DSA;
- HybridClassicalPq.

Signature byte admission is algorithm-specific. Fixed-length algorithms must
use exact byte sizes at the envelope boundary. Variable-length or family-level
algorithms use explicit upper bounds for envelope parsing; those bounds are not
cryptographic verification.

Timing note: Sagnir admits `subtle` for constant-time byte comparison and
`sanitization` for owned signature-byte clearing before live verification code
depends on these envelope paths.

## Hybrid Binding Rule

`HybridClassicalPq` must not be implemented as an ambiguous byte
concatenation. The envelope format must bind the classical and post-quantum
components so stripping one component fails verification.

Required verification properties:

- the envelope identifies both component algorithms;
- both component signatures are present;
- the post-quantum component commits to the classical component bytes;
- the verifier fails closed if either component is missing;
- the verifier returns valid only when both component verifications pass;
- partial verification must never promote a hybrid signature to valid.

Initial binding model:

```text
sig_classical = Ed25519_sign(classical_key, message)
sig_pq = ML-DSA_sign(pq_key, message || sig_classical)
hybrid = sig_classical || sig_pq
```

Verification must check `sig_pq` over `message || sig_classical` and then check
`sig_classical` over `message`. Both checks must pass.
