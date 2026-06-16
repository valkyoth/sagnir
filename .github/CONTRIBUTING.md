# Contributing To Sagnir

Sagnir is security-sensitive source-state infrastructure. Contributions are
welcome when they keep the project small, clear, tested, and honest about what
is stable.

## License

Sagnir is licensed under the European Union Public Licence 1.2. By
contributing, you agree that your contribution is provided under the same
license.

## Development Setup

Use the pinned Rust toolchain from `rust-toolchain.toml`.

```bash
cargo check --workspace
cargo test --workspace
```

## Checks

Before opening a pull request, run:

```bash
scripts/checks.sh
```

## Security-Sensitive Changes

Treat these areas as high risk:

- canonical object encoding and decoding;
- object identity and hash-domain separation;
- local store recovery and alias updates;
- proof, signature, and crypto-agility metadata;
- policy and promotion decisions;
- sync bundle parsing and verification;
- worktree materialization;
- dependency updates.

Do not post exploitable security details in public issues. Follow
[SECURITY.md](../SECURITY.md).

## Dependency Policy

Sagnir uses `deny.toml`, `cargo-deny`, and `cargo-audit`.

When adding or updating crates:

- use crates.io releases unless there is a documented reason not to;
- avoid git dependencies;
- check maintenance status and license;
- keep `Cargo.lock` updated;
- run `cargo deny check` and `cargo audit`;
- add tests for behavior introduced by the crate.

## Design Guidelines

- Prefer focused crates over large files.
- Keep trusted library crates `no_std` where practical.
- Keep parsing, validation, policy, state, I/O, and tests separate.
- Do not add hosted-service assumptions to local Sagnir behavior.
- Document stable, experimental, and future behavior honestly.
