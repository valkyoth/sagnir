# Sagnir 0.10.0 Release Notes

Status: implementation stop

## Summary

Sagnir v0.10.0 gives each local store a persistent realm identity and bounded
configuration. `saga init` now creates `.saga/realm.toml` from the
operating-system random source and writes `.saga/config.toml` with `standard`
profile metadata, `lazy-cone` verification metadata, and a `512MiB` memory
budget.

The `no_std` store crate provides allocation-free canonical readers and writers
for all admitted profiles and verification modes. Memory, parallelism, entry,
and reference controls are checked before use. Existing valid v0.9.0 stores can
be completed by rerunning `saga init`; existing realm IDs are never regenerated.

The first pentest pass closed directory redirection through pre-existing
symlinked `.saga/` paths. Initialization now requires every store path it
creates to be a real directory. Init serialization now uses Rust's native
operating-system file locks, which release on process exit or crash across
supported platforms instead of relying on Linux `/proc` PID checks.

The second pentest pass closed the remaining Unix namespace race by anchoring
initialization to opened directory handles. Canonical root traversal, store
directory creation, file opens, cleanup, permission changes, renames, and sync
now use no-follow handle-relative operations. Restricted-root aliases are
canonicalized before admission, bounded metadata reads tolerate short reads
without accepting prefixes, and non-Unicode CLI arguments return a controlled
usage error instead of panicking.

Rust is updated to 1.97.0 and `sanitization` to 1.2.4. `getrandom` 0.4.3 is
admitted only at the CLI boundary to obtain cross-platform operating-system
entropy for realm IDs.
`rustix` 1.1.4 is admitted only on Unix targets for safe handle-relative
filesystem operations.
The checksum-pinned CI installer is updated to `cargo-deny` 0.20.2.

The Rust builder image is pinned as
`docker.io/library/rust:1.97.0-bookworm@sha256:7d0723df719e7f213b69dc7c8c595985c3f4b060cfbee4f7bc0e347a86fe3b6a`.

## Verification

Required local verification for this implementation stop:

```bash
cargo test -p sagnir-store
cargo test -p sagnir-cli
cargo check --manifest-path fuzz/Cargo.toml --bins
cargo deny --manifest-path fuzz/Cargo.toml --config fuzz/deny.toml check
cargo run -p sagnir-cli --bin saga -- init --dry-run
scripts/checks.sh
scripts/release_0_10_gate.sh
```

`scripts/release_0_10_gate.sh` must fail until
`security/pentest/v0.10.0.md` is completed with `Status: PASS`.

Tag stop:

```text
v0.10.0 implementation stop reached. Run pentest for this exact commit.
```

Pentest task:

- run all local gates for the exact commit;
- review realm ID entropy acquisition, nonzero validation, canonical encoding,
  and preservation across repeated initialization;
- review bounded realm/config reads and writes, owner-only permissions, sync
  ordering, atomic rename, stale temp cleanup, metadata symlink refusal, and
  root/nested store-directory symlink refusal;
- race the visible `.saga/` namespace after its handle is opened and confirm
  directory and file writes remain anchored to the original store;
- test lexical restricted-root aliases, repeated short metadata reads, and
  invalid non-Unicode operating-system arguments;
- review operating-system init-lock exclusivity, crash release, persistent
  diagnostic content, and lock-path symlink refusal;
- test malformed UTF-8, oversized metadata, unknown and duplicate fields,
  invalid profiles and modes, invalid units, arithmetic overflow, and budget
  boundaries;
- test v0.9.0 format-only store completion and partial metadata recovery;
- review the `getrandom` 0.4.3, `rustix` 1.1.4, and `sanitization` 1.2.4
  supply-chain changes;
- confirm verification modes are documented as metadata, not active proof
  execution in this release;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- update `security/pentest/v0.10.0.md` with `Status: PASS`, exact commit,
  tester, date, scope, and notes before tagging;
- wait for GitHub CI to go green before tagging;
- tag only after explicit maintainer instruction;
- use a pentest reviewer independent from the implementation author.

## Security Notes

- Realm IDs are public identity metadata, not encryption keys or credentials.
- Realm creation fails if operating-system entropy is unavailable; it does not
  fall back to timestamps, process IDs, or deterministic pseudorandom data.
- Realm IDs are 256-bit, nonzero, lowercase canonical values and reject wrong
  prefixes, lengths, uppercase hex, non-hex data, and all-zero values.
- Realm/config parsing is allocation-free and bounded before interpretation.
- Unknown tables, unknown fields, duplicate fields, invalid values, and missing
  required fields fail closed.
- Metadata paths must be regular files. Symlinks and other file types fail.
- Store directories must be real directories. Root and nested directory
  symlinks fail before writes can reach their targets.
- Unix store operations remain relative to retained no-follow directory
  handles, so namespace replacement cannot split locking and writes across
  different `.saga/` directories.
- Bounded metadata reads continue through short reads and probe for one byte
  beyond the admitted limit before parsing.
- Non-Unicode command-line arguments fail with usage exit code `2` and do not
  expose raw argument bytes.
- `.saga/init.lock` is a persistent diagnostic file protected by an
  operating-system lock. Lock ownership is not inferred from PID text, and the
  lock is released when the process handle closes. Existing lock content is not
  rewritten, and multiply linked Unix lock files are rejected.
- Metadata writes use owner-only temp files, file sync, atomic rename, and Unix
  parent-directory sync.
- The default profile metadata remains `standard`; relaxed profile metadata
  requires an explicit configuration change.
- The release metadata gate now keeps standalone fuzz path dependencies on the
  same Sagnir version as the main workspace.
- Profile, verification mode, and resource values are metadata only in
  v0.10.0. Live repository-scale verification remains planned for v0.14.0 and
  profile-to-policy enforcement remains planned for v0.36.0.
- v0.10.0 does not create source objects, worlds, changes, facts, keys, or
  policies.
- Existing malformed realm or config metadata blocks initialization. Recovery
  is backup-first and must preserve realm identity and configured posture; see
  `docs/local-store.md`.
