# Sagnir 0.9.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.9.0 adds the first stateful local command: `saga init`.

This release creates the initial `.saga/` store layout without writing
source-state objects. The CLI supports a dry-run plan, creates the required
directories, writes `.saga/FORMAT`, tolerates repeated initialization, and
cleans a stale `.saga/FORMAT.tmp` file left by interrupted initialization.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-store
cargo test -p sagnir-cli
cargo run -p sagnir-cli --bin saga -- init --dry-run
scripts/checks.sh
scripts/release_0_9_gate.sh
```

`scripts/release_0_9_gate.sh` must fail until
`security/pentest/v0.9.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.9.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review `saga init --dry-run` output and no-write behavior;
- review `.saga/` directory creation;
- review `.saga/FORMAT` content and write ordering;
- review idempotent init behavior;
- review stale `.saga/FORMAT.tmp` cleanup;
- review refusal behavior when an existing `.saga/FORMAT` is unexpected;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.9.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- `saga init --dry-run` reports the planned layout without writing files.
- The trusted store layout remains defined in `sagnir-store`, which stays
  `no_std`.
- `.saga/FORMAT` is written through `.saga/FORMAT.tmp` and rename.
- Stale `.saga/FORMAT.tmp` files are removed during init so interrupted init can
  be retried.
- Existing `.saga/FORMAT` content must match the admitted format marker.
- v0.9.0 does not create source objects, worlds, changes, facts, keys, or
  policies.
