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
- `.saga/FORMAT` is written through `.saga/FORMAT.tmp` and rename;
- `.saga/` directories are owner-only on Unix systems;
- `.saga/FORMAT` is read with a bounded fixed-size buffer;
- `.saga/init.lock` serializes concurrent initialization attempts;
- malformed init locks and Linux locks whose owner process is gone are treated
  as stale and recovered during initialization;
- stale `.saga/FORMAT.tmp` files are removed so interrupted init can be
  retried;
- running init again against a valid layout is allowed;
- unexpected existing `.saga/FORMAT` content fails closed.

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
