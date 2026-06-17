# Sagnir 0.4.0 Release Notes

Status: released

## Summary

Sagnir v0.4.0 makes the core identifier and bounded-name layer reliable before
canonical object bodies and durable storage grow on top of it.

This release splits `sagnir-core` into focused modules for errors, IDs, and
names. It adds concrete typed ID wrappers for core source-state identities,
keeps `TypedId` available as the shared wire-shaped representation, and makes
format-version admission explicit through `FormatVersion::try_new`.

Bounded names remain allocation-free and reject empty input, oversize input,
path traversal segments, `.saga` control-path aliases, slash misuse in segment
checks, Windows path aliases, non-ASCII bytes, whitespace, and terminal control
bytes before object or store code can see them.

The first v0.4.0 pentest findings are closed in this line: timing-hardened
equality call sites force byte comparisons before combining discriminants,
length-prefixed decoding now has a caller-bounded read API, Windows path aliases
are rejected by both core names and worktree classification, obligation
emptiness has an explicit API, crypto dependency admission is release-gated,
and cryptographic signature envelopes no longer implement `Copy`.

The second v0.4.0 pentest findings are also closed: the comparison helper again
forces the accumulated diff directly through `black_box`, the secret/key
material `Copy` guard covers more vault-phase naming patterns, and
`read_len_prefixed` has simpler in-bounds tail handling.

## Verification

Required local verification for this release:

```bash
cargo test -p sagnir-core -p sagnir-codec -p sagnir-crypto -p sagnir-object -p sagnir-worktree -p sagnir-policy
scripts/checks.sh
scripts/release_0_4_gate.sh
```

`scripts/release_0_4_gate.sh` passes after
`security/pentest/v0.4.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.4.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review typed ID wrappers and wrong-kind rejection;
- review format-version admission;
- review bounded-name and path/name byte admission tests;
- confirm core validation performs no heap allocation;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.4.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- Concrete ID wrappers preserve the intended `IdKind` for realm, world, change,
  revision, state, fact, object, operation, and bundle IDs.
- Wrong-kind conversion from a generic `TypedId` fails closed with
  `SagnirError::InvalidValue`.
- `TypedId` debug output remains redacted, and wrapper debug output delegates to
  the redacted inner representation.
- `FormatVersion::try_new` accepts only the current format version.
- Bounded-name validation rejects invalid and oversize names before object or
  store layers receive them.
- Windows reserved device names and trailing-dot aliases are rejected before
  they can become tracked names or worktree candidates.
- `read_len_prefixed` requires a caller-provided maximum before accepting
  untrusted length-prefixed payloads.
- `ObligationSet::is_empty` separates emptiness checks from bit membership.
- Release gates reject known crypto provider crates unless `subtle` and
  `sanitization` are admitted.
- Second pentest review findings are closed for diff-forcing comparison,
  secret/key-material `Copy` guard coverage, and codec control-flow clarity.
- Core validation remains `no_std` and allocation-free.
