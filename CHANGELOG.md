# Changelog

All notable Sagnir changes are recorded here.

## Unreleased

## 0.8.0

Status: implementation stop.

- Added a fixed-capacity, `no_std` in-memory object graph table.
- Added typed object references that bind the expected target object kind.
- Added missing-reference detection with the exact unresolved reference in the
  verification report.
- Added an acyclic graph policy for pre-persistence object relationships.
- Added graph traversal tests proving reachable and unreachable object paths.
- Hardened graph traversal and verification to use bounded iterative worklists
  instead of recursion.
- Added public object graph capacity constants for the admitted v0.8.0 budget.
- Added object graph fuzz targets for verification and path traversal.
- Bound WAL CRC-32C metadata to frame kind and transaction ID in addition to
  payload bytes while documenting that CRC is not adversarial authentication.
- Documented that hybrid signature parsing is length admission only and added a
  compile-time stack-budget guard for owned signatures.
- Added ObjectId keyed-hasher policy guidance for attacker-influenced maps.
- Addressed the second v0.8.0 pentest findings by reporting defensive graph
  invariant failures as invalid entries instead of false cycles.
- Split graph tests from production code to preserve the 500-line modularity
  policy.
- Added v0.8.0 release notes, pentest placeholder, and release gate.

## 0.7.0

Status: tagged domain-separated object identity release.

- Added canonical object ID display and parse support with the
  `sagnir-object-v1:<type>:<algorithm>:<digest>` format.
- Added fail-closed object type and hash algorithm name parsing.
- Added digest slice admission checks for algorithm-specific digest lengths.
- Added object identity collision-domain tests across all admitted object
  kinds.
- Kept object ID equality on the admitted `subtle`-backed timing-safe byte
  comparison path.
- Added the hash migration plan for future algorithm admission.
- Added SHA3-256 hash algorithm admission metadata for sensitive deployment
  profiles.
- Hardened proof report construction so `Verified` requires an opaque
  verification token.
- Tightened world, change, and state-root references to use typed ID wrappers.
- Split parsed object headers into named `header`, `body`, and `rest` fields.
- Preserved `subtle::Choice` through composed timing-sensitive comparisons.
- Added redacted object ID display for sealed private contexts.
- Added hybrid signature component parsing and sanitized signature envelope
  rejection.
- Added redacted hybrid signature debug output and removed implicit `Copy` from
  hybrid signature envelopes.
- Added worktree path proof types for future symlink-resolved filesystem I/O.
- Added WAL frame CRC-32c integrity metadata.
- Allowed empty-payload WAL structural frames while still verifying their
  CRC-32c checksum.
- Added parser fuzz targets for object IDs, bounded names, worktree paths, and
  codec byte strings.
- Added forward-compatible protocol enum annotations and security tooling
  extraction hardening.
- Made security tool archive extraction portable by validating archive paths
  before extraction instead of relying on GNU-specific tar flags.
- Split object identity tests into a focused test module to preserve the
  500-line implementation file policy.
- Added v0.7.0 release notes, pentest placeholder, and release gate.

## 0.6.0

Status: tagged object header format release.

- Split `sagnir-object` into focused identity and header modules.
- Added fixed-width object header parsing and writing with magic, object type,
  format version, body length, and flags fields.
- Added fail-closed object type, format version, body length, and flags
  admission for object headers.
- Tightened object-header parsing so declared body bytes must be available at
  the parser boundary.
- Added a duplicate-field tracker for future variable header parsing.
- Admitted `subtle` for constant-time byte comparison and `sanitization` for
  owned signature-byte clearing without admitting `zeroize`.
- Tightened ML-DSA and hybrid signature-envelope admission to concrete
  algorithm sizes.
- Added an active object-header fuzz target package.
- Addressed the first v0.6.0 pentest findings across constant-time equality,
  signature bounds, parser body availability, sanitization policy, fuzz target
  activation, and header-length maintenance.
- Addressed the second v0.6.0 pentest findings across parser length comparison,
  fuzz workspace review documentation, NCSA license admission notes, and
  owned-signature stack-cost documentation.
- Addressed the third v0.6.0 pentest findings across explicit fuzz dependency
  policy, root license-policy scope, object-header flag naming, and
  zero-length structured body rejection.
- Added v0.6.0 release notes, pentest placeholder, and release gate.

## 0.5.0

Status: release-ready canonical scalar encoding release.

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
