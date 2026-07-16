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

## UX And Policy Principles

Sagnir is strict by default. A new realm must start with strict integrity,
canonical object validation, append-only history, safe worktree rules, and
policy-aware state transitions. Convenience commands must not bypass configured
policy.

Daily-use ergonomics are still mandatory. The CLI should offer high-level
commands for common local workflows so normal developers do not need to think
about every primitive object, proof, or world transition for simple work.

Planned profile shape:

- `standard`: default strict integrity with simple local workflow commands;
- `solo`: explicit opt-in profile with fewer evidence, review, and signature
  requirements for private local work;
- `team`: profile for signatures, reviews, and protected worlds;
- `regulated`: strict signatures, evidence, audit, and promotion policy for
  healthcare, sovereign, and other high-assurance environments.

`saga save "message"` is planned as secure workflow sugar. It composes native
Sagnir operations such as intent creation, source-state transition building,
sealing, operation recording, local proof evaluation, and current-world update.
It is not a Git commit clone and must fail when the active profile or world
policy requires more evidence.

## Verification Scale Principles

Bounded verification is a security boundary, not a repository-size promise.
`OBJECT_GRAPH_ENTRIES_MAX` and `OBJECT_GRAPH_REFS_MAX` are per-batch admission
budgets. Large repositories must use chunked verification, changed-cone
verification, rebuildable indexes, and cached proofs so a small save does not
require reloading an entire world.

Planned verification modes:

- `bounded-batch`: fixed-capacity admission for one object/reference batch;
- `lazy-cone`: default scalable mode for daily work, verifying only the touched
  source-state cone plus required boundary proofs;
- `full-world`: explicit high-assurance mode that verifies a whole world within
  configured resource budgets.

Configuration should be ergonomic and safe. If a user sets only
`memory_budget`, Sagnir should derive safe chunk sizes and graph limits from
that budget. If a user sets only `parallelism`, Sagnir should schedule work
within default memory limits. If exact `max_entries` or `max_refs` are set, they
act as hard ceilings.

Remote state must be preflighted before trust. `saga clone`, bundle import, and
sync should inspect lightweight metadata first: object/ref estimates, chunk
manifest, required profile, minimum verification mode, and estimated resource
needs. Sagnir must refuse trust or worktree materialization when upstream policy
requires stronger verification than local settings can satisfy. Quarantine and
`--no-worktree` fetch are allowed inspection states; they are not trusted
materialization.

## Trust Pipeline And Convergence Principles

Names that imply verification must match the security property actually
established. Structural graph checks over caller-supplied IDs and edges are not
cryptographic proofs. The production ingest path must derive authority from
canonical bytes:

```text
UntrustedBytes
  -> CanonicallyParsed
  -> HashVerified
  -> ReferencesDerived
  -> CausallyClosed
  -> SignaturesVerified
  -> PolicyAdmitted
  -> DurablyCommitted
```

Each transition must use a distinct type or capability that is bound to its
target. Sagnir must not expose a reusable generic verification token. Verified
results must commit to the target, proof, scope, policy root and epoch, crypto
epoch, verifier version, and verified frontier.

Protected transitions require one validated compound result:

- content integrity is valid;
- required signatures are valid and context-bound;
- causal closure is valid;
- policy decided `allow`;
- every required obligation is discharged;
- the exact admitted result is durably committed.

Append-only local files do not by themselves prevent rollback or equivocation.
Authoritative events require per-actor sequence chains, signed frontier
checkpoints, compare-and-swap updates, and explicit equivocation evidence.
Durability requires authenticated transaction commitments in addition to
non-adversarial corruption checks such as CRC.

World promotion cannot overwrite divergence. World aliases must retain
concurrent heads, fast-forward only when ancestry permits it, and otherwise
create an explicit multi-parent transition with deterministic merge inputs,
conflict objects, policy commitments, and signatures. Source text is merged by
an explicit reproducible transition, not silently by a metadata CRDT.

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

- Rust stable `1.97.0` pinned.
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
- `release-notes/RELEASE_NOTES_0.2.0.md`;
- `security/pentest/v0.2.0.md`;
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
- fuzz target scaffold for object-header parsing.

Verification:

- `cargo test -p sagnir-object`

Exit criteria:

- Object readers can reject wrong type, wrong version, duplicate fields, and
  oversize body metadata before allocation.
- New object parsers have a standard fuzz target location from the first parser
  milestone.

### v0.7.0 - Domain-Separated Object Identity

Goal: make object identity type-separated and algorithm-agile.

Deliverables:

- object domain tags;
- algorithm identifiers;
- digest length checks;
- object ID display format;
- object ID parse tests;
- equality policy separating public IDs from secret values and authentication
  tags;
- collision-domain tests across object kinds.

Verification:

- `cargo test -p sagnir-object`

Exit criteria:

- Blob, tree, state root, change, world, fact, operation, and bundle identities
  cannot be confused by equal raw digests.
- Public object IDs use efficient ordinary equality; timing-safe comparison is
  reserved for secret values and authentication tags where timing leakage is a
  relevant security property.

### v0.8.0 - In-Memory Object Graph

Goal: structurally validate caller-supplied object graph relationships before
disk persistence.

Deliverables:

- in-memory object table;
- typed object references;
- missing-reference detection;
- cycle policy for object kinds;
- iterative graph traversal tests;
- object graph fuzz targets;
- documented graph capacity constants.

Verification:

- `cargo test -p sagnir-object`
- `cargo check --manifest-path fuzz/Cargo.toml --bins`

Exit criteria:

- Tests can report whether a supplied small graph is structurally complete or
  identify exact missing supplied references.
- This release does not claim body-derived or cryptographic completeness; those
  trust properties begin at v0.12.0 and v0.13.0.
- Traversal is bounded and iterative so hostile graph shape cannot recurse into
  the host stack.

## Phase 2: Trust Format And Local Store

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
- default `standard` profile metadata;
- explicit profile parser for `standard`, `solo`, `team`, and `regulated`;
- verification config parser for mode, memory budget, parallelism, max entries,
  and max refs;
- realm ID validation;
- config read/write;
- invalid config tests;
- owner-checked, handle-relative Unix initialization;
- fail-closed stateful initialization on platforms without an admitted native
  handle-relative backend.

Admitted v0.10.0 defaults:

- `profile = "standard"`;
- `mode = "lazy-cone"`;
- `memory_budget = "512MiB"`;
- parallelism, max entries, and max refs remain optional hard controls.

The profile and mode are persisted metadata in this release. Repository-scale
graph execution and derived scheduling remain assigned to v0.18.0, while
profile-to-policy enforcement remains assigned to v0.49.0.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`
- `cargo run -p sagnir-cli --bin saga -- init --dry-run`

Exit criteria:

- Sagnir can distinguish a valid local store from an unrelated directory.
- A new realm records a strict default profile without weakening the security
  model.
- A realm can record verification budgets without requiring users to calculate
  every low-level graph limit manually.
- Unix initialization rejects store detachment, foreign ownership, and
  temporary-file substitution before reporting success.
- Unsupported stateful backends refuse initialization before creating
  `.saga/`.

### v0.10.1 - Native Windows Store Initialization

Goal: restore stateful Windows initialization without path-based race windows.

Deliverables:

- retained Windows root and `.saga/` directory handles;
- component-by-component reparse-point refusal;
- handle-relative directory and file operations;
- Windows file-ID verification around metadata commits;
- owner and access-control admission policy;
- hosted Windows junction, symlink, namespace replacement, and temp-file race
  tests;
- removal of the non-Unix `Unsupported` stop only after those tests pass.

Verification:

- hosted `windows-latest` `cargo test -p sagnir-cli`;
- independent Windows filesystem pentest.

Exit criteria:

- Windows `saga init` cannot be redirected through a junction, symlink,
  reparse point, namespace replacement, or temporary-file substitution.
- Windows initialization has the same fail-closed attachment and commit
  guarantees as the Unix backend.

### v0.11.0 - Normative Canonical Protocol Specification

Goal: publish the exact bytes and validation rules that later trust boundaries
will use.

Deliverables:

- normative scalar, header, object-body, identifier, and extension rules;
- canonical ordering and duplicate-field rules;
- cross-language known-answer test vectors;
- malformed and non-canonical vector corpus;
- versioning and critical-extension compatibility rules;
- explicit statements of what structural validation does and does not prove.

Verification:

- `cargo test -p sagnir-codec`
- `cargo test -p sagnir-object`
- documentation vector validator.

Exit criteria:

- An independent implementation can produce identical canonical bytes and
  reject the same non-canonical inputs.
- No later signature or hash transcript depends on undocumented serialization.

### v0.12.0 - Canonical Object Body Decoders

Goal: make object bodies authoritative instead of trusting caller-supplied
relationship metadata.

Deliverables:

- bounded canonical decoders for every admitted object kind;
- typed decoded bodies;
- object-kind-specific required and optional fields;
- duplicate, unknown-critical-field, and trailing-byte rejection;
- fuzz targets and malformed-body vectors for every decoder.

Verification:

- `cargo test -p sagnir-object`
- `cargo check --manifest-path fuzz/Cargo.toml --bins`

Exit criteria:

- Every admitted object kind has one canonical decoder.
- Untrusted callers cannot construct an authoritative object by supplying an ID
  and relationships without the corresponding canonical body.

### v0.13.0 - Hash Computation And Derived References

Goal: compute object identity and references from canonical bytes.

Deliverables:

- reviewed hash-provider admission;
- streaming domain-separated object hashing;
- hash-versus-claimed-ID verification;
- body-derived typed reference extraction;
- allowed edge schemas between object kinds;
- omitted, extra, mistyped, and cross-kind reference tests.

Verification:

- `cargo test -p sagnir-object`
- known-answer hash vectors;
- object graph fuzz targets.

Exit criteria:

- A complete graph result means every object digest was recomputed and every
  authoritative edge was derived from its body.
- Caller-supplied edge lists are diagnostic input only, never the source of
  truth.

### v0.14.0 - Identifier Privacy And Realm Scoping

Goal: prevent public, private, ciphertext, and redacted identifiers from being
interchanged before durable formats depend on them.

Deliverables:

- separate `PublicObjectId`, `PrivateObjectId`, `CiphertextStorageId`, and
  `RedactedCommitment` types;
- realm-scoped identity rules for revisions, facts, decisions, and transitions;
- non-revealing formatting for private identifiers;
- explicit debug and log redaction policy;
- migration notes for existing scaffold identifiers;
- compile-fail and runtime misuse tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-core`

Exit criteria:

- Private identifiers cannot accidentally use a revealing `Display` path.
- Realm-bound statements cannot be replayed as realm-neutral content.

### v0.15.0 - Authenticated WAL Frame Format

Goal: define bounded WAL frames and an adversarial transaction commitment
before writing transactions.

Deliverables:

- WAL magic;
- WAL format and version;
- frame kind;
- transaction ID;
- monotonic frame sequence;
- payload length;
- CRC or equivalent corruption check;
- ordered payload digest commitment;
- previous committed transaction or checkpoint commitment;
- commit marker with expected resulting realm frontier;
- malformed frame tests.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- WAL frames reject malformed length, kind, sequence, and checksum metadata.
- A forged CRC cannot make reordered, removed, inserted, or replayed frames
  appear as one valid committed transaction.

### v0.16.0 - WAL Writer And Recovery

Goal: make committed local transactions recoverable.

Deliverables:

- begin transaction;
- append frame;
- commit transaction;
- ignore incomplete transaction;
- replay committed frames;
- file and directory synchronization at required durability boundaries;
- recovery tests for torn writes and every write/rename/sync boundary;
- refusal when a committed alias lacks any referenced immutable body.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- Startup can recover committed operations and ignore incomplete ones.
- Recovery cannot apply a committed frontier or alias before all transaction
  bodies are present and verified.

### v0.17.0 - Loose Object Store

Goal: store immutable loose objects under `.saga/objects`.

Deliverables:

- object path derivation;
- temp-write then atomic publish;
- duplicate object behavior;
- canonical-decode, hash, and body-derived-reference checks before acceptance;
- corrupt loose object tests.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- Loose objects are immutable and corruption is detected before indexing.

### v0.18.0 - Local Fsck And Verification Modes

Goal: verify local store structure and loose objects through explicit resource
budgets.

Deliverables:

- `saga fsck`;
- `saga fsck --mode bounded-batch`;
- `saga fsck --mode lazy-cone`;
- `saga fsck --mode full-world`;
- memory-budget admission and derived chunk sizing;
- parallelism admission and bounded scheduler plan;
- format file check;
- realm file check;
- non-mutating realm/config repair plan output;
- object graph check;
- WAL replay check;
- checkpoint and transaction-chain verification;
- proof-cache generation and stale-cache diagnostics;
- clear failure output.

Verification:

- `cargo test -p sagnir-store`
- `cargo run -p sagnir-cli --bin saga -- fsck --dry-run`

Exit criteria:

- A user can run a local integrity check without network access.
- Corrupt metadata produces a backup-first repair plan without silently
  replacing realm identity or profile metadata.
- High-resource users can request full-world verification without making that
  unbounded or mandatory for normal machines.
- If only memory or parallelism is configured, Sagnir derives conservative
  internal limits from the available budget.

## Phase 3: Worktree And Source State

### v0.19.0 - Byte-Preserving Worktree Path Model

Goal: represent real cross-platform source paths without excluding valid
projects or confusing display text with path identity.

Deliverables:

- byte-preserving Unix path components;
- platform-native Windows path component representation;
- explicit filesystem normalization and case-collision policy;
- support for `.github`, `.cargo`, `.gitignore`, and other dotfiles;
- exclusion of `.saga/` specifically rather than all dot-prefixed paths;
- display escaping that never changes canonical path identity;
- fixtures for non-UTF-8, normalization collisions, reserved names, and case
  collisions.

Verification:

- `cargo test -p sagnir-worktree`
- hosted platform path fixtures where available.

Exit criteria:

- Sagnir can track ordinary platform-native source trees, including dotfiles
  and non-Unicode paths where the host supports them.
- Ambiguous or unrepresentable cross-platform paths fail with an explicit
  portability diagnosis.

### v0.20.0 - Worktree Path Scanner

Goal: classify source paths safely across supported operating systems.

Deliverables:

- relative path scanner;
- `.saga/` control path exclusion;
- parent traversal rejection;
- Windows separator rejection policy;
- root-bound directory handle traversal;
- symlink and reparse-point policy;
- resolver output bound to the exact root, path, and file identity;
- no reusable zero-sized symlink proof capability;
- replacement-race tests around scan and materialization;
- path tests for Linux, Windows-style separators, BSD, MacOS, Android, and iOS.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Sagnir never treats `.saga/` control data as source content.
- Windows-style separator inputs are rejected consistently before control-path
  materialization.
- A path admitted under one root cannot be reused as proof for another root or
  after the underlying file identity changes.

### v0.21.0 - Ignore Rules

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

### v0.22.0 - Blob And Tree Builder

Goal: build deterministic source tree objects from tracked files.

Deliverables:

- blob object creation;
- one-pass streaming file hashing;
- tree entry sorting;
- incremental tree construction under memory budgets;
- executable metadata policy;
- empty directory policy;
- tree hash tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Equivalent worktrees produce equivalent tree object bytes.

### v0.23.0 - State Root Object

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

### v0.24.0 - Status Command

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

### v0.25.0 - Text Diff

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

### v0.26.0 - Binary And Large File Bounds

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

### v0.27.0 - World Metadata

Goal: model worlds as first-class source states.

Deliverables:

- world object;
- world kind;
- current state reference;
- frontier set and parent world-state references;
- stable replica/device causal identity;
- compact dotted-version-vector or Merkle-clock context;
- typed edges for `depends_on`, `derived_from`, `supersedes`,
  `attests_to`, and `relates_to`;
- acyclicity enforced only for dependency edge classes;
- accepted and quarantined change set references.

Verification:

- `cargo test -p sagnir-world`

Exit criteria:

- World metadata can represent local, draft, review, staging, production,
  audit, simulation, and agent worlds.
- Concurrent state is represented as an explicit frontier rather than being
  collapsed into one last-writer-wins head.

### v0.28.0 - World Aliases

Goal: map human world names to immutable world states.

Deliverables:

- alias file format;
- alias validation;
- causally versioned multi-value alias register;
- expected-old-frontier compare-and-swap;
- realm-scoped writer serialization or admitted MVCC;
- concurrent-head limits and explicit resolution state;
- alias update transaction atomically bound to objects and world states;
- alias rollback test;
- concurrent writer and stale-CAS tests;
- current world pointer.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-world`

Exit criteria:

- Mutable names point only to existing immutable world states.
- Concurrent alias updates preserve every admitted head and never silently
  discard divergent history.

### v0.29.0 - World Open And List

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

### v0.30.0 - World Switch Materialization

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

### v0.31.0 - Dirty Worktree Protection

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

### v0.32.0 - Change Begin

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

### v0.33.0 - Change Revision Object

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

### v0.34.0 - Seal Command

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

### v0.35.0 - Save, Amend, And Log

Goal: support both explicit change evolution and a simple secure local save
workflow.

Deliverables:

- `saga save "message"`;
- auto-create local intent when no active change exists;
- policy check before world update;
- operation record for the save workflow;
- clear denial output when policy requires more evidence;
- `saga change amend`;
- revision parent tracking;
- `saga log`;
- `saga log --change`;
- tests for save workflow, policy denial, and amend chains.

Verification:

- `cargo test -p sagnir-change`
- `cargo test -p sagnir-cli`

Exit criteria:

- `saga save` creates native Sagnir changes and sealed revisions without
  exposing every primitive step to normal users.
- `saga save` never bypasses the active profile or world policy.
- A logical change can evolve without deleting prior sealed revisions.

### v0.36.0 - Operation Ledger

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

### v0.37.0 - Undo

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

## Phase 6: Cryptography, Proof, Convergence, And Promotion

### v0.38.0 - Typed Ingest Pipeline

Goal: encode the trust pipeline so partially checked data cannot reach durable
or policy-authoritative APIs.

Deliverables:

- distinct untrusted, canonical, hash-verified, reference-derived,
  causally-closed, signature-verified, policy-admitted, and committed types;
- consuming transitions between each stage;
- no public constructors that skip stages;
- diagnostic structural-validation results named separately from proofs;
- compile-fail tests for invalid stage reuse and bypass attempts.

Verification:

- `cargo test --workspace`
- API compile-fail test suite.

Exit criteria:

- Durable acceptance APIs cannot receive raw bytes, caller-supplied graph
  entries, or generic validation flags.
- Structural graph validation is never reported as cryptographic proof.

### v0.39.0 - Actor, Device, And Replica Identity

Goal: make every authoritative signer and causal writer explicit.

Deliverables:

- actor ID;
- device and replica ID;
- key registry metadata;
- public key metadata storage;
- actor-to-device authorization records;
- `saga actor init`;
- identity collision, substitution, and malformed-registry tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sealed revisions, facts, world transitions, and sequence chains can identify
  the exact authorized actor and device that produced them.

### v0.40.0 - Signature Envelope And Set Admission

Goal: bound untrusted signature metadata before invoking cryptography.

Deliverables:

- signature envelope parser;
- algorithm and suite allow-list;
- signer key ID and key epoch;
- per-signature and total signature-set byte bounds;
- role and threshold metadata bounds;
- duplicate-signer rejection;
- unknown, oversized, duplicate, and conflicting signature tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- Untrusted signature sets are bounded, uniquely attributed, and structurally
  valid before cryptographic verification.

### v0.41.0 - Canonical Signed Statement Transcripts

Goal: bind every authoritative signature to its complete action and context.

Deliverables:

- versioned transcript format per statement/action type;
- realm, object or transition, parent/frontier, base root, and result root
  commitments;
- source and target world commitments where applicable;
- policy root and epoch, crypto suite and epoch, signer key and epoch;
- per-key sequence number and verification scope;
- domain separation and cross-statement replay tests;
- transcript known-answer vectors.

Verification:

- `cargo test -p sagnir-crypto`
- transcript vector validator.

Exit criteria:

- A valid signature for one realm, action, epoch, world, scope, or frontier
  cannot authorize another.

### v0.42.0 - Signing And Verification Providers

Goal: perform actual admitted cryptographic signing and verification.

Deliverables:

- reviewed provider abstraction;
- one mandatory production signature suite;
- key generation/import boundary;
- signing and verification APIs over canonical transcripts only;
- known-answer and established malformed/adversarial vectors;
- secret redaction and zeroization through the admitted sanitization crate;
- provider failure and unsupported-suite tests.

Verification:

- `cargo test -p sagnir-crypto`
- provider known-answer vector suite;
- `cargo deny check`

Exit criteria:

- Sagnir can create and verify real context-bound signatures.
- Protected operations do not rely on envelope length checks or signature
  placeholders.

### v0.43.0 - Key Lifecycle And Anti-Replay

Goal: make key rotation, revocation, and replay rules explicit before signatures
authorize transitions.

Deliverables:

- key epoch transitions;
- revocation and compromise evidence;
- per-key monotonic sequence admission;
- stale-key and stale-epoch rejection;
- duplicate sequence and replay detection;
- bounded role/threshold evaluation;
- documentation that recipient removal cannot revoke already acquired keys.

Verification:

- `cargo test -p sagnir-crypto`
- state-machine property tests for key lifecycle.

Exit criteria:

- A valid old signature cannot silently regain authority after revocation,
  rotation, or policy epoch advancement.

### v0.44.0 - Signed Event DAG And Checkpoints

Goal: detect rollback and equivocation without imposing one global blockchain
order.

Deliverables:

- signed per-replica sequence chains;
- event DAG parent/frontier commitments;
- signed realm frontier checkpoints;
- equivocation evidence for conflicting events at one sequence;
- missing-suffix and rollback diagnostics;
- optional peer or hardware checkpoint witness interface;
- fork and checkpoint property tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-crypto`
- event-DAG model tests.

Exit criteria:

- Deleted suffixes, stale snapshots, and actor equivocation are detectable when
  compared with an admitted later checkpoint or witness.
- Sagnir documents the limits of purely local rollback detection honestly.

### v0.45.0 - Target-Bound Verification Results

Goal: replace generic bearer-style verification capability with immutable,
target-specific results.

Deliverables:

- `Verified<Target>`-style result bound to target and proof IDs;
- scope, policy root and epoch, crypto epoch, verifier version, and verified
  frontier fields;
- non-`Copy` ownership and consumption rules for authoritative admission;
- stale epoch, wrong target, wrong scope, and wrong frontier tests;
- removal of generic reusable verification tokens.

Verification:

- `cargo test -p sagnir-proof`
- API compile-fail tests.

Exit criteria:

- Verification of one object, change, world, scope, or epoch cannot authorize
  another operation.

### v0.46.0 - Integrity Proof

Goal: produce target-bound integrity results from canonical bodies rather than
caller-supplied graph claims.

Deliverables:

- proof target model;
- canonical body decoder and hash verifier integration;
- body-derived typed-reference graph verifier;
- chunk proof model;
- changed-cone proof model;
- full-world proof mode;
- boundary commitments and cross-chunk cycle detection;
- dense node indexes, sorted lookup tables, and streaming traversal;
- missing object diagnostics;
- wrong object type diagnostics;
- `saga prove` integrity checks.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can prove local object integrity offline.
- Large worlds can be proven by composing bounded chunk proofs instead of
  loading every object into one graph.
- Protected worlds can require full-world proofs when policy demands it.
- General relationship cycles remain representable while dependency cycles are
  rejected.

### v0.47.0 - Proof Artifact And Soundness Suite

Goal: define exactly what each proof proves and make proof artifacts portable.

Deliverables:

- canonical proof artifact envelope;
- inclusion and absence proofs;
- append-only consistency proofs;
- changed-cone and causal-closure proofs;
- completeness claim with explicit scope and assumptions;
- verifier-version binding;
- proof soundness and non-goal statements;
- malformed, truncated, substituted, and scope-confusion tests.

Verification:

- `cargo test -p sagnir-proof`
- proof vector validator.

Exit criteria:

- Every proof kind states its target, assumptions, coverage, and limitations.
- A proof cannot claim full-world completeness when it covers only a changed
  cone or bounded chunk.

### v0.48.0 - Proof Cache And Incremental Verification

Goal: reuse verified immutable subtrees without accepting stale security state.

Deliverables:

- cache key bound to target root, verifier version, policy root and epoch,
  crypto epoch, and verified frontier;
- generation-number invalidation;
- persistent verified-subtree cache;
- changed-cone cache reuse;
- stale, substituted, partially written, and epoch-change cache tests;
- cache deletion and deterministic rebuild behavior.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-store`

Exit criteria:

- Cached results can accelerate one-file changes in large worlds without
  surviving any relevant verifier, policy, crypto, or frontier change.

### v0.49.0 - Local Policy File

Goal: load local policy requirements without hosted infrastructure.

Deliverables:

- policy file format;
- profile-to-policy defaults for `standard`, `solo`, `team`, and `regulated`;
- world policy sections;
- seal requirements;
- promotion requirements;
- validated compound admission result combining integrity, signatures, causal
  closure, policy decision, and discharged obligations;
- impossible-state prevention for `allow` with unsatisfied obligations;
- invalid policy tests.

Verification:

- `cargo test -p sagnir-policy`

Exit criteria:

- Draft, review, staging, and production policies can differ locally.
- Relaxed behavior is explicit through profile selection; strict environments
  can require signatures, evidence, review, and promotion checks.
- Promotion code consumes one validated admission result rather than checking
  independent enums that can contradict each other.

### v0.50.0 - World Transition Object

Goal: represent every world-state move as an immutable, signed transition.

Deliverables:

- all parent world-state IDs;
- causal frontier before the operation;
- selected merge bases;
- base, source, target, and result state roots;
- deterministic transition and merge algorithm versions;
- conflict and resolution references;
- proof, policy, and crypto commitments;
- canonical signed transcript integration.

Verification:

- `cargo test -p sagnir-world`
- `cargo test -p sagnir-crypto`

Exit criteria:

- No world mutation can be represented as an unexplained alias overwrite.

### v0.51.0 - Deterministic Merge Base And Fast-Forward

Goal: define convergence when peers advance the same world concurrently.

Deliverables:

- ancestry and frontier comparison;
- deterministic merge-base selection;
- fast-forward only when the target frontier is an ancestor;
- multi-parent result when histories diverge;
- criss-cross, missing-parent, and adversarial-depth tests;
- bounded ancestry traversal with cancellation.

Verification:

- `cargo test -p sagnir-world`
- merge state-machine property tests.

Exit criteria:

- Divergent history is preserved and cannot be discarded by promotion or sync.

### v0.52.0 - Deterministic Tree Merge

Goal: create reproducible source-state results for divergent worlds.

Deliverables:

- three-way tree merge;
- deterministic path ordering;
- add/add, delete/modify, type-change, executable-bit, and rename handling;
- binary and oversized-file merge policy;
- merge algorithm version committed into the transition;
- independent replay and fixture tests.

Verification:

- `cargo test -p sagnir-worktree`
- `cargo test -p sagnir-world`

Exit criteria:

- Identical merge inputs and algorithm versions produce identical result trees
  on every supported platform.

### v0.53.0 - Conflict And Resolution Objects

Goal: preserve unresolved source, policy, compartment, and evidence conflicts
as explicit state.

Deliverables:

- typed conflict object;
- explicit resolution object;
- source-text, rename, type, policy, compartment, and evidence conflict kinds;
- resolver actor and rationale binding;
- no silent arbitrary-text CRDT merge;
- conflict fanout and duplicate-resolution bounds;
- conflict round-trip and malicious-input tests.

Verification:

- `cargo test -p sagnir-world`
- `cargo test -p sagnir-policy`

Exit criteria:

- A world with concurrent heads or unresolved conflicts cannot be represented
  as cleanly promoted.

### v0.54.0 - Promotion Preflight

Goal: evaluate promotion before mutating target worlds.

Deliverables:

- source and target world selection;
- target-frontier compare-and-swap precondition;
- fast-forward versus multi-parent transition selection;
- conflict categories;
- missing proof requirements;
- context-bound signature and threshold evaluation;
- compound target policy admission;
- `saga promote --check`.

Verification:

- `cargo test -p sagnir-world`
- `cargo test -p sagnir-policy`

Exit criteria:

- Promotion denial is deterministic and explainable.
- Preflight cannot approve a transition against a stale target frontier.

### v0.55.0 - Promotion Commit

Goal: move proven source state between worlds.

Deliverables:

- `saga promote`;
- signed promotion fact;
- immutable world transition and result state;
- authenticated target world update transaction;
- target alias compare-and-swap;
- equivocation and concurrent-head retention;
- rollback preflight;
- promotion tests.

Verification:

- `cargo test -p sagnir-world`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir promotes admitted state without destructive merges or divergent-history
  loss.
- Protected promotion consumes real signatures and target-bound proof results;
  placeholders cannot authorize it.

## Phase 7: Facts And Evidence

### v0.56.0 - Local Fact Envelope

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

### v0.57.0 - Fact Log

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

### v0.58.0 - Test Evidence Recording

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

### v0.59.0 - Review Evidence Recording

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

### v0.60.0 - Why Query

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

### v0.61.0 - Local Impact Traversal

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

## Phase 8: Causal Memory And Explanation

### v0.62.0 - Structured Event Log

Goal: separate noisy command events from stable canonical facts.

Deliverables:

- event envelope;
- event kind registry;
- operation-to-event binding;
- bounded command argument digest;
- event replay tests;
- corrupt event log tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-fact`

Exit criteria:

- Every state-changing `saga` command can emit bounded events without making
  those events authoritative facts.

### v0.63.0 - Fact Compiler

Goal: derive stable local facts from admitted events and objects.

Deliverables:

- fact compiler input model;
- event-to-fact derivation rules;
- missing-source rejection;
- duplicate derivation behavior;
- compiler replay tests.

Verification:

- `cargo test -p sagnir-fact`

Exit criteria:

- Rebuilding facts from canonical objects and admitted events is deterministic.

### v0.64.0 - Causal Graph Indexes

Goal: build rebuildable indexes for forward and reverse causal traversal.

Deliverables:

- causal edge model;
- forward causal index;
- reverse causal index;
- path-to-fact index;
- operation index;
- stale-index detection.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-store`

Exit criteria:

- Deleting indexes does not delete truth; Sagnir can rebuild memory projections
  from canonical objects, events, and facts.

### v0.65.0 - Explanation Object

Goal: make explanations auditable instead of transient text output.

Deliverables:

- explanation object;
- query plan reference;
- evidence edge list;
- missing evidence list;
- redaction notice list;
- explanation verification tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-proof`

Exit criteria:

- An explanation can be inspected later and tied to the exact facts, objects,
  and policy decisions used to produce it.

### v0.66.0 - Explain Command

Goal: explain local changes, decisions, worlds, and operations.

Deliverables:

- `saga explain change`;
- `saga explain decision`;
- `saga explain world`;
- `saga op explain last`;
- missing-evidence output;
- golden-output tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-fact`

Exit criteria:

- Sagnir can answer why a local policy decision or operation succeeded or
  failed without external infrastructure.

### v0.67.0 - Trace Command

Goal: follow local causal paths across changes, facts, proofs, and worlds.

Deliverables:

- `saga trace change`;
- `saga trace world`;
- `saga trace fact`;
- `saga trace operation`;
- confidence and uncertainty output;
- trace traversal tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can show causal chains and clearly mark derived or uncertain analysis.

### v0.68.0 - Context Packs

Goal: build deterministic context packages for diagnostics and optional AI
summaries.

Deliverables:

- context pack object;
- `saga context build`;
- fact and object selection rules;
- redaction rules;
- missing-evidence section;
- context pack verification tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can prepare bounded evidence packs without exposing unrelated local
  source, facts, keys, or protected metadata.

### v0.69.0 - Ask Query Scaffold

Goal: allow natural-language questions only as a bounded layer over
deterministic facts.

Deliverables:

- `saga ask` query-plan scaffold;
- deterministic retrieval before optional summarization;
- fact ID citations;
- redaction notices;
- output that separates known, inferred, and missing information;
- tests that AI output cannot become authoritative evidence.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-proof`

Exit criteria:

- `saga ask` cannot approve changes, override policy, create authoritative
  facts, hide missing evidence, or promote worlds.

## Phase 9: Native Encrypted Realms

### v0.70.0 - Vault Key Hierarchy And Derivation

Goal: define cryptographic key separation before encrypted bytes are written.

Deliverables:

- key-encryption-key, realm, compartment/world, and data-key hierarchy;
- domain-separated derivation for metadata, objects, indexes, WAL, wrapping,
  and proof-disclosure keys;
- key identifier and crypto epoch binding;
- bounded KDF and derivation parameter admission;
- cross-purpose key-reuse rejection;
- known-answer derivation vectors.

Verification:

- `cargo test -p sagnir-crypto`
- key-derivation vector validator.

Exit criteria:

- Compromise or misuse of one derived purpose key does not silently authorize a
  different cryptographic purpose.

### v0.71.0 - Authenticated Encrypted Pages And Index

Goal: support bounded random access without decrypting an entire realm.

Deliverables:

- authenticated encrypted page or segment format;
- encrypted manifest and index;
- signed pack/root commitment;
- compression-before-encryption policy;
- authentication before decompression or plaintext parsing;
- expanded-size and decompression-ratio limits;
- ciphertext storage hash computed during encryption;
- random-read, substitution, truncation, and decompression-bomb tests.

Verification:

- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-store`

Exit criteria:

- Status and index lookup can read bounded authenticated regions.
- Unauthenticated ciphertext never reaches decompression or canonical parsing.

### v0.72.0 - Device Recipients And Recovery Model

Goal: define access recovery and device authorization without one shared user
secret.

Deliverables:

- per-device recipient keys;
- OS keychain, TPM, secure enclave, and hardware-token backend interfaces;
- threshold and offline recovery metadata;
- recipient and key-transparency records;
- key compromise and recovery evidence;
- forward-secrecy versus permanent-history access policy;
- backend-unavailable and recovery-threshold tests.

Verification:

- `cargo test -p sagnir-crypto`
- backend contract tests with software fixtures.

Exit criteria:

- The format supports device-specific access, revocation evidence, and offline
  recovery without embedding a mandatory platform backend.

### v0.73.0 - Vault Metadata Model

Goal: represent encrypted realm state without encrypting data yet.

Deliverables:

- vault enabled marker;
- vault mode model;
- crypto epoch reference;
- protected metadata policy;
- encrypted realm status model;
- tests for invalid vault metadata.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-store`

Exit criteria:

- Sagnir can distinguish open realms from encrypted realms before touching
  object encryption.

### v0.74.0 - Encrypted Object Envelope

Goal: define encrypted object bytes and authenticated metadata.

Deliverables:

- encrypted object magic;
- AEAD algorithm identifier;
- nonce field;
- ciphertext length field;
- authentication tag field;
- associated-data binding for envelope format and suite, realm, compartment,
  object schema and type, private commitment, crypto epoch, key ID,
  compression/delta format, pack/segment, and record position;
- malformed envelope tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-crypto`

Exit criteria:

- Encrypted object metadata is bounded and context-bound before decryption.

### v0.75.0 - Passphrase Unlock Baseline

Goal: support one local unlock method for development and tests.

Deliverables:

- passphrase-based key wrapping metadata;
- admitted memory-hard KDF and bounded parameter ranges;
- key-encryption-key metadata;
- realm-master-key wrapping model;
- denial-of-service limits for untrusted KDF parameters;
- no passphrase in logs or debug output tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- A passphrase can unlock a test realm key without becoming the realm key.

### v0.76.0 - Encrypt Project Command

Goal: enable encrypted realm storage through `saga`.

Deliverables:

- `saga encrypt project`;
- vault init transaction;
- encryption-enabled fact placeholder;
- existing object migration plan;
- refusal for already encrypted realm;
- dry-run output tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-store`

Exit criteria:

- A user can turn a local realm into an encrypted realm through an explicit
  command.

### v0.77.0 - Unlock Command

Goal: load admitted keys for a local encrypted realm.

Deliverables:

- `saga unlock`;
- unlock session metadata;
- time-to-live metadata;
- `--no-worktree` verification mode;
- failed unlock tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-crypto`

Exit criteria:

- Sagnir can verify encrypted storage without always materializing plaintext.

### v0.78.0 - Lock Command

Goal: evict local unlock state and optionally remove materialized plaintext.

Deliverables:

- `saga lock`;
- key eviction metadata;
- `--wipe-worktree`;
- `--keep-worktree`;
- warning text about imperfect plaintext cleanup;
- lock tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Sagnir clearly separates encrypted storage from plaintext worktree state.

### v0.79.0 - Vault Status And Leak Scanner

Goal: make encrypted realm state and plaintext leak surfaces visible.

Deliverables:

- `saga vault status`;
- `saga vault scan-leaks`;
- ignored-directory checks;
- editor cache checks;
- build output checks;
- leak warning fixture tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Users get honest warnings about plaintext risks while unlocked.

### v0.80.0 - Recipient Slot Model

Goal: support recipient-based key wrapping metadata.

Deliverables:

- recipient ID;
- recipient kind;
- wrapped key metadata;
- key-wrap algorithm identifier;
- created-by actor reference;
- signed recipient authorization;
- anonymous recipient slot option where feasible;
- add/remove validation tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- Sagnir can describe who may unlock future encrypted realm keys.

### v0.81.0 - Rekey And Crypto Epochs

Goal: rotate encrypted realm keys without mutating old history in place.

Deliverables:

- crypto epoch transition;
- `saga vault rekey`;
- recipient rewrap plan;
- old-key retention policy;
- crash-safe staged key rotation;
- rollback and interrupted-rotation recovery;
- tests for invalid epoch transitions.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Key rotation is a signed transition model, not an in-place mutation.

### v0.82.0 - Sealed Private Object IDs

Goal: avoid known-plaintext membership leaks in encrypted realms.

Deliverables:

- private keyed object ID metadata;
- ciphertext storage ID metadata;
- deduplication ID policy;
- randomized encryption requirement;
- non-revealing private-ID formatting integration;
- tests that public plaintext hashes are not exposed in sealed private mode.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-crypto`

Exit criteria:

- Encrypted realms can hide whether known plaintext objects are present.

### v0.83.0 - Compartment Encryption Scaffold

Goal: prepare path, world, and projection-level encryption boundaries.

Deliverables:

- compartment ID;
- compartment key metadata;
- `saga vault compartment create`;
- `saga vault protect`;
- `saga vault unprotect`;
- partial unlock metadata;
- compartment policy tests.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can represent different access boundaries inside one encrypted realm.

### v0.84.0 - Hybrid Post-Quantum Readiness Scaffold

Goal: prepare recipient wrapping and signatures for reviewed hybrid algorithms.

Deliverables:

- hybrid key-wrap metadata;
- post-quantum algorithm registry placeholders;
- algorithm admission document;
- crypto provider review checklist;
- tests that unknown or unadmitted algorithms fail closed.

Verification:

- `cargo test -p sagnir-crypto`
- `scripts/checks.sh`

Exit criteria:

- Sagnir is ready to admit hybrid classical plus post-quantum providers without
  changing object formats.

### v0.85.0 - Private Metadata And Padded Storage

Goal: reduce information leakage from encrypted filesystem layout and transfer
shape.

Deliverables:

- two-layer public commitment and encrypted semantic ledger model;
- encrypted paths, worlds, actors, facts, graph edges, policies, and proofs;
- fixed-size or bucketed record classes;
- epoch batching and pack compaction;
- configurable object-count and transfer-size padding;
- metadata leakage inventory and profile-specific defaults;
- tests that public summaries do not expose redacted plaintext fields.

Verification:

- `cargo test -p sagnir-vault`
- encrypted metadata fixture review.

Exit criteria:

- Locked or blind-storage views expose only documented minimal commitments and
  availability metadata.
- Public proof summaries are commitments or deliberate predicates, not
  plaintext reports with fields removed.

### v0.86.0 - Selective Disclosure Proofs

Goal: disclose only policy or evidence claims required by a recipient.

Deliverables:

- Merkle multiproof disclosure format;
- signed disclosed claims;
- hidden-witness commitment model;
- scope and audience binding;
- replay and claim-substitution tests;
- documented admission rule that zero-knowledge proofs are added only for
  predicates that genuinely require hidden witnesses.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-crypto`

Exit criteria:

- A peer can verify selected evidence or policy inputs without receiving the
  unrelated encrypted ledger.

## Phase 10: Bundles And Sync

### v0.87.0 - Pack File Format

Goal: store multiple immutable objects in a bounded pack.

Deliverables:

- pack header;
- object table;
- object body offsets;
- pack footer;
- pack manifest hash;
- total compressed and expanded byte limits;
- per-object size and reference-count limits;
- compression and delta format admission;
- maximum decompression ratio and delta-chain depth;
- compartment-local delta base rule;
- authenticated random-access page integration for encrypted packs;
- malformed pack tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`

Exit criteria:

- Pack readers verify bounds before trusting offsets or object counts.
- Pack readers reject decompression bombs, deep delta chains, invalid bases, and
  offset arithmetic overflow before expensive materialization.

### v0.88.0 - Bundle Manifest

Goal: describe a proof-carrying offline transfer, including encrypted transfer
metadata.

Deliverables:

- bundle manifest;
- object refs;
- world refs;
- fact refs;
- policy refs;
- encrypted bundle marker;
- visible versus encrypted metadata policy;
- resource estimate metadata;
- compressed and expanded byte estimates;
- ancestry depth, reference fanout, concurrent-head, and proof-complexity
  estimates;
- minimum verification mode metadata;
- recommended verification profile metadata;
- manifest validation tests.

Verification:

- `cargo test -p sagnir-sync`

Exit criteria:

- Sagnir can describe what a bundle claims before loading bundle bodies.
- Sagnir can estimate whether local verification settings are sufficient before
  import or materialization.

### v0.89.0 - Bundle Create And Verify

Goal: create and verify offline bundles before import or decrypt.

Deliverables:

- `saga bundle create`;
- `saga bundle verify`;
- `saga bundle create --encrypted`;
- recipient-targeted bundle metadata;
- context-bound bundle signature footer;
- missing object detection;
- deduplication before expensive proof or signature work;
- cancellation and resumable verification checkpoints;
- malicious bundle and fork-bomb tests;
- verification-budget preflight tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-cli`

Exit criteria:

- A bundle can be verified before import, and encrypted bundle metadata is
  checked before decrypt.
- Bundle verification reports when local budgets cannot satisfy the bundle's
  minimum verification mode.

### v0.90.0 - Bundle Import

Goal: import verified bundles safely, including encrypted bundles.

Deliverables:

- `saga bundle import`;
- object deduplication;
- fact deduplication;
- lazy quarantine without worktree materialization;
- per-import byte, object, ancestry, fanout, and time budgets;
- cancellation and resumable import;
- decrypt-before-import policy;
- world alias import policy;
- resource-budget comparison before trust;
- refusal when bundle policy requires stronger verification than local config;
- quarantine-on-policy-failure behavior.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-store`

Exit criteria:

- Import cannot overwrite local world aliases without explicit policy.
- Import can place data in quarantine for inspection without trusting or
  materializing it.
- Budget refusal leaves no partially trusted alias, index, or worktree state.

### v0.91.0 - Sync Negotiation

Goal: exchange local and remote heads before transfer.

Deliverables:

- remote head request;
- compact causal-context or Merkle-clock exchange;
- missing object response;
- missing fact response;
- protocol version negotiation;
- encrypted realm mode negotiation;
- remote resource estimate exchange;
- minimum verification mode negotiation;
- replay rejection metadata;
- private set reconciliation option for private realms;
- equivocation evidence exchange without recursive fork expansion.

Verification:

- `cargo test -p sagnir-sync`

Exit criteria:

- Sync can determine the smallest required bundle for a remote.
- Sync can determine whether local verification budgets satisfy remote trust
  requirements before transfer.

### v0.92.0 - Sync Transfer

Goal: transfer proof-carrying bundles to a remote endpoint.

Deliverables:

- `saga sync`;
- `saga clone`;
- `saga clone --no-worktree`;
- transport-independent framed protocol;
- chunk acknowledgement and resume tokens;
- per-peer quotas and backpressure;
- transfer cancellation;
- accepted response;
- denied response;
- quarantined response;
- local-budget-insufficient response;
- blind remote response;
- split-trust remote response;
- local sync result fact;
- protocol tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-cli`

Exit criteria:

- Local work can sync without requiring a hosted product, including encrypted
  blind-storage workflows.
- Clone and sync do not silently downgrade verification; they either satisfy
  remote requirements, quarantine fetched state, or refuse trust/materialization.

### v0.93.0 - Sparse Materialization And Partial Clone

Goal: let large or compartmentalized realms fetch and materialize only admitted
state.

Deliverables:

- sparse path and compartment selection;
- state-only, receipt-only, and no-worktree clone modes;
- promised-object and missing-body representation;
- proof boundaries for omitted subtrees;
- on-demand object fetch;
- policy refusal when full materialization is required;
- sparse update and missing-object tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Omitted state is explicit and cryptographically committed, never confused
  with verified absence.

### v0.94.0 - Native Transport Adapters

Goal: carry the same Sagnir protocol over practical decentralized transports.

Deliverables:

- removable-file/bundle adapter;
- SSH or stdin/stdout adapter;
- QUIC adapter;
- transport authentication and endpoint identity binding;
- transport-independent transcript tests;
- replay, truncation, reordering, disconnect, and resume tests.

Verification:

- `cargo test -p sagnir-sync`
- local transport integration tests.

Exit criteria:

- Protocol meaning does not change with transport.
- Local-first exchange remains possible without a hosted service.

### v0.95.0 - Git Import And Export Bridge

Goal: ease adoption without making Git the native storage or trust model.

Deliverables:

- bounded Git object and reference reader;
- import mapping from commits and branches to Sagnir source states and worlds;
- explicit provenance and information-loss report;
- export of representable source history;
- refusal or disclosure for Sagnir proofs, policies, worlds, encryption, and
  facts that Git cannot represent;
- adversarial repository and round-trip fixtures.

Verification:

- `cargo test -p sagnir-sync`
- bridge fixture suite.

Exit criteria:

- Users can migrate source history while the CLI clearly identifies which
  Sagnir semantics cannot survive export.
- Git is an interoperability boundary, not Sagnir's backend.

### v0.96.0 - Private Anti-Entropy And Discovery

Goal: reconcile encrypted peers while limiting head, graph, and access-pattern
leakage.

Deliverables:

- private set reconciliation for heads and fact roots;
- padded request and response buckets;
- optional LAN discovery with explicit enablement;
- optional high-security cover-traffic interface;
- request-correlation and peer-quota controls;
- metadata-leakage tests and traffic-shape fixtures.

Verification:

- `cargo test -p sagnir-sync`
- privacy fixture review.

Exit criteria:

- Private peers can discover missing state without announcing raw semantic
  heads to an untrusted blind store.

### v0.97.0 - Minimal Daemon

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

## Phase 11: Hardening And Portability

### v0.98.0 - Verifiable Archive Pack Concept

Goal: keep a future path for disk-space relief without making deletion part of
the early trust model.

Deliverables:

- `.saga/archival/` planning document;
- compressed archive pack concept;
- archive manifest concept;
- archive receipt and root commitment concept;
- rehydrate/restore concept;
- receipt-only downstream clone concept;
- policy notes for disabling archival in regulated realms.

Verification:

- documentation review;
- threat-model update.

Exit criteria:

- Sagnir has a documented future path to compress cold history while retaining
  verifiable receipts.
- Archive receipts cannot be treated as proof that missing archive bodies are
  available or valid.

### v0.99.0 - Malicious Corpus

Goal: make hostile input testing part of normal development.

Deliverables:

- canonical codec corpus;
- object corpus;
- WAL corpus;
- pack corpus;
- bundle corpus;
- encrypted envelope corpus;
- proof and sync-message corpus;
- decompression and delta-chain bomb corpus;
- fork-bomb and causal-fanout corpus;
- regression tests for every accepted corpus case.

Verification:

- `cargo test --workspace`

Exit criteria:

- Known malicious bytes stay rejected across releases.

### v0.100.0 - Expanded Fuzzing

Goal: expand fuzz and model testing beyond the parser scaffolds added earlier.

Deliverables:

- fuzz target workspace;
- codec fuzz target;
- every canonical object-body fuzz target;
- WAL and recovery-state fuzz targets;
- pack and encrypted-envelope fuzz targets;
- bundle parser fuzz target;
- proof and sync-message fuzz targets;
- bounded decompression and delta-chain fuzz targets;
- documentation for running fuzz targets.

Verification:

- parser unit tests;
- documented optional fuzz command.

Exit criteria:

- New parsers have a standard place to add fuzz coverage.

### v0.101.0 - Formal State Models

Goal: model the state machines whose failures could lose, fork, or falsely
admit source state.

Deliverables:

- TLA+/PlusCal or equivalent WAL recovery model;
- alias CAS and concurrent-head model;
- promotion and multi-parent transition model;
- partition reconciliation and checkpoint model;
- checked invariants for atomicity, no lost heads, no stale admission, and
  eventual convergence under documented assumptions;
- model execution instructions and CI smoke bounds.

Verification:

- bounded model-check command;
- model invariant review.

Exit criteria:

- Counterexamples for stale CAS, partial commit, lost divergence, and replay are
  represented in executable models rather than prose alone.

### v0.102.0 - Crash And Concurrency Assurance

Goal: exercise local mutation behavior at every admitted interruption and race
boundary.

Deliverables:

- crash-consistency fault injection at every write, rename, file sync, and
  directory sync;
- state-machine property tests for recovery;
- loom or equivalent tests for writers, proof caches, and alias updates;
- process and thread race tests;
- stale-handle and namespace-replacement fixtures;
- deterministic failure reproduction seeds.

Verification:

- crash fault-injection suite;
- concurrency model test suite.

Exit criteria:

- No injected interruption produces a trusted alias without complete immutable
  bodies or a cache result from the wrong generation.

### v0.103.0 - Partition And Adversarial Network Tests

Goal: validate convergence and refusal behavior under hostile distributed
ordering.

Deliverables:

- reorder, replay, duplication, delay, truncation, and disconnect tests;
- concurrent partitioned world advancement;
- equivocation and conflicting sequence tests;
- delayed evidence and stale policy epoch tests;
- fork-bomb, deep ancestry, and high-fanout simulations;
- quota, cancellation, resume, and quarantine assertions.

Verification:

- deterministic network simulation suite;
- sync state-machine property tests.

Exit criteria:

- Partitions preserve divergent heads and later converge through explicit
  transitions.
- Hostile peers cannot force unbounded recursive expansion or partial trust.

### v0.104.0 - Differential Vectors And Performance Budgets

Goal: prove interoperability and set measurable scale expectations before 1.0.

Deliverables:

- independent canonical-codec reference implementation;
- differential canonical bytes and object-ID tests;
- cryptographic known-answer and malformed-vector suites;
- benchmarks for cold/warm status and one-file changes in million-file realms;
- encrypted random-read and proof-cache reuse benchmarks;
- full-world verification and hostile-bundle rejection benchmarks;
- p50/p95/p99 latency, memory, I/O amplification, and ciphertext-expansion
  budgets;
- CI regression thresholds where stable.

Verification:

- differential test suite;
- benchmark runner and recorded baseline artifact.

Exit criteria:

- Canonical interoperability does not depend on one implementation.
- Sagnir publishes explicit resource budgets and detects material regressions
  in its critical local and hostile-input paths.

### v0.105.0 - Cross-Platform Build Gate

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

### v0.106.0 - Rootless Podman Gate

Goal: make `saga` usable from a rootless container.

Deliverables:

- rootless Podman build;
- rootless Podman run;
- release base image digest pinning;
- CLI smoke test;
- non-root user in image;
- container documentation.

Verification:

- `scripts/podman_smoke.sh`

Exit criteria:

- A user can run the CLI in rootless Podman.
- Release images do not use mutable base image tags.

### v0.107.0 - Release Evidence

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

### v0.108.0 - 1.0 Release Candidate Gate

Goal: freeze the 1.0 feature set and reject incomplete production behavior.

Deliverables:

- 1.0 release gate script;
- all required commands covered by tests;
- canonical and cryptographic vectors pass independently;
- formal models complete within admitted bounds;
- crash, concurrency, partition, and hostile-network suites pass;
- documented p50/p95/p99 resource budgets meet release thresholds;
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
- normative canonical formats and independent vectors;
- computed object hashes and body-derived references;
- authenticated WAL, signed event DAG, and rollback/equivocation evidence;
- world and change workflow;
- convergent multi-head worlds and deterministic multi-parent merges;
- seal and amend;
- status and diff;
- byte-preserving cross-platform paths and root-bound materialization;
- context-bound signatures, key lifecycle, and anti-replay;
- target-bound proof artifacts and compound policy admission;
- non-destructive protected promotion;
- operation undo;
- local facts;
- event log and deterministic fact compiler;
- auditable explanations;
- why, explain, trace, and impact;
- bounded context packs;
- `saga ask` scaffold over deterministic facts;
- encrypted local realms;
- lock and unlock;
- vault status and leak scanning;
- recipient metadata and rekeying;
- encrypted indexes, authenticated pages, private IDs, and metadata protection;
- selective disclosure;
- bundles;
- encrypted bundles;
- bounded quarantine, sparse materialization, and partial clone;
- resumable transport-independent sync;
- removable-file, SSH/stdin, and QUIC transports;
- blind or split-trust encrypted sync mode;
- Git import/export interoperability without using Git as native storage;
- optional daemon;
- formal, crash, concurrency, partition, fuzz, and performance assurance;
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

## Phase 12: Post-1.0 Differentiating Capabilities

These releases extend the native Sagnir model after the first production-ready
CLI. They are not prerequisites for calling v1.0.0 serious local-first
source-state infrastructure.

### v1.1.0 - Build Attestations And Artifact Provenance

Goal: bind reproducible build outputs to exact admitted source state.

Deliverables:

- build attestation object;
- toolchain, dependency, environment, command, and input commitments;
- artifact digest and state-root binding;
- reproducibility comparison;
- `saga build record` and `saga artifact verify`;
- forged, incomplete, and mismatched attestation tests.

Verification:

- `cargo test -p sagnir-fact`
- artifact provenance integration suite.

Exit criteria:

- A produced artifact can carry verifiable provenance back to its exact source,
  policy, and build evidence.

### v1.2.0 - Capability-Scoped Automation And Agent Worlds

Goal: let automation operate without receiving ambient realm authority.

Deliverables:

- capability object with realm, world, path, action, and expiry scope;
- agent world lifecycle;
- delegated signing constraints;
- human acceptance requirement hooks;
- revocation and replay handling;
- overreach, confused-deputy, and stale-capability tests.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-world`

Exit criteria:

- Automation can propose or produce bounded state without gaining permission to
  promote, decrypt, or modify unrelated realm state.

### v1.3.0 - Committed Semantic Merge Plugins

Goal: support domain-aware merges without making plugin execution invisible or
authoritative by default.

Deliverables:

- plugin identity, version, input, output, and configuration commitments;
- deterministic execution profile;
- sandbox and resource policy;
- plugin attestation and human acceptance hooks;
- fallback to explicit ordinary conflicts;
- nondeterminism, substitution, timeout, and over-budget tests.

Verification:

- semantic merge fixture suite;
- policy and sandbox integration tests.

Exit criteria:

- Semantic merge output is reproducible or explicitly attested and never hides
  the plugin, inputs, version, or unresolved uncertainty.

### v1.4.0 - Role-Hiding Reviewer Credentials

Goal: prove authorized review roles while minimizing reviewer identity
disclosure.

Deliverables:

- anonymous or pseudonymous reviewer credential model;
- role and epoch binding;
- uniqueness and duplicate-review prevention;
- revocation evidence;
- selective disclosure integration;
- correlation, replay, and threshold-manipulation tests.

Verification:

- `cargo test -p sagnir-crypto`
- private review policy fixtures.

Exit criteria:

- Policy can verify an admitted reviewer threshold without exposing more
  identity information than the configured realm requires.

### v1.5.0 - Hidden-Witness Policy Proofs

Goal: admit zero-knowledge proofs only for policy predicates that require hidden
witnesses.

Deliverables:

- reviewed proof-system admission process;
- versioned circuit or statement registry;
- policy root, epoch, realm, target, and scope binding;
- proof size and verification-cost limits;
- trusted-setup disclosure where applicable;
- malformed, replay, downgrade, and resource-exhaustion vectors.

Verification:

- `cargo test -p sagnir-proof`
- independent cryptographic review and vectors.

Exit criteria:

- Hidden policy claims can be verified without turning a generic zero-knowledge
  proof into an unscoped authorization token.

### v1.6.0 - Verifiable Archival And Availability

Goal: move cold history out of the hot store without erasing its existence or
lying about availability.

Deliverables:

- compressed archive pack and manifest;
- immutable archive receipt and root commitment;
- archive rehydrate and verify commands;
- receipt-only downstream mode;
- availability proof or explicit unavailable state;
- regulated-policy retention controls;
- missing, substituted, truncated, and unavailable archive tests.

Verification:

- `cargo test -p sagnir-store`
- archival recovery integration suite.

Exit criteria:

- Cold history can be compacted and restored while missing archive bodies are
  detectable and never represented as locally available proof.

### v1.7.0 - Cross-Domain Causal Queries

Goal: query causal relationships across source, dependencies, keys, builds,
models, artifacts, and deployments.

Deliverables:

- typed cross-domain subject registry;
- bounded local query planner;
- confidence, uncertainty, and missing-evidence output;
- policy-aware redaction;
- provenance-preserving result citations;
- high-fanout and mixed-trust query tests.

Verification:

- `cargo test -p sagnir-query`
- causal query fixture suite.

Exit criteria:

- Sagnir can answer impact and provenance questions across the delivery chain
  without converting inferred relationships into authoritative facts.

### v1.8.0 - Privacy Transport Extensions

Goal: add optional privacy-oriented decentralized transports without changing
the Sagnir protocol.

Deliverables:

- Tor adapter;
- evaluated libp2p adapter or documented rejection rationale;
- endpoint unlinkability and authentication policy;
- padded transfer integration;
- abuse quotas and denial-of-service controls;
- replay, correlation, downgrade, and disconnect tests.

Verification:

- transport integration suite;
- privacy threat-model review.

Exit criteria:

- High-privacy peers can use admitted transports while preserving the same
  signed protocol transcripts, verification budgets, and trust semantics.
