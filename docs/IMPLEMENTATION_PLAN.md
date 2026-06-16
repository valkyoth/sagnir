# Sagnir Implementation Plan

Status: planning document

Project name: `Sagnir`

CLI name: `saga`

1.0 target: a serious production-ready local-first CLI for proof-carrying
source-state work.

Sagnir is an open source source-state engine, object format, proof model, local
store, and protocol. The implementation must stay useful without a hosted
service, without a central server, and without an external database.

## Core Position

Sagnir tracks source state as worlds, changes, sealed revisions, facts, proofs,
policies, local operations, and sync bundles.

The canonical layer must be boring, durable, testable, and auditable. Advanced
behavior grows from world overlays, deterministic promotion preflight,
proof-carrying bundles, native encrypted realms, causal impact traversal,
cryptographic agility, capability-scoped automation metadata, and strict local
verification.

## Non-Negotiable Engineering Rules

- Rust stable `1.96.0`, edition 2024, workspace resolver `3`.
- Latest stable Rust, crate versions, CI actions, and tooling are re-checked
  before dependency, toolchain, or release changes.
- Core library crates use `#![no_std]` where practical.
- External crates are exceptional: discuss, verify, document, and test before
  use.
- No unsafe Rust in trusted crates. If unsafe ever becomes unavoidable, isolate
  it in a dedicated boundary crate after policy admission.
- Main crate `sagnir` orchestrates focused crates.
- Normal `.rs` files should stay under 300 lines and must stay under 500 lines
  unless documented.
- Every subsystem must be testable without a hosted service.
- Security, provenance, and policy checks are part of the engine, not user
  interface decoration.
- Documentation and release notes are release artifacts, not afterthoughts.

## Workspace Shape

- `sagnir-core`: IDs, versions, bounded names, shared limits, and errors.
- `sagnir-codec`: canonical binary encoding primitives.
- `sagnir-object`: object types, object identity, state-root metadata.
- `sagnir-store`: `.saga/` layout, WAL metadata, packs, aliases, and recovery.
- `sagnir-worktree`: path scanning, ignore rules, and materialization safety.
- `sagnir-change`: changes, sealed revisions, amend chains, and touched scope.
- `sagnir-world`: worlds, promotion preflight, rollback preflight, conflicts.
- `sagnir-fact`: local fact envelopes, evidence references, confidence, causes.
- `sagnir-policy`: local policy decisions, proof requirements, obligations.
- `sagnir-crypto`: crypto-agile algorithm metadata and signature envelopes.
- `sagnir-proof`: verification reports for objects, changes, worlds, releases.
- `sagnir-sync`: bundles and protocol metadata.
- `sagnir-vault`: planned encrypted realm, lock/unlock, recipient, compartment,
  and key-epoch logic.
- `sagnir`: main library crate using the focused crates.
- `sagnir-cli`: package that builds the `saga` binary.
- `sagad`: optional daemon scaffold.
- `xtask`: repeatable local automation.

## Phase 1: Format And Local Store Foundation

Build the canonical object identity model before user-facing behavior grows.

Required work:

- domain-separated object IDs;
- algorithm-agile object identity metadata;
- canonical binary encoding rules;
- `.saga/` layout;
- append-only operation log model;
- WAL transaction frame model;
- immutable object placement rules;
- rebuildable index policy;
- startup recovery state machine;
- `saga init` and `saga fsck`.

Truth rule:

- immutable objects and committed WAL frames are truth;
- indexes are rebuildable acceleration;
- aliases are mutable pointers to immutable signed states.

## Phase 2: Worktree, Worlds, And Changes

Build local daily work.

Required work:

- portable worktree scanner for Linux, Windows, BSD, MacOS, Android, and iOS
  constraints;
- `.saga/` control-path exclusion;
- deterministic tree builder;
- text diff baseline;
- `saga status`;
- `saga diff`;
- `saga world open`;
- `saga world switch`;
- `saga change begin`;
- `saga seal`;
- `saga change amend`;
- `saga log`.

Design rule:

- a change is logical intent;
- a sealed revision is immutable evidence for one exact change version;
- a world is policy-bound state, not just a mutable branch pointer.

## Phase 3: Proofs, Policy, And Promotion

Build verification before network sync.

Required work:

- object graph verification;
- policy epoch metadata;
- proof report shape;
- signature envelope validation;
- local policy file;
- `saga prove`;
- `saga promote`;
- deterministic conflict categories;
- promotion denial messages with missing proof requirements;
- rollback preflight.

Promotion rule:

- Sagnir promotes proven state between worlds.
- Promotion failure must be deterministic and explainable.
- Promotion must never delete immutable history.

## Phase 4: Operation Ledger And Undo

Build safe recovery for user mistakes.

Required work:

- operation objects;
- operation log replay;
- inverse operation planning;
- alias restore;
- materialized state restore;
- `saga undo`;
- `saga op log`;
- crash tests for interrupted operations.

Undo rule:

- undo creates a new operation;
- undo does not erase immutable objects, facts, sealed revisions, or committed
  WAL frames.

## Phase 5: Facts, Evidence, And Local Impact

Build local fact support that can answer provenance questions without external
infrastructure.

Required work:

- local fact envelope;
- fact log;
- evidence references;
- confidence score;
- causal links;
- `saga test record`;
- `saga review approve`;
- `saga why`;
- `saga impact`;
- taint and quarantine fact types.

Impact rule:

- if a key, dependency, worker, model, fact, or sealed revision is tainted,
  Sagnir must be able to walk local causal links and identify downstream state
  that needs review or quarantine.

## Phase 6: Crypto-Agile Control Plane

Build cryptographic metadata without hard-coding one permanent algorithm.

Required work:

- algorithm registry;
- signature envelopes;
- bounded signature set;
- key identifiers;
- key epoch metadata;
- crypto epoch metadata;
- hybrid classical and post-quantum readiness metadata;
- offline verification;
- `saga actor init`;
- `saga sign`;
- `saga verify`.

Crypto rule:

- object formats must carry algorithm identifiers;
- unknown algorithms are rejected unless local policy explicitly admits them;
- migration is a signed epoch transition, not in-place mutation.

## Phase 7: Native Encrypted Realms

Build encryption as a Sagnir primitive, not a bolt-on file filter.

The user-facing model is:

```text
saga encrypt project
saga unlock
saga lock
saga vault status
```

The technical model is:

- encrypted `.saga/` objects, facts, worlds, indexes, operation logs, and
  bundles;
- lock/unlock materialization rather than permanent decrypt/re-encrypt toggles;
- recipient-based key wrapping;
- crypto epochs;
- optional worktree wipe on lock;
- explicit plaintext-leak warnings;
- future compartment encryption for path, world, and projection boundaries.

Required work:

- vault metadata object;
- encrypted object envelope;
- recipient slot metadata;
- key hierarchy metadata;
- passphrase unlock baseline;
- local key storage abstraction;
- `saga encrypt project`;
- `saga unlock`;
- `saga lock`;
- `saga vault status`;
- `saga vault recipient list`;
- `saga vault rekey`;
- `saga vault scan-leaks` scaffold.

Encryption rule:

- Sagnir must never claim perfect secure deletion.
- Plaintext may exist while unlocked in the worktree, editor caches, build
  outputs, OS indexes, shell history, swap, backups, and tooling caches.
- The UI must distinguish encrypted realm storage from plaintext worktree
  materialization.

Privacy rule:

- public mode may use plaintext content hashes for open verification;
- sealed private mode uses private keyed object IDs and randomized ciphertext;
- path names, world names, change titles, author identity, facts, symbol names,
  and AI context packs are protected metadata in serious encrypted mode;
- sync-visible operational metadata is minimized and documented.

Post-quantum readiness rule:

- Sagnir says post-quantum-ready, quantum-resistant, or hybrid classical plus
  post-quantum; it does not claim a permanent quantum-proof key.
- Hybrid recipient wrapping and post-quantum signature admission happen through
  crypto epochs and reviewed algorithm registries.
- ML-KEM, ML-DSA, and SLH-DSA are current standards to track, but Sagnir must
  check official standards and implementation maturity before admitting a
  provider.

## Phase 8: Encrypted Bundles And Sync Modes

Extend bundles and sync so encrypted realms can use trusted, blind, and
split-trust remotes.

Required work:

- encrypted bundle manifest;
- encrypted pack envelope;
- recipient-targeted bundle creation;
- bundle verification before decrypt/import;
- blind remote storage mode;
- split-trust metadata mode;
- encrypted sync result facts;
- policy for visible versus encrypted bundle metadata.

Sync modes:

- trusted remote: remote can decrypt only through admitted key policy;
- blind remote: remote stores encrypted objects and facts but cannot read
  source;
- split-trust remote: remote can see approved proof summaries and redacted
  metadata while protected source and fact bodies remain encrypted.

## Phase 9: Bundles, Sync, And Rootless Podman

Build portable transfer and optional daemon support.

Required work:

- bundle manifest;
- bundle object pack;
- bundle fact range;
- bundle signature footer;
- `saga bundle create`;
- `saga bundle verify`;
- `saga bundle import`;
- minimal remote protocol;
- `saga sync`;
- `sagad serve`;
- rootless Podman smoke path for the `saga` CLI.

Sync rule:

- sync moves proof-carrying bundles;
- local work never requires network access;
- remote acceptance can allow, deny, quarantine, or ask for more evidence.

## Phase 10: Production Hardening

Build the 1.0 security and portability gates.

Required work:

- malicious object corpus;
- malicious bundle corpus;
- crash-recovery tests;
- fuzz targets for parsers;
- file permission checks for sensitive local key material;
- no secret material in logs or debug output;
- reproducible release build check;
- SBOM generation;
- release metadata validator;
- vault leak-scan fixtures;
- encrypted bundle malicious corpus;
- Linux, Windows, BSD, MacOS, Android, and iOS build checks where practical;
- documented future operating-system portability constraints so Sagnir does not
  lock itself to one host operating system.

## 1.0 Definition

Sagnir 1.0.0 is production-ready when `saga` can:

- initialize a local realm;
- inspect and diff worktree state;
- create and switch worlds;
- begin, seal, amend, and log changes;
- verify object graph integrity;
- record local test and review facts;
- prove a change or world against local policy;
- promote state between worlds;
- explain why a path exists;
- trace local blast radius;
- enable and use encrypted local realms;
- lock and unlock encrypted realm materialization;
- create and verify encrypted bundles;
- undo through the operation ledger;
- create, verify, and import bundles;
- sync with a minimal Sagnir remote;
- pass security, modularity, release, dependency, and documentation gates.
