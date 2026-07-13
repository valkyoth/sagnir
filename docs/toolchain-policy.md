# Sagnir Toolchain Policy

Status: policy

Sagnir currently pins Rust stable `1.97.0`.

## Update Rule

The release gate runs `scripts/check_latest_crates.sh`. That script checks the
latest stable Rust manifest, compatible Cargo updates, pinned CI cargo tools,
and the pinned `actions/checkout` release.

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

Current third-party dependencies are kept narrow. `getrandom` is admitted only
at the CLI filesystem boundary to obtain cross-platform operating-system
entropy for realm IDs. `rustix` is admitted only on Unix targets so local-store
initialization can use safe handle-relative filesystem operations. `subtle`
provides timing-hardened equality and `sanitization` provides the project's
`no_std` secret-clearing policy.
