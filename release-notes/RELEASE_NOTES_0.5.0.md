# Sagnir 0.5.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.5.0 establishes canonical scalar encoding before object headers and
object bodies start depending on serialized bytes.

This release makes `sagnir-codec` a focused module with fixed-width
little-endian readers and writers for `u16`, `u32`, and `u64`. It also adds
canonical byte-string helpers and bounded list-length helpers so future object,
bundle, and sync parsers have one fail-closed scalar path instead of ad hoc
length handling.

The existing length-prefixed byte helpers remain available as aliases for the
canonical byte-string API.

The first v0.5.0 pentest findings are closed in this line: tracked names reject
leading-dot path segments, signature envelopes use algorithm-specific maximums,
credential scanning covers more token and key patterns, scanner bypass markers
are restricted to documentation and reviewed fixtures, bundle count arithmetic
has a compile-time invariant, `SagnirError` has controlled display messages,
and hybrid signature composition has a binding policy before implementation.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-codec
scripts/checks.sh
scripts/release_0_5_gate.sh
```

`scripts/release_0_5_gate.sh` must fail until
`security/pentest/v0.5.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.5.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review fixed-width integer byte order and fail-closed short-buffer behavior;
- review byte-string length handling and caller-provided payload bounds;
- review list-length parsing and caller-provided item bounds;
- confirm scalar encoding performs no heap allocation;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.5.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- Scalar readers return `SagnirError::BufferTooSmall` for short fixed-width
  buffers.
- Byte-string decoding validates the declared length against a caller-provided
  maximum before returning a payload slice.
- List-length decoding validates the declared item count against a
  caller-provided maximum before later parser code can allocate or iterate.
- Scalar writers fail closed when the caller-provided output buffer is too
  small.
- Non-control leading-dot path segments are rejected before worktree
  materialization can treat them as tracked candidates.
- Signature envelope bounds are algorithm-specific and account for ML-DSA-87
  and hybrid classical plus post-quantum envelopes.
- Credential scanning rejects broader token, key, PEM private key, and
  JWT-shaped literals.
- Hybrid signature implementation is blocked on a documented composition rule
  that requires both classical and post-quantum components to verify.
- Canonical scalar encoding remains `no_std` and allocation-free.
