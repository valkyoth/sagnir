# Sagnir 0.6.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.6.0 defines the object header format before durable object storage is
implemented.

This release adds a fixed-width object header with magic bytes, object type,
format version, body length, and flags fields. Header parsing fails closed for
wrong magic, unknown object types, unsupported format versions, oversized body
metadata, unknown flags, and truncated buffers before any object body
allocation or indexing can occur.

The object crate is split into focused identity and header modules. A
dependency-free fuzz target scaffold is also reserved at
`fuzz/fuzz_targets/object_header_parse.rs` so object-header fuzzing has a stable
location when the fuzz harness is admitted.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-object
scripts/checks.sh
scripts/release_0_6_gate.sh
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
- review the object-header fuzz target scaffold location;
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
- Body length metadata is bounded before future parser allocation paths.
- No object header flags are admitted yet; any nonzero flag fails closed.
- A no-allocation field tracker rejects duplicate header fields for future
  variable header parsing.
- The object-header fuzz target location is stable but the fuzz harness is not
  yet admitted into release gates.
