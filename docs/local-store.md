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
- missing indexes are rebuilt;
- aliases must point to existing immutable states.
