# Sagnir 0.4.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.4.0 makes the core identifier and bounded-name layer reliable before
canonical object bodies and durable storage grow on top of it.

This release splits `sagnir-core` into focused modules for errors, IDs, and
names. It adds concrete typed ID wrappers for core source-state identities,
keeps `TypedId` available as the shared wire-shaped representation, and makes
format-version admission explicit through `FormatVersion::try_new`.

Bounded names remain allocation-free and reject empty input, oversize input,
path traversal segments, `.saga` control-path aliases, slash misuse in segment
checks, non-ASCII bytes, whitespace, and terminal control bytes before object
or store code can see them.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-core
scripts/checks.sh
scripts/release_0_4_gate.sh
```

`scripts/release_0_4_gate.sh` must fail until
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
- Core validation remains `no_std` and allocation-free.
