# Sagnir Toolchain Policy

Status: policy

Sagnir currently pins Rust stable `1.96.0`.

## Update Rule

Before changing the toolchain:

1. Check the official Rust release announcements.
2. Read the release notes for compatibility and security changes.
3. Run `scripts/checks.sh`.
4. Update this document and release notes.

## Crate Rule

Before adding a third-party crate:

1. Check crates.io for the latest stable version.
2. Review license compatibility with EUPL-1.2.
3. Review maintenance and advisory status.
4. Add tests that cover behavior introduced by the crate.
5. Run `cargo deny check` and `cargo audit`.

The initial scaffold intentionally has no third-party Rust dependencies.
