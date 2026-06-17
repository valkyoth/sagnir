# Changelog

All notable Sagnir changes are recorded here.

## Unreleased

## 0.5.0

Status: implementation stop.

- Split `sagnir-codec` into module wiring and canonical scalar encoding.
- Added fixed-width little-endian `u16`, `u32`, and `u64` readers and writers.
- Added canonical byte-string and bounded list-length helpers.
- Preserved the existing length-prefixed byte API as byte-string aliases.
- Expanded malformed scalar tests for short buffers, truncated payloads, and
  caller-bound violations.
- Addressed the first v0.5.0 pentest findings across dotfile path rejection,
  signature bounds, credential scanning, scanner bypass policy, bundle count
  arithmetic, controlled error display, and hybrid signature composition policy.
- Addressed the second v0.5.0 pentest finding by enforcing exact 64-byte
  Ed25519 signature envelope admission.
- Added v0.5.0 release notes, pentest placeholder, and release gate.

## 0.4.0

Status: tagged core IDs and bounds release.

- Split `sagnir-core` into focused error, ID, and name modules.
- Added concrete typed ID wrappers for realm, world, change, revision, state,
  fact, object, operation, and bundle IDs.
- Added explicit current-format admission through `FormatVersion::try_new`.
- Expanded allocation-free bounded-name tests for oversize names, invalid
  bytes, slash handling, and control-path aliases.
- Addressed the first v0.4.0 pentest findings across timing-hardened equality,
  bounded length-prefix decoding, Windows path aliases, obligation emptiness,
  crypto dependency admission gates, multiple-version dependency bans, and
  cryptographic envelope copying.
- Addressed the second v0.4.0 pentest findings across diff-forcing comparison,
  secret/key-material `Copy` guard coverage, and codec control-flow clarity.
- Added v0.4.0 release notes, pentest placeholder, and release gate.

## 0.3.0

Status: tagged CLI router release.

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
- Recorded the second v0.3.0 pentest review with no new introduced issues.
- Fixed CI freshness validation to read crates.io metadata directly instead of
  parsing `cargo info` output.
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
