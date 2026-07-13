# Sagnir 0.6.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.6.0 defines the object header format before durable object storage is
implemented.

This release adds a fixed-width object header with magic bytes, object type,
format version, body length, and flags fields. Header parsing fails closed for
wrong magic, unknown object types, unsupported format versions, oversized body
metadata, missing declared body bytes, unknown flags, and truncated buffers
before any object body allocation or indexing can occur.

The object crate is split into focused identity and header modules. A
`cargo-fuzz` object-header target is also admitted at
`fuzz/fuzz_targets/object_header_parse.rs`. The fuzz package stays outside the
stable Rust workspace so normal release gates do not require nightly tooling.

The first v0.6.0 pentest findings are closed in this line: constant-time byte
comparison now uses `subtle`, metadata equality is folded into fixed-size byte
comparisons, ML-DSA and hybrid signature envelopes enforce concrete sizes,
owned signature bytes clear through `sanitization`, object-header parsing
requires the declared body bytes to be present, the header length derives from
codec constants, and the object-header fuzz target is active.

The second v0.6.0 pentest findings are also closed: object-header body
availability now compares `tail.len()` against a checked `usize` body length,
the standalone fuzz workspace review rule is documented, NCSA license admission
names the libFuzzer dependency, and `OwnedSignature` documents its fixed stack
cost.

The third v0.6.0 pentest findings are closed as well: the fuzz workspace now
has an explicit `fuzz/deny.toml`, the root dependency policy no longer carries
the fuzz-only NCSA license entry, object-header flag admission uses a literal
zero-bit policy, and zero-length structured object bodies fail closed.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-object
scripts/checks.sh
scripts/release_0_6_gate.sh
cargo test -p sagnir-crypto
cargo check --manifest-path fuzz/Cargo.toml --bins
cargo deny --manifest-path fuzz/Cargo.toml --config fuzz/deny.toml check
```

`scripts/release_0_6_gate.sh` must fail until
`security/pentest/v0.6.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.6.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review object magic, type, version, body length, and flag parsing;
- review malformed header tests and body-length bounds;
- review unknown flag rejection and critical-extension policy;
- review duplicate-field tracker behavior for future variable header parsing;
- review the object-header fuzz target package;
- review `subtle` and `sanitization` dependency admission;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.6.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- Object header parsing rejects wrong magic before type, version, length, or
  flags are trusted.
- Object type and format version parsing fail closed on unknown values.
- Body length metadata is bounded and declared body bytes must be available
  before future parser allocation paths.
- No object header flags are admitted yet; any nonzero flag fails closed.
- Zero-length bodies are admitted only for blob objects in v0.6.0.
- A no-allocation field tracker rejects duplicate header fields for future
  variable header parsing.
- The object-header fuzz target is active in the separate `fuzz/` package.
- `zeroize` is not admitted; owned signature clearing uses `sanitization`.
