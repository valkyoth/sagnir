# Sagnir Local Store

Status: planning document

A Sagnir project stores local state under `.saga/`.

Planned layout:

```text
.saga/
  FORMAT
  config.toml
  realm.toml
  objects/
  wal/
  indexes/
  worlds/
  changes/
  facts/
  ops/
  keys/
  policies/
  projections/
  tmp/
  locks/
```

Initialization:

- `saga init --dry-run` prints the planned layout and writes nothing;
- `saga init` creates `.saga/` and required subdirectories;
- `.saga/FORMAT` contains `sagnir-format = 1`;
- `.saga/realm.toml` contains a cryptographically random, nonzero, 256-bit realm
  ID in canonical lowercase text form;
- `.saga/config.toml` starts with the strict `standard` profile, `lazy-cone`
  verification metadata, and a `512MiB` memory budget;
- configuration admits only the explicit `standard`, `solo`, `team`, and
  `regulated` profiles;
- verification metadata admits only `bounded-batch`, `lazy-cone`, and
  `full-world`, with optional bounded memory, parallelism, entry, and reference
  controls;
- memory budgets range from `16MiB` through `1TiB`, and explicit parallelism
  ranges from 1 through 256 workers;
- `.saga/FORMAT` is written through `.saga/FORMAT.tmp` and rename;
- realm and config files are written through owner-only temporary files,
  synchronized, renamed atomically, and followed by a Unix directory sync;
- `.saga/` directories are owner-only on Unix systems;
- `.saga/FORMAT`, realm metadata, and config metadata are read with bounded
  fixed-size buffers that continue across short reads and reject trailing
  bytes beyond the admitted limit;
- every existing store directory must be a real directory; symlinks and other
  file types fail closed before Sagnir writes through them;
- realm, config, and init-lock paths must be regular files; symlinks and other
  file types fail closed;
- Unix initialization canonicalizes the root, opens each root component with
  no-follow semantics, retains the opened `.saga/` directory handle, and
  resolves directory creation, file opens, cleanup, permission changes,
  renames, and directory sync relative to that handle;
- replacing the visible `.saga/` path after it is opened cannot redirect the
  active initialization transaction to the replacement namespace;
- `.saga/init.lock` uses a non-blocking operating-system file lock to serialize
  concurrent initialization on supported platforms;
- a newly created lock file records diagnostic PID metadata, but existing
  content is never rewritten and does not determine ownership; closing the
  process file handle releases the lock after normal exit or a crash;
- stale `.saga/FORMAT.tmp` files are removed so interrupted init can be
  retried;
- running init again against a valid layout is allowed;
- running init against a valid v0.9.0 format-only layout creates the missing
  realm and config files without changing the format marker;
- unexpected existing `.saga/FORMAT` content fails closed.

Canonical default config:

```toml
format = 1
profile = "standard"

[verification]
mode = "lazy-cone"
memory_budget = "512MiB"
```

The v0.10.0 metadata parser accepts a deliberately narrow canonical TOML
subset: root `format` and `profile` fields plus one `[verification]` table.
Unknown tables, unknown fields, duplicate fields, escaped strings, invalid
units, oversized files, and out-of-range values fail closed. This keeps the
`no_std` parser allocation-free and makes the persisted representation
deterministic.

The configured profile and verification mode are metadata only in v0.10.0.
Live chunked, lazy-cone, and full-world execution is introduced by the later
local fsck release. Profile-to-policy enforcement is introduced by the later
policy release. This release does not claim either path is active.

## Metadata Recovery

`saga init` validates existing realm and config metadata and fails closed on
corruption. It does not silently replace either file.

Until a dedicated repair command is implemented:

1. Copy the complete worktree and `.saga/` directory before changing anything.
2. Restore `.saga/realm.toml` from a trusted backup. Do not delete it to make
   `saga init` generate a different realm identity.
3. Restore `.saga/config.toml` from a trusted policy source or correct it to the
   canonical admitted form. Deleting it recreates default `standard` metadata
   and is not an accepted recovery method for team or regulated realms.
4. Run the current local verification gate before trusting the repaired store.

The planned local fsck release must provide a non-mutating metadata repair plan
before any automated repair command is admitted. Repair must never change a
realm ID or weaken profile metadata without explicit operator confirmation and
an append-only operation record.

Recovery rule:

- committed WAL transactions are replayed;
- incomplete transactions are ignored;
- every WAL frame header carries a CRC-32c checksum over the frame kind,
  transaction ID, and payload for crash-corruption detection before replay;
- checksum failure aborts recovery instead of silently skipping the frame;
- CRC-32c is not adversarial tamper detection; encrypted realms must add a
  keyed MAC over each frame when vault keys are available;
- missing indexes are rebuilt;
- aliases must point to existing immutable states.
