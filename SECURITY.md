# Security Policy

Sagnir is security-sensitive source-state infrastructure. Treat dependency,
encoding, object identity, local-store recovery, proof, policy, sync, and
worktree materialization changes as high risk until tested.

## Routine Checks

Run these regularly and before releases:

```bash
scripts/checks.sh
scripts/security_tool_gate.sh
scripts/validate-pentest-pass.sh v0.1.0
cargo deny check
cargo audit
scripts/generate-sbom.sh
```

GitHub Actions run CI. GitHub CodeQL default setup must be enabled in the
repository security settings, and Sagnir intentionally does not maintain a
custom advanced CodeQL workflow. Keep only one active CodeQL configuration.

## Dependency Policy

The dependency policy lives in `deny.toml`. Unknown registries and git sources
are denied by default. License exceptions must be narrow, named, versioned, and
documented with the reason for acceptance.

Build scripts, procedural macros, `*-sys` crates, vendored native code, Cargo
aliases, CI workflow edits, and release script edits are executable supply-chain
changes. Review them before merging dependency updates.

Reviewed advisory exceptions are allowed only when there is no compatible
upgrade and the affected API is not reachable in Sagnir. Each exception must be
listed in the tool that reports it, with a removal condition.

The `fuzz/` package is a standalone workspace so `cargo-fuzz` and nightly
tooling stay outside the stable root workspace. Its dependency tree must be
reviewed during pentest handoff and checked manually with:

```bash
cargo deny --manifest-path fuzz/Cargo.toml check
```

## Release Supply-Chain Evidence

Stable releases must publish SBOM files generated from the tagged source tree.
Release notes must include source archive checksums, binary checksums, container
digests where applicable, and signed tag verification notes.

## Reporting

Do not publish exploitable security details before a fix is available. Open a
private security advisory or contact the maintainers directly once public
repository security channels are configured.
