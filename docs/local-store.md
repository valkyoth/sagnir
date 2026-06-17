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
