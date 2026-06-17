# Sagnir Architecture

Status: planning document

Sagnir is a local-first source-state system.

The architectural layers are:

- `saga`: CLI command.
- `sagnir`: main library crate.
- focused Sagnir crates: core, codec, object, store, worktree, change, world,
  event, fact, memory, causality, query, explain, context, audit, policy,
  crypto, proof, and sync.
- planned Sagnir causal memory layer: events, fact compiler, causal indexes,
  explanation objects, context packs, and evidence-first queries.
- planned Sagnir vault layer: encrypted realms, lock/unlock materialization,
  recipients, compartments, and encrypted bundles.
- `.saga/`: local embedded store in a project worktree.
- Sagnir protocol: proof-carrying bundle and sync protocol.

## Local Store

The local store is not an external database. It is a durable embedded store
inside `.saga/`.

Durable rules:

- objects are immutable;
- world states are immutable;
- facts are append-only;
- admitted events are append-only;
- operations are append-only;
- indexes are rebuildable;
- aliases are mutable pointers to immutable states.

## Trust Boundary

Every parser and verifier treats local bytes as untrusted. A copied repository,
sync bundle, object pack, WAL frame, or policy file may be malicious.

Verification happens before durable acceptance.

Worktree source-state I/O must not accept raw path strings. Lexical paths pass
through `WorktreePath`, and filesystem materialization must require a
`VerifiedWorktreePath` carrying symlink-boundary proof.

Encrypted realms add another trust boundary: while locked, Sagnir storage is
encrypted; while unlocked, plaintext may exist in the worktree and external
tool caches. Sagnir must make that distinction visible.
