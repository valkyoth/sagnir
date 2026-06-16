# Sagnir 0.3.0 Release Notes

Status: released

## Summary

Sagnir v0.3.0 establishes the first testable `saga` command router.

This release keeps the CLI deliberately small. It stabilizes `saga help`,
`saga version`, unknown-command handling, and command-line usage errors before
stateful realm commands are added.

The release gate also now enforces the strict 500-line Rust source limit and
checks toolchain/tooling freshness before pentest handoff.

Local implementation, finding closure, and pentest review are complete for the
reviewed commit. Tagging waits for GitHub to go green and for explicit
maintainer instruction.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-cli
cargo run -p sagnir-cli --bin saga -- version
scripts/check_latest_crates.sh
scripts/checks.sh
scripts/release_0_3_gate.sh
```

`scripts/release_0_3_gate.sh` must pass before tagging.

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
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction.

## Security Notes

- CLI dispatch is now tested without process execution.
- Golden-output tests pin help, version, unknown-command, and extra-argument
  behavior.
- Unknown commands and unexpected extra arguments fail closed with exit code
  `2`.
- Release checks fail if Rust pins, compatible Cargo dependencies, CI cargo
  tools, or pinned GitHub checkout tooling are stale.
- Release checks fail if a non-generated Rust source file exceeds 500 lines.
- First v0.3.0 pentest findings closed before tag: timing-hardened equality no
  longer short-circuits on metadata, worktree paths reject unsafe bytes, state
  roots use typed format versions, future-private IDs redact debug bytes,
  advisory and wildcard dependency policy is stricter, CLI usage errors
  sanitize terminal control characters, bundle item totals use checked
  arithmetic, and CI security tools install from checksum-verified crate
  archives.
- Second v0.3.0 pentest review confirmed the first finding closure and found no
  new introduced issues.
- GitHub freshness validation now reads crates.io metadata directly instead of
  parsing `cargo info` output, so CI and local release gates use the same
  version source.
- Stateful source-state commands remain out of scope for this release.
- No hosted service, external database, network protocol, or durable realm
  storage is introduced in this release.
