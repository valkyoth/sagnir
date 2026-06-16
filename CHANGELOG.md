# Changelog

All notable Sagnir changes are recorded here.

## Unreleased

## 0.3.0

Status: implementation stop.

- Added a testable `saga` CLI router.
- Added golden-output tests for help, version, unknown-command, and
  extra-argument behavior.
- Added stable command-line usage errors with exit code `2`.
- Tightened the modularity gate so non-generated Rust files over 500 lines fail
  without exception.
- Added release-gate freshness checks for Rust, compatible Cargo updates, CI
  cargo tools, and the pinned GitHub checkout action.
- Addressed the first v0.3.0 pentest findings across timing-hardened equality,
  path byte validation, typed format versions, redacted debug output,
  dependency policy, terminal-safe CLI errors, checked bundle totals, and
  checksum-verified CI security tool installation.
- Added v0.3.0 release notes, pentest placeholder, and release gate.

## 0.2.0

Status: tagged release-gate baseline.

- Added the v0.2.0 release gate baseline.
- Added reusable release-note and pentest-report validators.
- Added v0.2.0 release notes and pentest placeholder.
- Addressed the first v0.2.0 pentest findings.

## 0.1.0

Status: tagged foundation release.

- Initialized the Sagnir Rust workspace.
- Added the `saga` CLI scaffold.
- Added security, modularity, toolchain, implementation, and version planning
  documents.
