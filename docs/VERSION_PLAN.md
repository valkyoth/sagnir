# Sagnir Version Plan

Status: planning document

Tags use:

```text
v0.N.0      milestone release
v0.N.P      patch or fix release
v1.0.0      first serious production-ready CLI
```

The list below is not a maximum. Add versions whenever smaller release stops
make review and testing clearer.

This plan is intentionally granular. Sagnir is source-state infrastructure, so
each tag should represent a small, testable step. Avoid bundling multiple
trust-boundary changes into one version.

## Release Principles

Every release must have:

- definition of done;
- a clean stop point before tag work;
- local verification command;
- rootless Podman status;
- security review notes;
- pentest handoff and resolution pass;
- known limitations;
- release notes;
- no hidden dependency on one developer machine.

Every release should prefer:

- small local-first increments;
- host tests before durable storage behavior;
- deterministic behavior before automation;
- policy-aware APIs even when enforcement is still simple;
- explicit unsupported behavior over silent compatibility promises.

## Clean Stop And Pentest Rule

Each version has a deliberate clean stop. When implementation criteria are done,
work stops before tagging and the maintainer is told:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

No tag is created at that point.

Pentest flow:

1. Implementation reaches the version stop point.
2. Local gates pass: `scripts/checks.sh`, `cargo deny check`, and `cargo audit`.
3. The maintainer runs pentest and writes findings to root `PENTEST.md`.
4. Findings are reviewed and fixed.
5. `PENTEST.md` is removed after findings are handled.
6. Local gates are run again.
7. A permanent report is written at `security/pentest/<tag>.md` only when the
   exact commit is ready to tag and the result is `Status: PASS`.
8. Tagging and pushing tags happen only when explicitly requested.

Root `PENTEST.md` is temporary scratch input. It must not be committed, and the
release metadata validator fails while it exists.

## Mandatory Per-Version Stop

Every version section below inherits this tag stop:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

Every version also inherits this pentest task:

- run the local gates for the exact commit;
- review security-sensitive changes in scope;
- write temporary findings to root `PENTEST.md`;
- fix or document every release-blocking finding;
- remove root `PENTEST.md`;
- create or update `security/pentest/<tag>.md` with `Status: PASS`, exact
  `Commit:`, non-blank `Tester:`, non-blank `Scope:`, and `Date: YYYY-MM-DD`;
- tag only after the maintainer explicitly requests tagging.

## Phase 0: Foundation

### v0.1.0 - Repository Foundation

Goal: initialize the serious Rust workspace and policy baseline.

Deliverables:

- Rust stable `1.96.0` pinned.
- Focused workspace crates.
- `saga` CLI scaffold.
- `scripts/checks.sh`.
- CI, dependency policy, security policy, release notes.
- Implementation, version, modularity, threat-model, toolchain, and security
  control docs.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_1_gate.sh`
- `cargo test --workspace`

Exit criteria:

- A contributor can read the README and understand Sagnir's 1.0 target.
- The project policy clearly forbids huge one-file implementations.

### v0.2.0 - Release Gate Baseline

Goal: make release metadata, pentest handoff, and security gates enforceable.

Deliverables:

- release metadata validator;
- pentest report validator;
- release notes validator;
- root `PENTEST.md` rejection;
- release gate script for v0.2.0;
- documentation for no tag before pentest.

Verification:

- `scripts/checks.sh`
- `scripts/release_0_2_gate.sh`

Exit criteria:

- A release cannot pass locally while root `PENTEST.md` exists.
- A release cannot pass without release notes and a pentest report placeholder.

### v0.3.0 - CLI Router And Golden Output

Goal: make `saga` command dispatch testable before adding stateful commands.

Deliverables:

- command router;
- `saga help`;
- `saga version`;
- stable exit codes;
- golden-output tests;
- unknown-command tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo run -p sagnir-cli --bin saga -- version`

Exit criteria:

- User-facing command output is stable enough for future tests.

## Phase 1: Canonical Format

### v0.4.0 - Core IDs And Bounds

Goal: make core identifiers and bounded names reliable.

Deliverables:

- typed ID wrappers;
- format version type;
- bounded name rules;
- path/name byte admission;
- no allocation in core ID validation.

Verification:

- `cargo test -p sagnir-core`

Exit criteria:

- Invalid names and oversize names fail before reaching object or store code.

### v0.5.0 - Canonical Scalar Encoding

Goal: specify and test scalar encoding before object bodies exist.

Deliverables:

- integer encoding;
- byte string encoding;
- list-length encoding;
- fail-closed buffer writes;
- malformed scalar tests.

Verification:

- `cargo test -p sagnir-codec`

Exit criteria:

- Canonical scalar bytes are deterministic and reject short buffers.

### v0.6.0 - Object Header Format

Goal: define object headers without durable storage.

Deliverables:

- object magic;
- object type field;
- format version field;
- body length field;
- flags field;
- critical-extension rejection model;
- parser tests for malformed headers.

Verification:

- `cargo test -p sagnir-object`

Exit criteria:

- Object readers can reject wrong type, wrong version, duplicate fields, and
  oversize body metadata before allocation.

### v0.7.0 - Domain-Separated Object Identity

Goal: make object identity type-separated and algorithm-agile.

Deliverables:

- object domain tags;
- algorithm identifiers;
- digest length checks;
- object ID display format;
- object ID parse tests;
- collision-domain tests across object kinds.

Verification:

- `cargo test -p sagnir-object`

Exit criteria:

- Blob, tree, state root, change, world, fact, operation, and bundle identities
  cannot be confused by equal raw digests.

### v0.8.0 - In-Memory Object Graph

Goal: verify object graph relationships before disk persistence.

Deliverables:

- in-memory object table;
- typed object references;
- missing-reference detection;
- cycle policy for object kinds;
- graph traversal tests.

Verification:

- `cargo test -p sagnir-object`

Exit criteria:

- Tests can prove a small object graph is complete or identify exact missing
  references.

## Phase 2: Local Store

### v0.9.0 - `.saga/` Directory Creation

Goal: create a local Sagnir store without writing source-state objects yet.

Deliverables:

- `saga init`;
- `.saga/FORMAT`;
- required directory creation;
- idempotent init behavior;
- interrupted-init cleanup policy.

Verification:

- `cargo test -p sagnir-store`
- `cargo run -p sagnir-cli --bin saga -- init --dry-run`

Exit criteria:

- A project can get a valid `.saga/` layout without external services.

### v0.10.0 - Realm And Config Files

Goal: persist local realm identity and config.

Deliverables:

- `.saga/realm.toml`;
- `.saga/config.toml`;
- realm ID validation;
- config read/write;
- invalid config tests.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- Sagnir can distinguish a valid local store from an unrelated directory.

### v0.11.0 - WAL Frame Format

Goal: define append-only WAL frames before writing transactions.

Deliverables:

- WAL magic;
- frame kind;
- transaction ID;
- payload length;
- checksum placeholder or admitted checksum algorithm;
- malformed frame tests.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- WAL frames reject malformed length, kind, and checksum metadata.

### v0.12.0 - WAL Writer And Recovery

Goal: make committed local transactions recoverable.

Deliverables:

- begin transaction;
- append frame;
- commit transaction;
- ignore incomplete transaction;
- replay committed frames;
- recovery tests for torn writes.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- Startup can recover committed operations and ignore incomplete ones.

### v0.13.0 - Loose Object Store

Goal: store immutable loose objects under `.saga/objects`.

Deliverables:

- object path derivation;
- temp-write then atomic publish;
- duplicate object behavior;
- hash-before-accept policy;
- corrupt loose object tests.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- Loose objects are immutable and corruption is detected before indexing.

### v0.14.0 - Local Fsck

Goal: verify local store structure and loose objects.

Deliverables:

- `saga fsck`;
- format file check;
- realm file check;
- object graph check;
- WAL replay check;
- clear failure output.

Verification:

- `cargo test -p sagnir-store`
- `cargo run -p sagnir-cli --bin saga -- fsck --dry-run`

Exit criteria:

- A user can run a local integrity check without network access.

## Phase 3: Worktree And Source State

### v0.15.0 - Worktree Path Scanner

Goal: classify source paths safely across supported operating systems.

Deliverables:

- relative path scanner;
- `.saga/` control path exclusion;
- parent traversal rejection;
- Windows separator rejection or normalization policy;
- symlink policy scaffold;
- path tests for Linux, Windows-style separators, BSD, MacOS, Android, and iOS.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Sagnir never treats `.saga/` control data as source content.

### v0.16.0 - Ignore Rules

Goal: add deterministic ignored/untracked/tracked classification.

Deliverables:

- `.sagaignore` parser;
- default ignore rules;
- untracked classification;
- ignored classification;
- invalid pattern tests.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Worktree scans produce stable tracked and ignored path sets.

### v0.17.0 - Blob And Tree Builder

Goal: build deterministic source tree objects from tracked files.

Deliverables:

- blob object creation;
- tree entry sorting;
- executable metadata policy;
- empty directory policy;
- tree hash tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Equivalent worktrees produce equivalent tree object bytes.

### v0.18.0 - State Root Object

Goal: bind source tree state to policy and crypto epochs.

Deliverables:

- state root object;
- content root reference;
- policy epoch reference;
- crypto epoch reference;
- operation reference;
- state root verification tests.

Verification:

- `cargo test -p sagnir-object`

Exit criteria:

- Sagnir can represent a complete source state without a change workflow.

### v0.19.0 - Status Command

Goal: compare worktree state against the current state root.

Deliverables:

- current world lookup scaffold;
- worktree scan integration;
- added/modified/deleted output;
- untracked output;
- status tests with fixture directories.

Verification:

- `cargo test -p sagnir-cli`
- `cargo run -p sagnir-cli --bin saga -- status`

Exit criteria:

- `saga status` is useful for a simple local project.

### v0.20.0 - Text Diff

Goal: show deterministic text diffs for tracked files.

Deliverables:

- line diff;
- deterministic path ordering;
- context line option;
- UTF-8 and non-UTF-8 behavior;
- add/modify/delete tests.

Verification:

- `cargo test -p sagnir-worktree`
- `cargo test -p sagnir-cli`

Exit criteria:

- `saga diff` can explain simple local text changes.

### v0.21.0 - Binary And Large File Bounds

Goal: protect status and diff from unbounded memory behavior.

Deliverables:

- binary detection;
- large file size limits;
- bounded read behavior;
- clear binary diff output;
- tests for large and binary files.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Large or binary files do not cause unbounded diff allocations.

## Phase 4: Worlds

### v0.22.0 - World Metadata

Goal: model worlds as first-class source states.

Deliverables:

- world object;
- world kind;
- current state reference;
- parent world references;
- accepted and quarantined change set references.

Verification:

- `cargo test -p sagnir-world`

Exit criteria:

- World metadata can represent local, draft, review, staging, production,
  audit, simulation, and agent worlds.

### v0.23.0 - World Aliases

Goal: map human world names to immutable world states.

Deliverables:

- alias file format;
- alias validation;
- alias update transaction;
- alias rollback test;
- current world pointer.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-world`

Exit criteria:

- Mutable names point only to existing immutable world states.

### v0.24.0 - World Open And List

Goal: create and inspect draft worlds.

Deliverables:

- `saga world open`;
- `saga world list`;
- parent world selection;
- duplicate world-name rejection;
- tests for world isolation.

Verification:

- `cargo test -p sagnir-cli`
- `cargo run -p sagnir-cli --bin saga -- world list`

Exit criteria:

- A user can create a draft world without mutating the source world.

### v0.25.0 - World Switch Materialization

Goal: materialize another world into the worktree safely.

Deliverables:

- tree diff between materialized states;
- atomic file update plan;
- backup/rollback plan;
- `saga world switch`;
- materialization tests.

Verification:

- `cargo test -p sagnir-worktree`
- `cargo test -p sagnir-cli`

Exit criteria:

- Switching worlds updates files without corrupting the local store.

### v0.26.0 - Dirty Worktree Protection

Goal: prevent accidental data loss during world switches and promotions.

Deliverables:

- dirty worktree detector;
- safe refusal output;
- explicit override policy;
- tests for modified, deleted, untracked, and ignored files.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Sagnir refuses destructive materialization unless the user explicitly chooses
  an admitted path.

## Phase 5: Changes And Sealing

### v0.27.0 - Change Begin

Goal: record developer intent before source state is sealed.

Deliverables:

- change object;
- title and description validation;
- base world reference;
- base state reference;
- `saga change begin`;
- active change file.

Verification:

- `cargo test -p sagnir-change`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can distinguish intent from the final sealed revision.

### v0.28.0 - Change Revision Object

Goal: represent an exact immutable version of a change.

Deliverables:

- revision ID;
- parent revision list;
- base state;
- result state;
- touched paths;
- evidence bundle reference;
- revision validation tests.

Verification:

- `cargo test -p sagnir-change`

Exit criteria:

- Revisions are immutable and tied to exact source-state transitions.

### v0.29.0 - Seal Command

Goal: turn worktree changes into a sealed revision.

Deliverables:

- `saga seal`;
- object writes;
- state root creation;
- revision creation;
- world alias update;
- WAL transaction;
- interrupted-seal tests.

Verification:

- `cargo test -p sagnir-change`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`

Exit criteria:

- `saga seal` creates immutable source-state history.

### v0.30.0 - Amend And Log

Goal: update a logical change through a new immutable revision.

Deliverables:

- `saga change amend`;
- revision parent tracking;
- `saga log`;
- `saga log --change`;
- tests for amend chains.

Verification:

- `cargo test -p sagnir-change`
- `cargo test -p sagnir-cli`

Exit criteria:

- A logical change can evolve without deleting prior sealed revisions.

### v0.31.0 - Operation Ledger

Goal: record user-facing operations as append-only history.

Deliverables:

- operation object;
- operation log;
- operation replay;
- operation display;
- `saga op log`.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- User-visible mutations are attributable to durable operation records.

### v0.32.0 - Undo

Goal: make common local mistakes reversible without erasing history.

Deliverables:

- inverse operation planner;
- `saga undo`;
- alias restore;
- worktree restore;
- refusal for unsafe undo;
- undo tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`

Exit criteria:

- Undo creates a new operation and never deletes immutable history.

## Phase 6: Proof, Policy, And Promotion

### v0.33.0 - Integrity Proof

Goal: verify object graph integrity for changes and worlds.

Deliverables:

- proof target model;
- object graph verifier;
- missing object diagnostics;
- wrong object type diagnostics;
- `saga prove` integrity checks.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can prove local object integrity offline.

### v0.34.0 - Actor Identity Metadata

Goal: make local actor and device identity explicit.

Deliverables:

- actor ID;
- device ID;
- key registry metadata;
- public key metadata storage;
- `saga actor init`;
- identity validation tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sealed revisions and facts can reference an attributable local actor.

### v0.35.0 - Signature Envelope Validation

Goal: validate signature metadata before cryptographic verification grows.

Deliverables:

- signature envelope parser;
- algorithm allow-list;
- signature byte bounds;
- signature set bounds;
- unknown algorithm tests;
- oversized signature tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- Untrusted signature metadata is bounded before durable ingest.

### v0.36.0 - Local Policy File

Goal: load local policy requirements without hosted infrastructure.

Deliverables:

- policy file format;
- world policy sections;
- seal requirements;
- promotion requirements;
- invalid policy tests.

Verification:

- `cargo test -p sagnir-policy`

Exit criteria:

- Draft, review, staging, and production policies can differ locally.

### v0.37.0 - Promotion Preflight

Goal: evaluate promotion before mutating target worlds.

Deliverables:

- source and target world selection;
- conflict categories;
- missing proof requirements;
- target policy evaluation;
- `saga promote --check`.

Verification:

- `cargo test -p sagnir-world`
- `cargo test -p sagnir-policy`

Exit criteria:

- Promotion denial is deterministic and explainable.

### v0.38.0 - Promotion Commit

Goal: move proven source state between worlds.

Deliverables:

- `saga promote`;
- promotion fact placeholder;
- target world update transaction;
- rollback preflight;
- promotion tests.

Verification:

- `cargo test -p sagnir-world`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir promotes proven state without destructive merges.

## Phase 7: Facts And Evidence

### v0.39.0 - Local Fact Envelope

Goal: record small local facts with bounded metadata.

Deliverables:

- fact ID;
- subject and predicate model;
- evidence references;
- confidence score;
- causal link list;
- bounds tests.

Verification:

- `cargo test -p sagnir-fact`

Exit criteria:

- Facts can be validated before entering the local fact log.

### v0.40.0 - Fact Log

Goal: append and replay local facts.

Deliverables:

- fact log frame;
- fact append;
- fact replay;
- duplicate fact behavior;
- corrupt fact log tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-store`

Exit criteria:

- Local facts survive process restart and corruption is detected.

### v0.41.0 - Test Evidence Recording

Goal: bind command results to sealed revisions or state roots.

Deliverables:

- `saga test record`;
- command digest;
- exit code capture;
- state root binding;
- output capture bounds;
- tests for pass, fail, and timeout.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- Test results become local evidence without trusting shell output blindly.

### v0.42.0 - Review Evidence Recording

Goal: record local review approval facts.

Deliverables:

- `saga review approve`;
- reviewer identity binding;
- revision binding;
- scope binding;
- self-review policy hook;
- review tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-policy`

Exit criteria:

- Review facts can satisfy local policy requirements.

### v0.43.0 - Why Query

Goal: explain why a path or state exists.

Deliverables:

- path provenance query;
- change-to-path index;
- fact lookup;
- `saga why`;
- stable explanation output tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- A user can trace a path back to the change and evidence that produced it.

### v0.44.0 - Local Impact Traversal

Goal: trace local blast radius from tainted inputs.

Deliverables:

- forward causal traversal;
- taint fact;
- quarantine fact;
- `saga impact`;
- tests for key, dependency, change, fact, and model identifiers.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can identify downstream local state that needs review or quarantine.

## Phase 8: Bundles And Sync

### v0.45.0 - Pack File Format

Goal: store multiple immutable objects in a bounded pack.

Deliverables:

- pack header;
- object table;
- object body offsets;
- pack footer;
- pack manifest hash;
- malformed pack tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`

Exit criteria:

- Pack readers verify bounds before trusting offsets or object counts.

### v0.46.0 - Bundle Manifest

Goal: describe a proof-carrying offline transfer.

Deliverables:

- bundle manifest;
- object refs;
- world refs;
- fact refs;
- policy refs;
- manifest validation tests.

Verification:

- `cargo test -p sagnir-sync`

Exit criteria:

- Sagnir can describe what a bundle claims before loading bundle bodies.

### v0.47.0 - Bundle Create And Verify

Goal: create and verify offline bundles.

Deliverables:

- `saga bundle create`;
- `saga bundle verify`;
- bundle signature footer placeholder;
- missing object detection;
- malicious bundle tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-cli`

Exit criteria:

- A bundle can be verified before import.

### v0.48.0 - Bundle Import

Goal: import verified bundles safely.

Deliverables:

- `saga bundle import`;
- object deduplication;
- fact deduplication;
- world alias import policy;
- quarantine-on-policy-failure behavior.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-store`

Exit criteria:

- Import cannot overwrite local world aliases without explicit policy.

### v0.49.0 - Sync Negotiation

Goal: exchange local and remote heads before transfer.

Deliverables:

- remote head request;
- missing object response;
- missing fact response;
- protocol version negotiation;
- replay rejection metadata.

Verification:

- `cargo test -p sagnir-sync`

Exit criteria:

- Sync can determine the smallest required bundle for a remote.

### v0.50.0 - Sync Transfer

Goal: transfer proof-carrying bundles to a remote endpoint.

Deliverables:

- `saga sync`;
- accepted response;
- denied response;
- quarantined response;
- local sync result fact;
- protocol tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-cli`

Exit criteria:

- Local work can sync without requiring a hosted product.

### v0.51.0 - Minimal Daemon

Goal: provide optional local and remote daemon support.

Deliverables:

- `sagad serve`;
- remote object store;
- remote fact store;
- policy-light acceptance;
- graceful shutdown;
- restart tests.

Verification:

- `cargo test -p sagad`
- `cargo run -p sagad --bin sagad`

Exit criteria:

- A minimal Sagnir remote exists for sync testing.

## Phase 9: Hardening And Portability

### v0.52.0 - Malicious Corpus

Goal: make hostile input testing part of normal development.

Deliverables:

- canonical codec corpus;
- object corpus;
- WAL corpus;
- pack corpus;
- bundle corpus;
- regression tests for every accepted corpus case.

Verification:

- `cargo test --workspace`

Exit criteria:

- Known malicious bytes stay rejected across releases.

### v0.53.0 - Fuzz And Model Test Scaffold

Goal: prepare parser and state-machine fuzzing.

Deliverables:

- fuzz target workspace;
- codec fuzz target;
- object parser fuzz target;
- bundle parser fuzz target;
- state-machine model tests;
- documentation for running fuzz targets.

Verification:

- parser unit tests;
- documented optional fuzz command.

Exit criteria:

- New parsers have a standard place to add fuzz coverage.

### v0.54.0 - Cross-Platform Build Gate

Goal: keep Sagnir portable from day one.

Deliverables:

- Linux check;
- Windows check;
- MacOS check;
- BSD instructions;
- Android check scaffold;
- iOS check scaffold;
- documented unsupported behavior.

Verification:

- local Linux gate;
- CI matrix where available.

Exit criteria:

- Platform assumptions are explicit and tested where practical.

### v0.55.0 - Rootless Podman Gate

Goal: make `saga` usable from a rootless container.

Deliverables:

- rootless Podman build;
- rootless Podman run;
- CLI smoke test;
- non-root user in image;
- container documentation.

Verification:

- `scripts/podman_smoke.sh`

Exit criteria:

- A user can run the CLI in rootless Podman.

### v0.56.0 - Release Evidence

Goal: make release outputs auditable.

Deliverables:

- SBOM generation;
- release checksum checklist;
- reproducible local release build check;
- release notes validator;
- signed tag checklist;
- release runbook.

Verification:

- `scripts/generate-sbom.sh`
- release metadata validator.

Exit criteria:

- A release candidate produces auditable local evidence.

### v0.57.0 - 1.0 Release Candidate Gate

Goal: freeze the 1.0 feature set and reject incomplete production behavior.

Deliverables:

- 1.0 release gate script;
- all required commands covered by tests;
- known limitations document;
- security controls updated;
- threat model updated;
- release notes for 1.0.0 draft.

Verification:

- `scripts/release_1_0_gate.sh`
- `scripts/checks.sh`

Exit criteria:

- The project is ready for final 1.0 pentest handoff.

## v1.0.0 - Production-Ready CLI

Goal: first serious production-ready `saga` CLI.

Deliverables:

- local realm initialization;
- world and change workflow;
- seal and amend;
- status and diff;
- proofs and promotion;
- operation undo;
- local facts;
- why and impact;
- bundles;
- minimal sync;
- optional daemon;
- complete release notes;
- completed pentest report;
- passing release gate.

Verification:

- `scripts/release_1_0_gate.sh`
- `scripts/checks.sh`
- `cargo deny check`
- `cargo audit`
- `scripts/generate-sbom.sh`
- completed 1.0 pentest report for the exact commit.

Exit criteria:

- `saga` is ready for serious local-first source-state work.
- Tagging happens only after explicit maintainer instruction.
