# Sagnir Architecture

Status: planning document

Sagnir is a local-first source-state system.

The architectural layers are:

- `saga`: CLI command.
- `sagnir`: main library crate.
- focused Sagnir crates: core, codec, object, store, worktree, change, world,
  fact, policy, crypto, proof, and sync.
- `.saga/`: local embedded store in a project worktree.
- Sagnir protocol: proof-carrying bundle and sync protocol.

## Local Store

The local store is not an external database. It is a durable embedded store
inside `.saga/`.

Durable rules:

- objects are immutable;
- world states are immutable;
- facts are append-only;
- operations are append-only;
- indexes are rebuildable;
- aliases are mutable pointers to immutable states.

## Trust Boundary

Every parser and verifier treats local bytes as untrusted. A copied repository,
sync bundle, object pack, WAL frame, or policy file may be malicious.

Verification happens before durable acceptance.
