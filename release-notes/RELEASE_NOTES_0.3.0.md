# Sagnir 0.3.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.3.0 establishes the first testable `saga` command router.

This release keeps the CLI deliberately small. It stabilizes `saga help`,
`saga version`, unknown-command handling, and command-line usage errors before
stateful realm commands are added.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-cli
cargo run -p sagnir-cli --bin saga -- version
scripts/checks.sh
scripts/release_0_3_gate.sh
```

`scripts/release_0_3_gate.sh` must fail until
`security/pentest/v0.3.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.3.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run the local gates for the exact commit;
- review the command router and user-facing output;
- confirm usage errors return exit code `2`;
- confirm stdout and stderr separation;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.3.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging.

## Security Notes

- CLI dispatch is now tested without process execution.
- Golden-output tests pin help, version, unknown-command, and extra-argument
  behavior.
- Unknown commands and unexpected extra arguments fail closed with exit code
  `2`.
- Stateful source-state commands remain out of scope for this release.
- No hosted service, external database, network protocol, or durable realm
  storage is introduced in this release.
