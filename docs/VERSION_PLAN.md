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

Every unreleased milestone also inherits continuous portability checks:

- canonical vectors run on Linux, Windows, and MacOS CI where hosted runners
  are available;
- format and policy results must be byte-identical across platforms;
- filesystem behavior has platform fixtures from the milestone that introduces
  it, not only at the final portability phase;
- BSD, Android, and iOS constraints are reviewed whenever a format or
  filesystem assumption changes;
- the late cross-platform release remains the comprehensive 1.0 matrix and
  unsupported-behavior audit, not the first portability check.

## Roadmap Authority And Review Cadence

Until the architecture documents are synchronized, this version plan is the
normative implementation order. Supporting documents must not be used to weaken
or bypass a later trust-boundary requirement recorded here.

The numbered versions are public milestone candidates and inherit the release
and pentest rules below. Model checks, format reviews, fault-injection stages,
and internal implementation checkpoints inside one version are not separate
tags. The maintainer chooses pentest scope proportional to the change and may
expand it at phase boundaries, cryptographic trust boundaries, distributed-
state boundaries, and release candidates without creating another release
ceremony or approval role.

Sagnir keeps granular stops because small security changes are easier to review.
The maintainer may combine only adjacent milestones before implementation starts
and only when the combined scope remains independently testable. Completed or
released versions are never renumbered or retroactively combined.

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

Policy has two separate roles:

- local acceptance policy controls what one device is willing to ingest, trust,
  decrypt, retain, or materialize;
- canonical realm and world policy is signed, history-bound state that defines
  whether a transition is valid for participants in that realm.

Local policy may be stricter than canonical policy. It cannot make an invalid
realm transition valid.

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

Private realms use three non-interchangeable identity layers:

- immutable semantic commitments identify canonical plaintext objects and
  signed graph roots inside the protected semantic ledger;
- epoch-specific private lookup locators allow keyed membership lookup without
  becoming the identity signed by historical transitions;
- ciphertext storage IDs identify randomized encrypted records or packs and may
  change during re-encryption, repacking, or relocation.

Canonical object references and signatures bind immutable semantic commitments.
Private locators and ciphertext IDs are storage and privacy projections linked
to those commitments by authenticated encrypted indexes and scoped translation
evidence.
Rotating a private-locator key never rewrites or impersonates the originally
signed semantic history.
Private logical-index leaves contain no mutable ciphertext placement. A
dedicated index-commitment key calculates logical roots, while signed
checkpointed manifests grant root authority. Re-encryption, repacking, receipt
renewal, and relocation change placement roots without changing semantic or
locator logical roots.

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

Authoritative ordering is causal and checkpoint-based. Wall-clock timestamps
are advisory unless an admitted timestamp authority attests to them. Local
monotonic clocks may control process-local session expiry, but wall-clock input,
environment state, filesystem state, and network responses cannot influence a
deterministic canonical policy decision.

## Security Boundary Documentation Rule

Every milestone that introduces or changes a parser, trust transition,
cryptographic purpose, key lifecycle, persistence format, privilege boundary,
network message, disclosure surface, recovery path, or release-signing boundary
must update the corresponding `docs/threat-model.md` section and
`docs/security-controls.md` entry before its release stop.

The update must identify:

- protected assets and security properties;
- trusted and untrusted inputs, actors, processes, devices, and services;
- misuse, compromise, rollback, replay, partition, resource-exhaustion, and
  metadata-leakage cases relevant to the boundary;
- preventive, detective, recovery, and residual-risk controls;
- exact tests, models, vectors, benchmarks, or operational evidence that
  exercise those controls.

Later milestones inherit this rule automatically. A feature is not release-ready
when its implementation exists but its boundary-specific threat model and
control-map evidence are stale.

## Assurance At First Admission Rule

Security testing belongs to the release that first admits a parser, state
machine, concurrent authority path, or cryptographic construction. The late
hardening phase expands scale and cross-system composition; it is never the
first time a security-critical format is fuzzed or a durability protocol is
modelled.

Each applicable milestone must add and run its own proportionate assurance:

- parser and format releases add canonical/malformed vectors, corpus-backed
  fuzz smoke, cumulative resource-budget tests, and permanent regression seeds;
- arithmetic and bounded-state releases add Kani or an equivalent bounded proof
  for no-panic, no-overflow, state-space, and impossible-state properties where
  the tool is suitable;
- concurrent state machines add Loom or equivalent schedule exploration around
  leases, publication, cache replacement, and consume/cancel races;
- durability and distributed protocols add a reproducible TLA+/PlusCal or
  equivalent model before implementation, with exact bounds, assumptions,
  explored-state counts, tool digest, and completion status;
- canonical algorithms add differential tests against a separately implemented
  reference before their output becomes authoritative;
- cryptographic formats/providers add standards vectors, malformed vectors,
  fault injection, side-channel regression evidence, and secret-copy lifetime
  review at admission;
- every crash, counterexample, differential disagreement, or fuzz finding is
  minimized where practical and retained as a permanent regression fixture.

CI uses short deterministic or time-bounded smoke profiles. Pentest and release
gates use longer profiles declared by the milestone. Nightly or scheduled
campaigns may add depth without becoming a separate human approval process.

## Solo Maintainer Release Workflow

Sagnir uses one simple release loop. Codex performs implementation, local
verification, documentation, release-note, report, commit, and tag work. The
maintainer performs the pentest, reports GitHub CI results, and explicitly
authorizes tagging and pushing.

### 1. Implementation Stop

Codex completes the version, tests it, updates the required documentation and
release notes, and commits the implementation as needed. When the version
criteria and local gates are complete, work stops before tagging and the
maintainer is told:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

No tag or push occurs at this point.

### 2. Pentest Loop

The maintainer runs the pentest and, when findings exist, writes them to root
`PENTEST.md`.

There are only two outcomes:

1. **Pentest is green.** Codex removes any scratch `PENTEST.md`, writes or
   updates `security/pentest/<tag>.md` with `Status: PASS`, records the exact
   tested commit, updates release documentation where needed, runs the release
   gate, and commits the release record. The project then waits for GitHub CI.
2. **Pentest reports issues.** Codex reads `PENTEST.md`, fixes the issues,
   updates tests and documentation, removes the scratch file when addressed,
   runs the local gates, and commits the fixes. Codex then calls for a retest.
   This repeats until the maintainer reports a green pentest, after which
   outcome 1 applies.

Root `PENTEST.md` is temporary scratch input and must never be committed. The
release metadata validator fails while it exists.

### 3. GitHub CI Loop

After the green pentest report and release record are committed, there are only
two outcomes:

1. **GitHub CI is green.** The maintainer reports green status and explicitly
   instructs Codex to tag and push. Codex verifies the final release gate,
   creates the tag, and pushes the requested commit and tag.
2. **GitHub CI reports an issue.** The maintainer provides the failure. Codex
   fixes it, records the CI finding and resolution in the permanent pentest
   report, runs the local gates, and commits the correction. The project waits
   for GitHub CI again. Codex handles any exact-commit report bookkeeping needed
   by the validator; the maintainer does not perform an extra release ceremony.

No tag is created while pentest or GitHub CI is unresolved. No external
reviewer, committee, approval board, or separate sign-off is required by the
ordinary per-version release workflow.

## Mandatory Per-Version Stop

Every version section below inherits this tag stop:

```text
vX.Y.Z implementation stop reached. Run pentest for this exact commit.
```

Every version also inherits this pentest task:

- run the local gates for the exact commit;
- let the maintainer review security-sensitive changes in scope;
- write temporary findings to root `PENTEST.md`;
- have Codex fix and document every release-blocking finding;
- repeat maintainer retesting until the pentest is green;
- remove root `PENTEST.md`;
- create or update `security/pentest/<tag>.md` with `Status: PASS`, exact
  `Commit:`, non-blank `Tester:`, non-blank `Scope:`, and `Date: YYYY-MM-DD`;
- commit the green report and wait for GitHub CI;
- when CI fails, have Codex fix it, append the finding and resolution to the
  report, commit, and wait for CI again;
- tag and push only after GitHub CI is green and the maintainer explicitly
  requests it.

## Phase 0: Foundation

### v0.1.0 - Repository Foundation

Goal: initialize the serious Rust workspace and policy baseline.

Deliverables:

- Rust stable `1.97.1` pinned.
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
- v0.12.2 corpus replay and bounded fuzz smoke for every object-body decoder.

Exit criteria:

- Tests can report whether a supplied small graph is structurally complete or
  identify exact missing supplied references.
- This release does not claim body-derived or cryptographic completeness; those
  trust properties begin at v0.13.0 and v0.14.0.
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
graph execution and derived scheduling remain assigned to v0.34.0, while
profile-to-policy enforcement remains assigned to v0.36.0 and compound
protected-transition admission remains assigned to v0.70.0.

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

### v0.10.1 - Shared Audited Store Platform Boundary

Goal: move durable filesystem authority out of the CLI so every frontend uses
one audited store implementation before another platform backend, WAL, or
object persistence expands.

Deliverables:

- split canonical store metadata and transaction formats from operating-system
  filesystem execution, using focused crates such as `sagnir-store-format` and
  `sagnir-store-fs` when the final dependency review confirms those names;
- move Unix secure initialization and metadata persistence from private CLI
  modules into the shared platform store boundary without weakening retained-
  handle, no-follow, ownership, attachment, hard-link, sync, and atomic-publication
  checks;
- correct the released v0.10.0 initialization boundary by synchronizing the
  already-open parent root directory after `.saga/` and its required initial
  state are durably created; syncing only `.saga/` is insufficient evidence
  that the parent directory entry survives a crash;
- record this parent-sync correction as new v0.10.1 behavior rather than
  rewriting the guarantees of the already tagged v0.10.0 release;
- `sagnir-store` owns platform-neutral store plans, verified canonical bytes,
  immutable IDs, transaction intents, and durability results; it does not
  depend on CLI output, policy evaluation, daemon transport, or UI types;
- `sagnir-store-fs` depends only on the store/format layers and narrow platform
  APIs; frontends depend inward on the shared store and cannot provide alternate
  durable write implementations;
- `saga`, `sagad`, tests, and migration tools use the same audited store
  boundary for every authoritative read, write, lock, sync, and publication
  operation;
- key agents are not general store clients: they receive only narrow
  capability-scoped sign, unwrap, secret-store, or status requests and cannot
  read arbitrary repository state, write WAL frames, publish objects, update
  aliases, or advance checkpoints;
- authoritative persistence methods consume typed capabilities such as
  `VerifiedCanonicalObject`, `AdmittedTransaction`, and retained store handles,
  not generic byte buffers plus booleans or frontend-created validation flags;
- path/display conversion remains at frontend edges and cannot replace an open
  root/store handle with a newly resolved path;
- compile-time dependency-direction checks reject cycles from store crates back
  to CLI, daemon, policy, sync, or presentation crates;
- parity fixtures run identical initialization, metadata, lock, recovery, and
  refusal cases through library, CLI, and daemon callers;
- no duplicate private secure-store implementation remains reachable after the
  migration.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`
- `cargo test -p sagad`
- workspace dependency-direction validator;
- Unix and hosted Windows platform-boundary fixture suites.

Exit criteria:

- CLI and daemon persistence cannot diverge because both use one shared audited
  platform store implementation.
- Store format code remains portable and deterministic, while unsafe platform
  assumptions are isolated behind native retained-handle implementations.
- No authoritative store method accepts a display path or unverified raw bytes
  as a substitute for an admitted handle and canonical object identity.
- New `.saga/` creation does not report success until its parent directory entry
  is durable under the admitted v0.10.1 Unix profile.
- A key agent cannot become a second repository writer or bypass typed ingest.

### v0.10.2 - Native Windows Store Initialization

Goal: implement stateful Windows initialization once against the shared
v0.10.1 store platform boundary without path-based race windows.

Deliverables:

- native Windows implementation of the shared `sagnir-store-fs` contract;
- retained Windows root and `.saga/` directory handles;
- component-by-component reparse-point refusal;
- handle-relative directory and file operations;
- Windows file-ID verification around metadata commits;
- owner and access-control admission policy;
- parent-root and nested-directory durability publication using admitted
  Windows flush semantics and explicit residual limitations;
- hosted Windows junction, symlink, namespace replacement, parent-publication,
  and temp-file race tests;
- CLI and daemon parity fixtures use the shared store API rather than separate
  Windows implementations;
- removal of the non-Unix `Unsupported` stop only after those tests pass.

Verification:

- hosted `windows-latest` `cargo test -p sagnir-store`;
- hosted `windows-latest` `cargo test -p sagnir-cli`;
- hosted `windows-latest` `cargo test -p sagad`;
- maintainer Windows filesystem pentest.

Exit criteria:

- Windows `saga init` cannot be redirected through a junction, symlink,
  reparse point, namespace replacement, or temporary-file substitution.
- Windows initialization has the same fail-closed attachment and typed shared-
  store boundary as Unix, with platform-specific durability limits stated
  honestly.
- Windows support introduces no private frontend persistence implementation.

### v0.11.0 - Architecture And Store Compatibility Contracts

Goal: make the architecture documents agree and define durable repository
compatibility before new canonical formats are implemented.

Deliverables:

- update `docs/IMPLEMENTATION_PLAN.md` for real signature verification before
  protected promotion;
- replace reusable symlink-proof language in `docs/architecture.md`;
- document multi-head aliases and merge transitions in `docs/world-model.md`;
- document target-bound proof artifacts and soundness scopes in
  `docs/proof-model.md`;
- document private reconciliation instead of unconditional raw-head disclosure
  in `docs/protocol.md`;
- add a documentation consistency validator for normative trust statements;
- declare which document is authoritative when future drafts conflict;
- durable `.saga/FORMAT` compatibility contract;
- minimum reader and writer versions in durable format metadata;
- read-only compatibility behavior for stores newer than the local writer but
  still within the admitted reader range;
- unknown-critical-format and unsupported-writer refusal;
- transactional, crash-safe upgrade protocol with preflight, dry-run, backup,
  staged writes, commit marker, and recovery;
- explicit downgrade and old-writer refusal after upgrade;
- storage migrations may rewrite indexes, object placement, encryption
  envelopes, ciphertext IDs, and pack layout;
- storage migrations must not silently reserialize canonical object bodies,
  change immutable semantic commitments, rewrite signed references, or
  invalidate historical signatures;
- every canonical object version reachable from admitted history remains
  decodable and verifiable;
- dropping an old decoder is prohibited while any admitted reachable history
  depends on it;
- semantic changes create new signed transition objects and roots rather than
  mutating old canonical objects;
- migration provenance distinguishes a storage-layout-preserving migration
  from a semantic transition, recording old format, new format, tool version,
  input roots, resulting storage commitments, and any separately signed
  semantic result roots;
- mixed-version peer and repository fixtures;
- golden repositories retained in CI for every released durable format.

Verification:

- `scripts/check_doc_links.sh`
- documentation consistency validator;
- repository-format compatibility fixture suite;
- migration crash-consistency suite;
- `scripts/checks.sh`

Exit criteria:

- No supporting architecture document describes a weaker trust boundary than
  this version plan.
- CI rejects reintroduction of the known contradictory patterns.
- A newer tool can explain, dry-run, and transactionally upgrade every admitted
  released durable format.
- An older or incompatible tool fails closed without partially rewriting the
  repository.
- Every durable migration is attributable and its root transition is
  reproducible from retained golden fixtures.
- A storage-layout migration preserves canonical semantic roots and historical
  signature validity.
- Any semantic root change is represented and authorized as a new native Sagnir
  transition, never disguised as a repository-format upgrade.

Inherited rule:

- every later milestone that changes durable bytes, `.saga/FORMAT`, object or
  envelope versions, index formats, pack formats, or migration metadata must
  satisfy the v0.11.0 compatibility fixtures, decoder-retention rules,
  provenance requirements, and crash-safe migration contract.

### v0.12.0 - Normative Canonical Protocol Specification

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

### v0.12.1 - Unified Decode Budgets And Atomic Encoding

Goal: make every nested decoder share one cumulative hostile-input budget and
make every encoder failure leave caller output unchanged.

Deliverables:

- mandatory mutable `DecodeBudget` or equivalent capability containing at
  least remaining input bytes, decoded items, nesting depth, references,
  objects, allocations, and abstract work units;
- one budget is threaded through every nested scalar, list, object, event,
  proof, pack, bundle, WAL, encrypted-envelope, decompression, and extension
  decoder instead of resetting independent per-field maxima;
- budget constructors accept only policy-admitted limits and enforce hard
  implementation ceilings so callers cannot request effectively unbounded
  parsing;
- checked addition, multiplication, integer conversion, offset, and range
  arithmetic before any slice, allocation, hash, decompression, loop, or
  reference expansion;
- debit-before-work semantics: failed allocation, malformed nested input, and
  early return cannot restore budget in a way that permits repeated work
  amplification;
- explicit distinction between consumed wire bytes, retained decoded bytes,
  allocation bytes, cryptographic work, decompressed bytes, and graph/query
  expansion units;
- deterministic budget-exhaustion errors that identify the exhausted class
  without reflecting secret or attacker-controlled diagnostics;
- non-cloneable `BudgetLease`/child-reservation model for parallel work: a
  parent atomically reserves bounded bytes, items, allocations, references,
  objects, work, and optional concurrency slots before spawning a worker;
- child leases cannot clone, mint, borrow from siblings, exceed the reservation,
  or return more capacity than they received; checked aggregation prevents
  counter wrap or double return;
- cancellation, worker panic, join failure, retry, and partial completion have
  deterministic accounting; policy specifies whether unused reservations are
  returned, burned for the operation, or reconciled only by the parent;
- logical allocation budgets remain distinct from measured resident memory,
  address-space, file-descriptor, thread, and process limits; satisfying one is
  never presented as proving the others;
- encoder `encoded_len`/preflight with checked arithmetic, or encode-to-bounded-
  scratch then commit, so insufficient output never writes a prefix into the
  caller's destination;
- success writes exactly one canonical byte sequence; failure preserves every
  byte of the destination and returns the exact required capacity when it can
  be reported safely;
- streaming encoders write only into a transaction-scoped uncommitted sink;
  partial frames/chunks cannot receive an authoritative digest, signature,
  index entry, manifest reference, or publication name before final length,
  canonical bytes, and commit marker are validated;
- streaming cancellation/error leaves an explicitly incomplete sink that is
  discarded or recovered only by its format state machine, never hashable or
  signable as a complete record;
- nested exhaustion, cumulative-small-field amplification, depth, conversion,
  multiplication, decompression, hashing, parallel reservation/return races,
  cancellation/panic accounting, streaming short-write/failure, and stale-
  output-reuse tests;
- bounded proof over admitted scalar/list arithmetic, no-panic behavior, budget
  monotonicity, and failure atomicity.

Verification:

- `cargo test -p sagnir-codec`
- `cargo test -p sagnir-object`
- Kani or equivalent bounded codec proof suite;
- independent budget-accounting and atomic-writer vectors.

Exit criteria:

- No production decoder can process nested untrusted data without sharing the
  caller's admitted cumulative budget.
- A sequence of individually valid lengths cannot exceed aggregate bytes,
  items, allocations, references, objects, depth, decompression, or work limits.
- Encoder failure is observationally atomic for caller-owned output buffers.
- Parallel workers cannot duplicate parent capacity, and incomplete streaming
  bytes cannot become authoritative content.

### v0.12.2 - Continuous Parser Fuzz And Regression Baseline

Goal: execute parser fuzz evidence continuously before canonical body formats
multiply, without making nightly tooling part of the stable workspace.

Deliverables:

- deterministic corpus-backed smoke runner for every parser and format fuzz
  target admitted through v0.12.1;
- short bounded smoke profile on pull requests and ordinary CI;
- longer sanitizer-enabled pentest/release profile with declared time, corpus,
  memory, and worker budgets;
- optional scheduled campaign profile for deeper exploration, with failures
  treated as ordinary findings rather than a separate approval ceremony;
- stable root workspace remains latest stable Rust; `cargo fuzz`/sanitizer
  nightly tooling remains isolated in the standalone fuzz workspace and pinned
  independently;
- seed corpus includes canonical values, malformed/non-canonical values, exact
  limits, one-past limits, truncated prefixes, duplicate fields, trailing data,
  nested-budget exhaustion, and prior release findings;
- crash artifacts are minimized, classified, assigned a stable corpus name,
  and committed as permanent regression inputs before closure;
- CI rejects a parser fuzz target that only compiles but has no smoke corpus,
  bounded execution command, or regression replay test;
- fuzz command, engine/toolchain digest, target list, corpus digest, duration,
  executions, crashes, timeouts, and out-of-memory outcomes are recorded in
  release evidence.

Verification:

- standalone fuzz workspace build and dependency policy;
- deterministic corpus replay command;
- bounded CI fuzz smoke command;
- release-profile sanitizer smoke on supported hosts.

Exit criteria:

- Every admitted parser has executed hostile-input evidence before a later
  release depends on its format.
- Fuzz crashes become reproducible tests and cannot disappear with temporary
  runner artifacts.
- Stable production builds do not acquire a nightly Rust dependency.

### v0.12.3 - Deterministic Authoritative Work Accounting

Goal: ensure machine speed, thread scheduling, and wall-clock duration cannot
change an authoritative verification, derivation, projection, or policy result.

Deliverables:

- versioned deterministic work-cost tables define integer debits for every
  authoritative parser, graph, proof, fact, query, projection, policy, bundle,
  and verification operation before those operations can publish a root;
- canonical success, truncation, exhaustion, and continuation boundaries depend
  only on admitted inputs, canonical traversal/order, versioned cost rules, and
  original integer ceilings, never elapsed time, CPU speed, worker count, load,
  local clock, or completion order;
- local monotonic deadlines may cancel resource use but produce only
  `Incomplete`/`Cancelled`, publish no authoritative fact/projection/proof root,
  and cannot be interpreted as deterministic budget exhaustion;
- parallel workers reserve non-cloneable v0.12.1 child leases against a fixed
  canonical partition of work; sequential and every admitted parallel schedule
  consume the same logical units and reach byte-identical results/status;
- unused child reservations are either burned until the operation ends or
  returned only for non-authoritative process accounting; their return timing
  cannot fund additional work in the same canonical decision;
- cancellation, panic, retry, work stealing, worker loss, and different
  parallelism settings cannot change the canonical consumed-work transcript or
  turn an otherwise incomplete operation into an admitted one;
- performance deadlines, resident-memory limits, thread limits, and other
  operational controls are recorded separately from canonical logical budgets;
- result transcript binds cost-table version, original ceilings, exact consumed
  counters by class, deterministic traversal/partition digest, completion
  status, and any cancellation reason;
- sequential/parallel differential fixtures, randomized schedule exploration,
  slow/fast host simulation, deadline injection, unused-lease return races,
  panic/retry, and exact-bound/one-past-bound tests.

Verification:

- `cargo test -p sagnir-codec`
- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-proof`
- Loom or equivalent budget-lease schedule exploration;
- independent work-accounting vectors.

Exit criteria:

- Two conforming implementations given the same inputs, cost-table version,
  and ceilings produce the same authoritative status and consumed counters.
- A local deadline can stop work but can never authorize, truncate, or publish
  canonical state.
- Returning unused parallel reservations cannot make acceptance depend on
  thread scheduling.

### v0.12.4 - Work-Cost Table Authority And Lifecycle

Goal: prevent hostile inputs or stale configuration from selecting an unknown,
downgraded, or underpriced authoritative work-cost table.

Deliverables:

- each deterministic work-cost table has a canonical version, domain, evaluator/
  verifier compatibility range, operation-class coverage, cost-rule digest,
  activation state, and independent differential vectors;
- before live canonical realm policy exists, each admitted protocol/format and
  evaluator/verifier version embeds one protocol-fixed bootstrap table; local
  configuration and untrusted input cannot select another canonical table;
- after live governance/policy activation, signed canonical realm state plus the
  admitted evaluator/verifier version selects the protocol table at an explicit
  activation frontier/checkpoint; replicas evaluating the same realm state use
  the same table;
- local admission ceilings are a separate stricter layer: insufficient local
  logical resources return `LocalResourcesInsufficient`/refusal without changing
  the canonical table, consumed-work semantics, or shared result;
- operational CPU, resident-memory, thread, I/O, and monotonic-deadline limits
  are a third layer and may cancel with `Incomplete` only; they never create a
  different authoritative root or canonical exhaustion result;
- an object, bundle, proof, remote peer, resume checkpoint, or other untrusted
  input may declare a requirement but cannot choose or downgrade the protocol
  table used for new admission;
- new authoritative evaluation fails closed on unknown, unsupported,
  ambiguous, retired-for-new-admission, mismatched-domain, or downgraded table
  versions;
- changing any debit, operation classification, traversal rule, rounding rule,
  or aggregate behavior requires a new table version and old/new differential
  vectors; existing version bytes are immutable;
- governance/policy can retire an underpriced table for new admission and bind
  its replacement at a signed activation frontier without reinterpreting
  historical transcripts that committed the old table and original logical
  result;
- historical verification uses the committed evaluator/table semantics when
  policy admits that historical context, but current operational CPU, memory,
  deadline, and abuse controls may cancel it with `Incomplete` rather than
  changing its canonical result or pretending it exhausted logical work;
- migration rules state when old authoritative roots remain historical-only,
  require recomputation under a new evaluator/table for current promotion, or
  are refused because the old implementation/vector set is unavailable;
- resume requires the exact originally bound table and cannot switch to a
  cheaper/newer table mid-operation; a retired table cannot begin new work by
  replaying an old input;
- table registry updates are signed/policy-bound where canonical admission
  depends on them and record activation, retirement, replacement, rationale,
  affected evaluator versions, and effective frontier/checkpoint;
- transition from the protocol-fixed bootstrap table to realm-selected tables
  is one signed non-retroactive activation transition; peers that lack the
  activated table refuse/return unknown instead of evaluating with a local
  substitute;
- attacker-selected version, unknown/downgraded table, underpricing retirement,
  local-table divergence, local-ceiling exhaustion, operational cancellation,
  bootstrap-to-realm activation, historical replay, cross-domain substitution,
  mid-resume switch, unavailable old implementation, and old/new differential
  tests.

Verification:

- `cargo test -p sagnir-codec`
- `cargo test -p sagnir-policy`
- independent work-cost registry and old/new differential vectors;
- table activation/retirement/migration state-machine tests.

Exit criteria:

- New authoritative work uses only the protocol-fixed bootstrap table or the
  table selected by signed canonical realm state and evaluator/verifier
  compatibility, never local configuration or attacker-controlled bytes.
- Local ceilings and operational limits may refuse/cancel work but cannot select
  canonical semantics or create a different shared result.
- Retiring an underpriced table protects new admission without rewriting what
  a historical transcript meant.
- Operational cancellation of historical work yields only incomplete evidence,
  not a different canonical logical decision.

### v0.13.0 - Canonical Object Body Decoders

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

### v0.14.0 - Hash Computation And Derived References

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

### v0.14.1 - Body-Derived Typed Graph Admission

Goal: replace the bounded scaffold's caller-described graph with an efficient
typed admission engine derived exclusively from verified canonical bodies.

Deliverables:

- graph admission input accepts hash-verified canonical bodies and immutable
  IDs, never authoritative caller-supplied edges;
- every edge is independently extracted by the canonical object-kind decoder
  and checked against a versioned source-kind, edge-role, target-kind, cardinality,
  and optionality schema matrix;
- supplied manifests and edge inventories are optimization hints only; omitted,
  extra, reordered, or mistyped hints cannot change the derived graph or produce
  `Complete`;
- duplicate references and duplicate canonical edges are rejected or
  canonicalized according to the object schema before traversal, with no
  duplicate-work amplification or alternate graph representation;
- immutable sorted ID table plus bounded compact adjacency offsets, or an
  equivalently reviewed structure, replaces repeated linear target scans;
- checked cumulative graph budget covers objects, unique edges, references,
  parents, unresolved promises, bytes, depth, hash work, and traversal work;
- graph class is explicit: content, event, world-transition, and other
  authority graphs enforce their admitted DAG/topological rules, while
  dependency and impact graphs may contain cycles and deterministically collapse
  strongly connected components;
- cycle diagnostics distinguish malformed authority cycles from valid
  dependency SCCs and remain bounded on adversarial input;
- independently implemented small-graph reference verifier derives edges from
  the same canonical bodies without sharing the optimized adjacency engine;
- differential property tests compare acceptance, derived edges, roots,
  topological order/SCC partition, diagnostics, and budget outcomes across
  optimized and reference implementations;
- malformed-body, omitted-edge, duplicate-edge, wrong-role, wrong-kind,
  high-fanout, long-chain, dense-SCC, hash-collision simulation, and budget-edge
  corpus cases.

Verification:

- `cargo test -p sagnir-object`
- object graph corpus-backed fuzz smoke;
- optimized/reference differential graph suite;
- Kani or equivalent bounded no-panic/overflow checks for adjacency arithmetic.

Exit criteria:

- `Complete` means canonical bodies, hashes, every derived typed reference,
  closure, and the graph-class rule were independently verified.
- A caller cannot hide an edge, invent an edge, exploit duplicate edges, or
  choose a graph policy inappropriate for the object layer.
- Repository-scale admission uses bounded indexed lookup rather than quadratic
  scans while remaining equivalent to the independent reference verifier.

### v0.15.0 - Identifier Privacy And Realm Scoping

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

### v0.16.0 - Hash And Algorithm Migration

Goal: define algorithm change and collision response before durable identities
become difficult to migrate.

Deliverables:

- algorithm downgrade resistance;
- algorithm retirement epochs;
- dual-commit transition format;
- cross-algorithm identity semantics;
- collision-suspicion and confirmed-collision response procedure;
- mixed-algorithm graph and signature admission rules;
- downgrade, stripping, ambiguous-identity, and partial-migration tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-crypto`
- migration vector validator.

Exit criteria:

- Peers cannot silently replace a required algorithm with an older admitted
  algorithm.
- A realm can migrate identities without treating two algorithms or digest
  domains as interchangeable.

### v0.17.0 - Authenticated Commitment Semantics

Goal: separate the required semantics for map membership/absence from append-
only consistency before selecting the uniquely represented map algorithm.

Deliverables:

- authenticated-map semantic contract for object, alias, fact, policy, and key
  roots, including membership, absence, update, map-kind separation, key/value
  schema binding, and bounded proof requirements;
- append-only event commitment such as an admitted MMR-style structure;
- canonical node, empty-root, append, and consistency encodings for the selected
  append-only structure;
- map algorithm, node shape, normalization, root, and proof bytes remain
  deliberately unadmitted until v0.17.1 selects a history-independent
  construction and v0.17.2 fixes its persistent page format;
- independent append-only known-answer vectors and map semantic fixtures;
- malformed proof, ambiguous key, and extension-confusion tests.

Verification:

- `cargo test -p sagnir-proof`
- commitment vector validator.

Exit criteria:

- Append-only root/proof operations have one normative structure and encoding;
  map semantics are fixed but no map bytes are yet authoritative.
- Membership and append-only consistency are not overloaded onto one structure
  when their required security properties differ.

### v0.17.1 - History-Independent Authenticated Map Admission Stop

Goal: select and justify one uniquely represented authenticated dictionary
before a persistent map format or incremental writer becomes authoritative.

Deliverables:

- algorithm decision record selects exactly one reviewed construction, such as
  a canonical radix/Merkle trie, fixed-depth sparse Merkle tree, content-defined
  deterministic pages over a sorted key stream, or a fully normalized tree whose
  incremental algorithm provably reproduces canonical bulk-build shape;
- an ordinary B-tree/B+ tree is explicitly inadmissible merely because each
  local split or merge choice is deterministic; admission requires global
  unique representation under every insertion, deletion, and rebuild history;
- normative algorithm fixes canonical key/value bytes, key-bit or separator
  interpretation, empty root, leaf/internal domains, boundary selection,
  underfull-node handling, neighboring-node rebalance, split/merge/normalization,
  duplicate rejection, and root construction;
- executable permutation invariant is
  `root(incremental(permutation(entries))) == root(canonical_bulk_build(entries))`;
- executable rebuild invariant is
  `root(after_arbitrary_updates_and_deletes) == root(rebuild(current_entries))`;
- delete/reinsert, repeated update, split/merge, compaction, and equivalent final
  key/value sets erase operation history from the logical shape/root while
  retaining only independently declared physical-placement history;
- adversarial key distributions cannot force unbounded depth, tiny-page chains,
  repeated whole-map normalization, or update/proof amplification beyond exact
  admitted page/byte/hash/work bounds;
- proof or executable model covers canonical boundary selection, normalization
  termination, unique fixed point, update locality, worst-case amplification,
  and agreement between incremental and bulk algorithms;
- exhaustive small-map permutation and operation-history enumeration includes
  every insertion order, deletion order, delete/reinsert path, and boundary case;
- differential randomized histories compare production/reference incremental
  roots with an independently implemented canonical full rebuild after every
  operation and retain failing seeds/counterexamples;
- algorithm/version transition rules bind old/new roots and never reinterpret
  historical roots under a replacement construction.

Verification:

- authenticated-map algorithm model/proof review;
- exhaustive bounded permutation/history suite;
- independent prototype versus canonical bulk-builder differential vectors.

Exit criteria:

- One construction has evidence for unique representation and bounded
  amplification; deterministic local tree edits alone do not satisfy admission.
- No durable page format, authoritative root, or production incremental writer
  proceeds until the algorithm decision and counterexample suite are complete.

### v0.17.2 - Bounded Persistent Authenticated Map Pages

Goal: implement the v0.17.1 admitted history-independent dictionary as a
canonical bounded persistent representation before long-lived roots depend on
unbounded in-memory serialization.

Deliverables:

- immutable bounded-fanout page format with independent leaf/internal domain
  labels, exact algorithm/page version, level, entry/child count, canonical key
  range/boundary evidence, payload byte length, and child/page commitments;
- every internal child descriptor authenticates canonical child key range or
  boundary, subtree entry count, subtree reachable-page count, child level/
  height, and child commitment; checked summary arithmetic rejects overflow and
  parent/root totals are recomputed from child summaries rather than trusted;
- page boundaries, underfull handling, neighboring-page rebalance, normalization,
  and deterministic bulk build are exactly the admitted v0.17.1 construction;
  equivalent key/value sets have one logical shape/root independent of operation
  history, scheduling, placement, allocator, or host architecture;
- authenticated root descriptor binds map kind/version, algorithm and commitment
  suites, logical entry count, tree height, reachable page count, root page
  commitment, and declared key/value schemas;
- protocol limits bound page bytes, entries, fanout, height, key/value lengths,
  proof nodes, update/normalization pages, and cumulative decode/hash/work
  budgets before allocation or cryptographic work;
- point lookup, inclusion, absence, and bounded single-key update proofs are
  `O(height)` plus normalization pages; range and page-set completeness proofs
  are output-sensitive `O(height + emitted_entries_or_pages)` and never described
  as logarithmic in result size;
- separate protocol maxima bind requested/emitted entries, emitted pages, proof
  nodes/bytes, subtree-summary work, and cumulative decode/hash/work so a valid
  large result cannot bypass ordinary proof budgets;
- incremental updates authenticate old path summaries and recompute affected
  subtree/global entry and reachable-page counts in work proportional to height
  plus changed/normalized pages, without traversing unaffected subtrees;
- deterministic full rebuild streams bounded sorted entries and independently
  recomputes root/count/height/page-count without loading or trusting the whole
  map at once;
- copy-on-write update result is an immutable bounded page set plus old/new root
  descriptors and replaced-page commitments suitable for atomic publication by
  a later WAL transaction; no mutable page can become authoritative in place;
- file offsets, mmap pointers, cache identity, filesystem order, and mutable
  index metadata are never authenticated authority; reads copy or integrity-bind
  bounded page bytes before parsing and verify page commitment plus parent/root
  path before use;
- malformed offset/range/boundary, page/level substitution, omitted or duplicate
  page, duplicate/out-of-order key, separator mismatch, non-canonical underfull
  page/rebalance, child key-range/entry-count/page-count/height mismatch,
  summary overflow, root count mismatch, deep tree, fanout violation, cyclic
  reference, stale root, update omission, oversized result, and cumulative-
  budget exhaustion tests;
- every incremental mutation is differentially checked against canonical full
  rebuild in exhaustive small maps and randomized long histories, including
  delete/reinsert and adversarial boundary/key distributions;
- independent vectors cover empty, singleton, canonical boundary changes,
  minimum/maximum fanout, multi-page normalization/update, full rebuild, and
  equivalent insertion/deletion histories.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-store`
- independent authenticated-map page/root/proof vector validator;
- bounded rebuild, permutation, history, and malformed-page corpus.

Exit criteria:

- No admitted authenticated map requires one unbounded record or whole-map
  allocation to load, update, prove, or rebuild it.
- Incremental roots equal canonical rebuild roots after every operation, while
  missing, substituted, mutable, or non-canonical pages fail before authority.
- Root counts and output-sensitive proof limits are independently verifiable from
  authenticated subtree summaries without whole-map traversal.

### v0.18.0 - Realm Genesis And Governance Schema

Goal: define the canonical origin and governance schema before live
cryptographic enforcement exists.

Deliverables:

- canonical realm-genesis object;
- realm ID derived from or cryptographically bound to the genesis commitment;
- initial administrative and trust keys;
- initial policy, key-registry, and crypto roots;
- actor and device enrollment authority;
- membership, role, threshold administration, ownership transfer, and emergency
  recovery statement schemas;
- explicit marker that this release defines canonical bytes and invariants but
  does not yet execute signed governance;
- genesis substitution, duplicate genesis, split-governance, and recovery
  takeover tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-policy`
- governance state-machine model.

Exit criteria:

- Every authoritative actor, key, policy, and governance transition traces to
  one admitted genesis object.
- Conflicting genesis or governance roots are detected rather than merged by
  local preference.
- Live governance enforcement remains unavailable until v0.25.0.

### v0.19.0 - First-Contact Trust Bootstrap

Goal: let a new device authenticate the intended realm genesis without trusting
a human-facing name or transport endpoint.

Deliverables:

- pinned genesis fingerprint mode;
- canonical signed invitation or capability schema carrying the exact
  realm/genesis commitment;
- invitation issuer, intended audience or device, requested role and
  capabilities, realm, governance epoch, nonce, issuance bound, expiry or
  no-expiry policy, and one-time or bounded-use semantics;
- invitation issuance, secure display/export, acceptance preview, explicit
  user confirmation, consumption, rejection, expiry, revocation, supersession,
  and audit states;
- governance-authorized revocation and supersession records that remain
  checkable after an invitation has been presented offline;
- deterministic handling for duplicate acceptance, concurrent acceptance,
  issuer key rotation, stale governance epochs, lost invitations, and
  interrupted enrollment;
- invitation acceptance never grants authority beyond the exact signed role,
  capability, compartment, device, and expiry scope;
- out-of-band fingerprint verification workflow;
- explicitly warned trust-on-first-use mode;
- persisted first-contact trust record and later mismatch detection;
- no silent trust inheritance from DNS, URL, transport certificate, remote
  endpoint identity, or realm name;
- fingerprint substitution, invitation replay, duplicate or concurrent
  acceptance, revoked or superseded invitation, stale issuer, endpoint
  impersonation, same-name realm, TOFU mismatch, and downgrade tests;
- explicit dependency that cryptographic invitation verification activates only
  after providers and live governance are admitted at v0.25.0.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-sync`
- trust-bootstrap golden-output tests.

Exit criteria:

- A clone or invitation identifies a realm by its genesis commitment.
- Users can distinguish pinned, invitation-backed, out-of-band, and warned TOFU
  trust states before any remote state is admitted.
- Invitation parsing, preview, and rejection are safe before activation, while
  issuance, authoritative acceptance, revocation, and enrollment become live
  only through admitted governance.
- Before v0.25.0, invitation bytes are schema fixtures and cannot establish live
  authority merely by parsing successfully.

### v0.20.0 - Actor, Device, And Replica Identity

Goal: make every authoritative signer and causal writer explicit before
worlds, events, or checkpoints depend on identity.

Deliverables:

- actor ID;
- device and replica ID;
- key registry metadata;
- public key metadata storage;
- actor-to-device authorization records;
- enrollment and retirement statement schemas;
- `saga actor init`;
- identity collision, substitution, unauthorized enrollment, and malformed
  registry tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Events, worlds, checkpoints, revisions, and facts can identify the exact
  authorized actor and device that produced them.

### v0.21.0 - Signature Envelope And Set Admission

Goal: bound signature metadata and principal counting before cryptographic use.

Deliverables:

- signature envelope parser;
- syntactic algorithm and suite allow-list with bounded identifiers; exact
  interoperable suite semantics are admitted by v0.21.1 before use;
- signer key ID and key epoch;
- per-signature and total signature-set byte bounds;
- roles, principal identities, and threshold metadata bounds;
- duplicate-device and duplicate-principal rejection;
- key proof-of-possession hook where required by the admitted suite;
- unknown, oversized, duplicate, rogue-key, and conflicting signature tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- Thresholds count independent admitted principals or roles, not merely distinct
  device keys controlled by one principal.

### v0.21.1 - Interoperable Cryptographic Suite Identity

Goal: define exact provider-independent suite identity before canonical signed
transcripts, production providers, encrypted envelopes, or policy epochs depend
on algorithm metadata.

Deliverables:

- canonical suite identity contains only interoperable cryptographic semantics:
  algorithm family, exact parameter set, standards/specification revision,
  encoding, pure/prehash or operation mode, and hybrid component/combiner rules;
- signature, KEM, KDF, AEAD, password-KDF, key-wrap, hash, and hybrid suites use
  separate typed namespaces and cannot be substituted because numeric IDs or
  byte lengths overlap;
- provider assurance is a separate v0.21.2 evidence record binding
  implementation, source/build digest, backend, platform, CPU features,
  acceleration/fallback path, side-channel profile, and validation evidence;
- admission context is separate realm state binding suite ID, crypto epoch,
  registry version, policy root, lifecycle status, errata decision, and
  downgrade/retirement rules;
- provider migration that preserves suite semantics does not change the meaning
  or canonical bytes of historical signatures, ciphertext, or transcripts;
- two conforming providers for one suite produce interoperable protocol objects
  even when their build, backend, platform, or assurance profiles differ;
- policy may require a provider assurance class in addition to a suite, but the
  provider class is not encoded into the suite ID;
- signature/ciphertext/encapsulation length is validated after selecting the
  suite and can never select an algorithm or parameter set by itself;
- unknown suite, wrong typed namespace, generic algorithm ambiguity, provider-
  encoded suite identity, admission-epoch substitution, errata downgrade,
  length inference, and provider-migration compatibility vectors;
- canonical registry and transcript test vectors independent of any one crypto
  library.

Verification:

- `cargo test -p sagnir-crypto`
- independent suite-registry vector validator;
- two-provider interoperability fixture using test providers;
- provider/admission/suite separation compile-fail and malformed tests.

Exit criteria:

- Every protocol object names exact cryptographic semantics without embedding
  a particular implementation, machine, backend, or realm policy epoch.
- Provider assurance and realm admission can change without silently changing
  the suite that historical bytes mean.
- No verifier infers suite identity from payload length or a generic family
  label.

### v0.21.2 - Provider Assurance Evidence Trust Contract

Goal: define who can vouch for cryptographic-provider assurance and what that
evidence means before production provider admission or policy use.

Deliverables:

- canonical provider-assurance evidence binds exact suite, implementation and
  source/build/artifact digests, dependency/toolchain identity, backend,
  platform/architecture, CPU features, acceleration/fallback path, claimed
  side-channel properties, test/validation evidence, scope, and limitations;
- issuer identity, issuer role/type, trust root, signature suite/key epoch,
  issuance authority, evidence provenance, and verification transcript are
  explicit and separate from the provider and crypto-suite identities;
- assurance categories distinguish unsigned/self-declared metadata, provider-
  signed claims, maintainer local audit evidence, independent third-party
  validation, standards/laboratory validation, and hardware-rooted attestation;
- a provider signature proves claim origin only and never upgrades its own
  statement to independent assurance;
- realm/local policy states which issuer roots, issuer categories, scopes,
  platforms, evidence classes, and minimum diversity satisfy each assurance
  requirement;
- issuance, causal/checkpoint or v0.28.0 expiry/freshness, revocation,
  compromise, supersession, revalidation, and historical-verification rules;
- evidence replacement or provider migration does not alter the exact v0.21.1
  suite identity or meaning of historical protocol bytes;
- unknown issuer, self-claim-as-independent, build/platform mismatch, stale,
  revoked, superseded, cross-suite, cross-backend, downgraded-evidence, and
  hardware-attestation substitution tests;
- machine-readable CLI/report output states claim source, assurance category,
  scope, freshness, trust basis, and residual limitations without collapsing
  them into one `trusted` boolean.

Verification:

- `cargo test -p sagnir-crypto`
- provider-assurance canonical vectors;
- issuer/revocation/supersession state-machine tests;
- self-claim and cross-context policy-bypass fixtures.

Exit criteria:

- Provider assurance is accepted only under an explicit trust root, evidence
  category, scope, platform, freshness, and policy requirement.
- Self-declared or provider-signed metadata cannot count as independent review
  merely because it carries a valid signature.
- Provider assurance can evolve without changing interoperable suite identity.

### v0.22.0 - Canonical Signed Statement Transcripts

Goal: bind authoritative signatures to complete realm and transition context.

Deliverables:

- versioned transcript format per statement and action type;
- realm, target, parent/frontier, base root, and result root immutable semantic
  commitments;
- explicit prohibition on signing epoch-specific private lookup locators or
  ciphertext storage IDs as canonical object identity;
- source and target world commitments where applicable;
- policy root and epoch, exact v0.21.1 crypto suite and realm crypto epoch,
  signer key and epoch;
- per-key sequence number, verification scope, and algorithm-transition state;
- domain separation and cross-statement replay tests;
- transcript known-answer vectors.

Verification:

- `cargo test -p sagnir-crypto`
- transcript vector validator.

Exit criteria:

- A valid signature for one realm, action, epoch, world, scope, frontier, or
  algorithm state cannot authorize another.
- Re-encryption, repacking, ciphertext relocation, or private-locator rotation
  cannot invalidate a signature over unchanged semantic history.

### v0.23.0 - Signing And Verification Providers

Goal: admit actual cryptographic signing and verification before signed state is
used by storage or worlds.

Deliverables:

- reviewed provider abstraction;
- one mandatory production signature suite identified and admitted under the
  v0.21.1 contract;
- production OS-CSPRNG abstraction for key generation, salts, randomized
  signatures, nonces, and other cryptographic randomness;
- fail-closed entropy errors with no timestamp, process ID, counter, hardware
  clock, or ordinary pseudorandom fallback;
- fork and reseed policy;
- VM-clone, suspend/resume, early-boot, and entropy-source-unavailable behavior;
- deterministic test RNG isolated behind test-only APIs and absent from
  production feature graphs;
- continuous source health checks only where the admitted operating-system or
  hardware interface defines them, without software entropy estimation claims;
- key generation/import boundary;
- non-cloneable opaque provider handles are the initial API for private signing
  keys, secret signing nonces/scalars, and provider session state; raw private
  key bytes are not returned to identity, WAL, world, policy, or CLI code;
- provider operations accept canonical transcript capabilities and return only
  public signatures/verification results; private signing keys and signing
  nonces/scalars never become caller-readable bytes;
- provider key generation/import and signing are test/vector/internal interfaces
  only in this release; no realm/genesis or post-genesis authoritative caller
  can invoke them until v0.23.3 admits durable reservations, v0.23.4 admits the
  exact sealed capability class, and v0.23.5 admits genesis reconciliation;
- opaque handles do not implement `Clone`, `Copy`, `Debug`, display,
  serialization, unrestricted byte extraction, or ambient conversion to a
  generic buffer;
- caller-owned key-import bytes have an explicit consume/cleanup contract;
  cleaning the provider's copy is never represented as cleaning the caller's
  original memory;
- persistent key-handle provider, key identity, suite, key epoch, and lifecycle
  binding prevents cross-provider/suite use or use after retirement; per-action
  authorization is carried by a separate consumed operation capability rather
  than permanently narrowing the reusable key handle;
- signing and verification over canonical transcripts only;
- known-answer and established adversarial vectors;
- secret redaction and zeroization through the admitted sanitization crate;
- a machine-readable side-channel assurance profile for every production
  provider/backend combination, naming the secret-bearing operations covered by
  an admitted constant-time implementation guarantee and the platforms,
  compiler settings, hardware features, and fallback paths for which that
  guarantee applies;
- review of secret-dependent branches, memory lookups, allocation behavior,
  early returns, parsing failures, and error mapping in provider code and its
  direct cryptographic dependencies;
- hardware-accelerated and software-fallback paths must implement equivalent
  transcript, validation, refusal, and response-shape semantics; an unavailable
  accelerator cannot silently select an unreviewed fallback;
- invalid-key, malformed-signature, decapsulation/ciphertext failure, wrong-key,
  and authentication-failure paths use bounded, policy-declared response shapes
  and avoid distinguishable secret-dependent diagnostics;
- timing-distribution tests with recorded host, CPU feature, compiler, sample,
  warmup, noise-control, and statistical-method metadata where such testing can
  detect regressions; passing a timing test is supporting evidence rather than
  proof of constant-time execution;
- secret-copy and lifetime inventory covering stack, heap, temporary buffers,
  FFI/provider boundaries, registers where observable, panic/error paths, and
  serialization scratch space, with zeroization review that records compiler
  optimization and unavailable whole-system erasure limitations;
- explicit side-channel exclusions and residual-risk statements for cache and
  shared-microarchitecture attacks, page-fault observation, speculative
  execution, physical probing, compromised firmware, and a privileged or local
  root adversary unless a later provider profile specifically admits those
  threats;
- provider failure, unsupported-suite, key-substitution, unavailable entropy,
  repeated randomness, forked state, VM-clone, early-boot, and compromised
  randomness fault-injection tests;
- secret-dependent timing, invalid-input response-shape, accelerator/fallback
  equivalence, and zeroization/copy-lifetime regression tests;
- compile-fail tests for handle cloning, serialization, raw extraction,
  cross-provider/suite reuse, and use after session/key retirement.

Verification:

- `cargo test -p sagnir-crypto`
- provider known-answer vector suite;
- `cargo deny check`

Exit criteria:

- Sagnir providers create and verify real context-bound test/vector signatures
  without relying on envelope shape as evidence of authenticity; authoritative
  production invocation remains unavailable until v0.23.4/v0.23.5.
- Production cryptographic operations that require randomness stop before
  emitting keys, signatures, salts, or state when the admitted entropy source
  fails.
- Deterministic test randomness cannot be selected by a production build or
  runtime configuration.
- Signing authority is founded on opaque provider handles; no later subsystem
  needs a migration from an admitted raw-private-key API.
- Every admitted production provider states its exact constant-time assurance
  boundary and residual side-channel exclusions; Sagnir does not infer a
  whole-system side-channel guarantee from a library name or timing test.

### v0.23.1 - Provider-Only Secrets And Plaintext Declassification

Goal: separate secrets that must remain provider-internal from plaintext that
Sagnir intentionally releases to an authorized consumer.

Deliverables:

- provider-only secret classes include private keys, signing nonces/scalars,
  KEKs, DEKs, KDF outputs, password-verifier intermediates, KEM secret keys and
  shared secrets, wrapping keys, and recovery reconstruction material;
- provider-only secrets have no caller-readable byte API, closure byte view,
  generic-buffer conversion, serialization path, debug/display path, or
  caller-selected output sink;
- sign, verify-secret-dependent state, derive, encapsulate/decapsulate, wrap/
  unwrap, AEAD seal/open, and recovery operations execute inside the provider
  and expose only their declared public result or an explicit declassification
  result; AEAD open release inherits v0.23.2 authenticate-before-release;
- persistent `KeyHandle` binds provider, key identity, exact suite, key epoch,
  lifecycle state, and permitted operation classes without embedding one realm
  action or transcript;
- short-lived sealed non-cloneable operation-capability classes bind realm,
  action, exact canonical transcript/target, available policy/crypto context,
  audience/purpose, provider session, limits, and expiry, and are consumed by
  one provider operation; v0.23.4 defines the non-convertible classes, exclusive
  minting, IPC, crash, and retry semantics;
- decrypted source files, exported recovery packages, and other intentionally
  released plaintext cross a named auditable `PlaintextRelease`/
  declassification boundary with authorized consumer, purpose, byte/time
  bounds, destination policy, cleanup contract, and release event;
- declassified plaintext is honestly treated as copyable after release; Rust
  lifetimes or closure scope limit borrowed-reference lifetime but are never
  claimed to prevent `to_vec`, hashing, logging, IPC, or another explicit copy;
- APIs prefer provider-to-audited-consumer streaming where practical, but the
  threat model records every consumer that can observe and copy plaintext;
- cancellation, panic, consumer rejection, partial output, and cleanup failure
  do not produce an authorized completion result and leave explicit residual-
  exposure evidence where plaintext may have crossed the boundary;
- compile-fail/API-surface tests reject provider-secret extraction and handle/
  capability confusion; adversarial consumers prove declassified plaintext can
  be copied so documentation and controls cannot rely on non-copyability.

Verification:

- `cargo test -p sagnir-crypto`
- provider-secret API and compile-fail suite;
- operation-capability replay/substitution tests;
- plaintext-release fault-injection and consumer audit fixtures.

Exit criteria:

- Provider-only secrets never enter caller-readable memory through an admitted
  production API.
- Reusable key identity and lifecycle are separate from one-use authorization
  for a specific action and transcript.
- Every intentional plaintext release is explicit, bounded, auditable, and
  described as copyable once an authorized consumer receives it.

### v0.23.2 - Authenticate-Before-Plaintext-Release

Goal: ensure no unauthenticated plaintext reaches a declassification consumer,
including large or resumable encrypted records.

Deliverables:

- whole-record AEAD open authenticates the complete ciphertext, associated
  data, suite, key epoch, nonce, and tag before releasing any plaintext byte;
  tag failure produces zero released plaintext;
- providers retain whole-record plaintext in bounded provider-owned or isolated
  transaction scratch until authentication succeeds, then cross the explicit
  v0.23.1 declassification boundary once;
- large/resumable data uses a versioned chunked authenticated-record format
  rather than exposing incremental plaintext from one unfinished whole-object
  AEAD operation;
- associated-data binding alone is not nonce uniqueness; encrypted chunked
  operation remains unavailable until v0.92.2 instantiates exact per-record
  nonce seed/subkey, checked chunk nonce derivation, final-record domain,
  exhaustion, retry, and clone/rollback rules for an admitted AEAD suite;
- every chunk is independently authenticated before release to its declared
  transaction-scoped consumer, and associated data binds format/version,
  bundle/object/encryption-instance identity, manifest or semantic/ciphertext
  root commitment, chunk index, total chunk count, total plaintext/ciphertext
  length, exact suite, key epoch, audience/purpose, and stream operation ID;
- canonical chunk ordering and uniqueness reject reordering, duplication,
  omission, overlap, index wrap, inconsistent totals, cross-object substitution,
  prefix truncation, and suffix omission;
- an authenticated final record commits the ordered chunk commitment root,
  exact chunk count, total lengths, object/bundle identity, and completion;
  earlier authenticated chunks remain non-authoritative staging until this
  final completeness record verifies;
- every pre-final staging consumer is rollbackable, isolated, and side-effect-
  free: it cannot invoke external IPC/hooks, materialize a worktree, publish an
  object/index/fact, update policy, release user-visible output, or perform any
  irreversible action before final completeness authentication;
- resumable decrypt/open checkpoints occur only after a complete authenticated
  chunk and bind the next index, running public commitment/transcript state,
  exact format/verifier/suite/key epoch, operation ID, manifest/root, and v0.12.4
  cost table/remaining budget;
- raw provider AEAD state, incremental MAC state containing secrets, nonces
  awaiting use, provider-only keys, DEKs, KEM shared secrets, or plaintext
  scratch are never serialized into a resume checkpoint;
- cancellation or crash discards unauthenticated current-chunk plaintext and
  can resume only by reauthenticating from an admitted chunk boundary;
- no worktree, trusted object, canonical index, policy fact, or authoritative
  root can consume staged chunks before final completeness authentication and
  typed ingest succeed;
- whole-record bad-tag zero-release, late-tag failure, chunk reordering,
  duplicate, omission, truncation, inconsistent total, final-record absence,
  resume-boundary substitution, serialized-secret-state, crash, and
  cancellation tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-sync`
- whole-record and chunked authenticated-release vectors;
- provider/plaintext-release fault-injection suite;
- resumable chunk state-machine model.

Exit criteria:

- Whole-record authentication failure releases no plaintext.
- Chunked mode releases only individually authenticated chunks to bounded
  staging, and no staged prefix becomes authoritative without an authenticated
  final completeness record.
- Chunked encryption/decryption is not operational until v0.92.2 proves exact
  nonce/key uniqueness and retry semantics for the selected suite.
- Resume state contains no provider-only key or raw cryptographic operation
  state and cannot skip authentication of the resumed chunk.

### v0.23.3 - Minimal Authority Transaction Substrate

Goal: model, format, and implement the smallest generic crash-safe authority log
before capability reservation or provider-result state depends on persistence.

Deliverables:

- TLA+/PlusCal or equivalent model is completed and has no known counterexample
  within declared bounds before transaction-substrate implementation begins;
- one shared transaction substrate under the audited store boundary is used from
  its first release for provider-operation reservations/results and later
  extended for objects, facts, worlds, aliases, checkpoints, and other WAL record
  kinds; no temporary second authorization journal is introduced;
- minimal machine-readable bootstrap durability profile covers retained-handle
  writer exclusion, write ordering, file data/metadata sync, atomic/no-replace
  publication, directory and parent-directory sync, rename assumptions, and
  explicit unsupported/degraded filesystems; v0.30.0/v0.32.1 later broaden the
  same profile contract rather than defining its semantics retroactively;
- versioned base frame freezes magic, format version, critical record kind,
  payload length, log/store incarnation, transaction/reservation ID, monotonic
  sequence, previous committed root/frame commitment, payload, and commit state;
- authority commitment suite `sagnir-authority-sha3-256-v1` uses FIPS 202
  SHA3-256 with one fixed suite ID; no frame, transaction, state entry, or
  checkpoint can select an algorithm independently or infer it from digest
  length;
- exact canonical length framing, integer widths/endianness, absent/present
  tags, and independent domain labels are frozen before implementation for
  `sagnir:authority-payload:v1`, `sagnir:authority-frame:v1`,
  `sagnir:authority-transaction:v1`, `sagnir:authority-state-entry:v1`,
  `sagnir:authority-state:v1`, and `sagnir:authority-log-checkpoint:v1`;
- `FrameCommitment` hashes suite/format version, store and log incarnation,
  monotonic sequence, transaction ID, critical record kind, exact payload
  length, payload digest, and predecessor frame commitment; the payload digest
  is separately domain-separated and commits to the exact canonical payload;
- transaction commit frames bind the ordered frame commitments, exact frame and
  byte counts, transaction ID, first/final sequence, prior committed transaction
  or log-checkpoint commitment, and resulting logical authority-state root;
- canonical `AuthorityStateRoot` is a versioned composite committing to the
  v0.17.2 active-operation map descriptor, terminal-fence map descriptor, sparse
  exception-map descriptor, authenticated epoch-archive manifest root, exact
  active/terminal/exception/archive counts, and commitment suite; v0.23.3 starts
  with canonical empty terminal/archive roots and v0.23.6 activates movement
  between those components without changing this root schema;
- active-map values bind operation ID, issuer/replica/incarnation and reservation
  sequence, capability class, request digest, status, provider-result commitment
  or explicit absence, and ambiguity/equivocation state; duplicate IDs, non-
  canonical order, unknown status, or omitted terminal evidence fail closed;
- active-map create/update commits v0.17.2 copy-on-write pages and an exact page-
  publication manifest through the same WAL transaction; all new pages and
  required parent-directory entries are durable before the transaction exposes
  the new root, and recovery ignores unreachable prepared pages;
- lookup/update/proof work is logarithmic and bounded; checkpoint, compaction,
  rebuild, migration, and cutover stream independently verified pages under
  cumulative budgets rather than encoding, copying, hashing, or authenticating
  the complete operation map as one record;
- canonical `AuthorityLogCheckpoint` binds store/log incarnation, final
  sequence, record count, physical chain root, `AuthorityStateRoot`, and
  predecessor authority-log checkpoint; physical history and logical operation
  state are distinct commitments and neither can substitute for the other;
- an `AuthorityLogCheckpoint` describes a closed physical frontier ending
  before its own record: final sequence, record count, and chain root cover that
  predecessor frontier, while the checkpoint record commitment becomes the
  authenticated predecessor of the successor segment/record; no checkpoint
  hashes itself;
- CRC-32C parameters/coverage inherit the exact Castagnoli reflected form,
  initial/final XOR, little-endian field, excluded checksum field, no-padding
  rule, and `123456789` vector later reused by v0.31.0;
- admitted base records include transaction begin/commit/abort, operation
  reserve, consume, cancel, ambiguous, provider-result reference, compaction
  checkpoint, and format-extension refusal markers;
- unknown critical record kinds fail closed; later record kinds extend the
  registry without changing the meaning or bytes of admitted base records;
- reservation commit is durable before any capability can be minted or sent to
  a provider/key agent; provider/local result commit is durable before the CLI/
  caller can receive successful completion;
- model covers torn/short/reordered writes, malformed length, CRC/checksum
  failure, sequence exhaustion, duplicate transaction/reservation ID, reserve/
  consume/cancel races, and repeated deterministic recovery;
- provider completion before local result commit, local result commit before
  caller receipt, lost response, orphaned provider result, explicit ambiguous
  state, and exact idempotent retry are modeled without assuming one filesystem
  transaction covers provider state;
- key-agent execution/result journal is a distinct provider-side authority log:
  Sagnir store WAL commits authorization/reservation and observed result
  commitments, while the agent journal commits execution/admission/result;
  reconciliation binds operation ID, request digest, provider/key/session,
  sequence, result commitment/status, and ambiguity evidence;
- physical log compaction creates a predecessor-linked `AuthorityLogCheckpoint`
  and may change segment layout, record count, and physical chain root only while
  preserving the exact logical `AuthorityStateRoot`; retained ambiguous/
  equivocation evidence remains committed and `Consumed`, `Cancelled`, or
  `Ambiguous` IDs can never become reusable;
- logical terminal-state compaction is a distinct v0.23.6 authenticated state
  transition that may change active/fence/exception/archive component roots only
  while preserving exact replay and historical-verification semantics;
- hash-suite transition requires a new critical format and signed transition
  binding old/new suite IDs, complete old/new logical roots, old/new physical
  checkpoints, and effective frontier; old history is never reinterpreted under
  a new algorithm and remains verifiable while reachable;
- known-answer vectors cover every commitment domain and empty/minimum/maximum
  transcript; mock/truncated-digest fixtures exercise collision refusal where
  complete canonical bytes are available and cross-domain substitutions are
  rejected; an undetectable collision against SHA3-256 is explicitly outside
  the suite's security claim and requires governed suite retirement;
- legacy-free extension rule: v0.30.0 extends the model and v0.31.0/v0.32.0 add
  record kinds/commitments/recovery behavior on this same substrate; no pre-WAL-
  to-WAL authority migration or dual-writer state exists;
- clone/snapshot rollback limitations are explicit: disconnected copies may
  each advance locally, comparison yields rollback/fork/equivocation evidence,
  and prevention requires an admitted non-rollback anchor;
- crash injection covers every record write, sync, publication, directory-sync,
  compaction, checkpoint, provider-reconciliation, and recovery boundary;
- model-run manifest records exact source/tool digests, bounds, assumptions,
  profiles, explored states/transitions, completion status, and counterexamples.

Verification:

- bounded authority-log model check before implementation;
- `cargo test -p sagnir-store`
- hardware/software CRC-32C differential vectors;
- independent SHA3-256 authority commitment and domain-separation vectors;
- process-kill and deterministic repeated-recovery suite;
- bounded persistent authority-map page/update/rebuild differential suite;
- provider/store reconciliation state-machine fixtures.

Exit criteria:

- No capability exists before its reservation is durable, and no success is
  returned before the exact local/provider result is durably reconciled.
- Torn data, races, exhaustion, repeated recovery, and compaction cannot mint,
  reuse, lose, or silently change an authority reservation.
- Authority-state admission and rebuild remain bounded as operation count grows;
  no whole-map record, allocation, mmap view, or mutable cache is authoritative.
- CRC detects accidental corruption; cryptographic chain/state commitments
  become tamper evidence only relative to genesis, a later signed checkpoint,
  or an independently retained witness.
- Later WAL work extends one modeled transaction substrate and never chooses
  authority between competing legacy/new logs.

### v0.23.4 - Sealed Operation Capabilities And Crash Idempotency

Goal: make one-use provider authorization real across process/IPC boundaries,
serialization, retries, and crashes rather than relying only on Rust `!Clone`.

Deliverables:

- four sealed non-convertible classes with no generic `OperationCapability`
  supertype that can recover broader authority: `BootstrapCryptoCapability`,
  `UnlockCapability`, `AuthoritativeOperationCapability`, and
  `RecoveryCapability`;
- no class has a public/default/deserialization constructor; only its owning
  typed ceremony/admission state can mint it, and provider APIs require the
  exact class for each operation;
- `BootstrapCryptoCapability` is minted only by the initialization ceremony
  holding retained new-store/genesis authority and permits bounded first realm/
  actor/device key generation or import plus exact genesis signing; it cannot
  unlock, rotate, recover, promote, decrypt ordinary state, or publish any
  post-genesis transition;
- `UnlockCapability` is a distinct dormant schema until encrypted-realm
  activation; its owning unlock ceremony may authorize only passphrase/
  recipient unwrap and bounded compartment/header decryption needed to discover
  protected policy/lifecycle state, never signing, rotation, promotion, or
  authoritative publication;
- `UnlockCapability` binds exact retained store and genesis identities,
  authenticated outer/header commitment, recipient-slot or passphrase-wrapper
  commitment, compartment plus maximum scope, admitted ciphertext ranges or
  encryption-instance IDs, crypto epoch, exact suite, audience/purpose, and
  permitted plaintext result class/destination;
- decrypted policy/lifecycle bytes remain untrusted until AEAD authentication,
  canonical decoding, signatures, checkpoint binding, and lifecycle/revocation
  validation complete; they cannot authorize their own decryption, widen scope,
  change slots/ranges/destination, or mint another capability;
- unlock provider APIs reject arbitrary caller-supplied ciphertext, slot,
  associated data, nonce, range, or output destination and use bounded policy-
  declared response shapes that do not become key-validity, membership,
  recipient-slot, or plaintext oracles;
- `RecoveryCapability` remains dormant until v0.64.0 and is minted only from the
  threshold-governed recovery ceremony for its exact recovery action/scope;
- `AuthoritativeOperationCapability` has no live minting path before v0.70.0;
  v0.70.0 activates minting only by consuming a complete typed compound
  admission result bound to policy, signatures, lifecycle/revocation, target,
  frontier, and transaction intent;
- every capability binds issuer/class and authority context, provider/key handle
  identity, exact suite/key epoch, realm or pre-genesis ceremony, action/
  transcript/target, available policy/crypto/frontier context, audience/purpose,
  original limits, session, sequence, expiry, and allowed result class;
- each capability instance authorizes exactly one provider operation; a
  bootstrap/genesis ceremony mints a sequence of narrowly bound bootstrap-class
  capabilities and one capability can never cover both key provisioning and
  genesis signing;
- shared typed durable reservation primitive allocates a domain-tagged operation
  ID before any result or containing transition is constructed, records exact
  purpose/context, and exposes non-reusable `Reserved`, `Consumed`, `Cancelled`,
  or `Ambiguous` states; later obligation issuance and encryption-instance
  creation reuse this lifecycle with their own domain-separated ID formats;
- provider operation ID is the admitted hash over domain
  `sagnir:provider-operation:v1` and exact length-framed `(provider_id,
  realm_id_or_pregenesis_ceremony_id, authority_kind_and_id,
  actor_or_device_id_if_available, replica_id_and_incarnation,
  durable_reservation_sequence, purpose, independent_256_bit_random_nonce)`;
  hash suite/version and every optional/discriminated field are explicit;
- `actor_or_device_id_if_available` and `replica_id_and_incarnation` use explicit
  canonical tagged unions (`Absent` or `Present(value)`); pre-genesis operation
  IDs never encode absence as an empty/default ID or ambiguous zero bytes;
- provider operation ID is durably reserved before provider/key-agent admission
  under retained store/ceremony authority; bootstrap reservation does not
  require a signature from a key that is being generated, and genesis later
  binds the reservation and result;
- every reservation/result is a v0.23.3 transaction-substrate record; no
  dedicated pre-WAL journal, migration state, dual writer, or authority-source
  selection exists, and no capability is returned until reservation durability;
- reservation sequence allocation is checked and fail-closed before exhaustion;
  OS-CSPRNG failure stops before reservation, and restored/cloned state inherits
  the explicit rollback/fork limitations below;
- uniqueness is probabilistic under the admitted hash collision/second-preimage
  assumptions and independent 256-bit randomness plus deterministic reservation
  separation; Sagnir does not claim mathematical global uniqueness;
- an existing identical ID/reservation is idempotent only for byte-identical
  class, purpose, authority, request, and context; any different reservation or
  request under that ID is a collision/equivocation security conflict and
  cannot overwrite prior state;
- in-process use consumes the capability atomically into one provider request;
  failure before provider admission returns an explicit unused/refused result,
  while post-admission uncertainty never reconstructs a fresh capability;
- crossing key-agent IPC consumes the local capability into an authenticated,
  replay-resistant request envelope with agent/session identity, monotonic
  sequence, operation ID, request digest, and channel binding;
- provider/key agent keeps durable or hardware-backed operation admission and
  result state sufficient for exact retry: `NotStarted`, `InProgress`,
  `Completed(result commitment/result)`, `Refused`, or `Ambiguous`;
- exact retry after crash returns the prior committed result/refusal or explicit
  ambiguous state and never executes the authorization a second time merely
  because the caller lost a response;
- a different request under the same operation ID, sequence reuse, stale
  session, transcript substitution, result substitution, or concurrent consume
  is rejected with replay/equivocation evidence;
- operations whose external side effect cannot be queried or made idempotent
  remain `Ambiguous` after uncertain completion and require governed recovery;
  they are not automatically retried;
- operation/result journal retention, privacy, key rotation, agent replacement,
  rollback, compaction, and historical-verification rules are explicit;
- full rollback/clone of both caller and provider operation state can fork local
  execution unless the profile uses hardware monotonic state or an external
  non-rollback anchor; conflicting operation results must produce replay/
  equivocation evidence when histories are compared, and isolated clones remain
  an explicit residual limitation;
- compile-fail/API tests for public mint/deserialization/clone plus IPC replay,
  cross-class conversion, bootstrap escalation, unlock-to-sign, premature
  authoritative/recovery minting, lost response, crash-before/after admission,
  concurrent consume, reservation exhaustion/collision, repeated randomness,
  explicit absent/present ID tags, unlock range/slot/AD/output substitution,
  oracle response shape, sequence rollback, agent restart, journal rollback,
  request mismatch, and ambiguous-side-effect tests.

Verification:

- `cargo test -p sagnir-crypto`
- operation-capability compile-fail suite;
- key-agent IPC authentication/replay vectors;
- crash/idempotency state-machine and fault-injection tests.

Exit criteria:

- Every provider operation requires one exact sealed capability class; bootstrap
  and unlock can perform only the minimum operations needed to reach full
  policy/lifecycle knowledge and cannot convert into authoritative authority.
- Signing, rotation, promotion, and publication after genesis require the
  v0.70.0 full-compound-admission minting path; recovery requires v0.64.0
  threshold governance.
- Serialization or IPC does not duplicate authority within one non-rolled-back
  provider history, and exact retries there cannot execute an admitted operation
  twice.
- After an uncertain crash, callers receive the prior result or an explicit
  ambiguous state rather than silently minting or replaying authorization.

### v0.23.5 - Bootstrap And Genesis Transaction Ceremony

Goal: reconcile provider/keystore/HSM key state with Sagnir store genesis state
without pretending both durability domains share one atomic transaction.

Deliverables:

- TLA+/PlusCal or equivalent cross-domain ceremony model completes before the
  implementation and treats provider/key-agent state plus v0.23.3 store
  transaction state as independently crashing/recovering authorities;
- canonical ceremony state machine is `Reserved -> KeyProvisioned ->
  GenesisTranscriptFixed -> GenesisSigned -> StoreGenesisCommitted ->
  ProviderOperationFinalized`, with explicit terminal `Aborted`, `Ambiguous`,
  and governed/manual-recovery outcomes;
- `Reserved` durably binds ceremony/realm-initialization ID, retained new-store
  identity, provider and suite, intended first actor/device/key roles, transcript
  schema, limits, bootstrap-log profile ID/public-header commitment, and all
  required v0.23.4 provider operation reservations;
- v0.23.5 implements only the bounded plaintext bootstrap-log profile and records
  its metadata-leakage class; direct encrypted bootstrap profile IDs and fields
  are reserved but have no production path until v0.101.2 completes their key-
  before-first-reservation state machines;
- each transition consumes one separately minted narrowly bound
  `BootstrapCryptoCapability`; key generation/import, genesis signing, key-
  usability/finalization, and any orphan-key destruction are distinct provider
  operations and no capability spans two stages;
- `KeyProvisioned` binds exact public key, provider/key ID, opaque-handle
  commitment or attestation, suite/key epoch, operation ID, provider result
  commitment, and proof that the handle can perform only admitted bootstrap
  operations;
- `GenesisTranscriptFixed` freezes canonical genesis bytes/transcript including
  realm/genesis identity, first actor/device, key/handle commitment, bootstrap
  operation IDs, suite, policy/bootstrap table, store format, bootstrap-log
  profile/evidence commitment, and the initial closed `AuthorityStateRoot` plus
  `AuthorityLogCheckpoint` before signing;
- the genesis authority anchor ends at the exact pre-sign frontier with the
  genesis-signing operation durably reserved; a signed root never claims to
  contain its own signature result, which is consumed into the next authority
  frontier and can be anchored by a later checkpoint or witness;
- `GenesisSigned` binds exact transcript digest, signature, signing operation ID,
  provider/key handle, and result commitment; a lost response is recovered by
  querying the provider journal for that exact operation rather than signing
  again;
- `StoreGenesisCommitted` durably stores signed genesis and all ceremony
  evidence as pending/non-authoritative state under v0.23.3; realm lookup,
  aliases, policy, and later operations cannot observe it as live genesis yet;
- `ProviderOperationFinalized` is recorded and pending genesis is published only
  after the provider/key agent confirms the exact bound key/handle remains
  present, usable for its admitted suite/role, not replaced/retired, and bound to
  the reconciled generation/signature results;
- recovery queries both logs by ceremony/operation IDs and advances only from
  durable evidence, never timestamps, file existence, response arrival order,
  user prompts, or guessed provider state;
- key generated but result not locally recorded, opaque handle committed without
  genesis, fixed transcript without signature, signature generated with response
  loss, genesis bytes durable but publication incomplete, provider completion
  unknown/ambiguous, and final response loss have deterministic recovery states;
- orphan key/handle policy supports provider query, quarantine, explicit
  retained-orphan reporting, and separately authorized destruction where the
  provider can prove it; inability to destroy is residual evidence, not silent
  cleanup success;
- retry with a different public key, provider, suite, transcript, actor/device,
  store identity, or genesis bytes requires a new ceremony and operation IDs;
  old pending state is aborted/quarantined and can never be rebound;
- CLI reports success only after provider finalization evidence and authoritative
  store genesis publication are both durable and mutually bound; partial states
  report exact resumable/ambiguous/manual-recovery status without exposing
  secrets;
- crash/fault injection covers every provider request/result, store write/sync/
  directory-sync/publication, response loss, retry, query, abort, orphan cleanup,
  and recovery transition.

Verification:

- bounded bootstrap/genesis cross-domain model check;
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-cli`
- software provider plus crash/restart ceremony integration suite.

Exit criteria:

- Genesis never becomes authoritative unless the exact signed key/opaque handle
  remains provider-confirmed and store/provider evidence is durably reconciled.
- A crash at any stage resumes, aborts, or reports ambiguity without generating
  a second key/signature under the same operation or binding a different key to
  pending genesis.
- One bootstrap capability authorizes one provider operation, and CLI success
  means both durability domains reached the exact final bound state.
- Genesis authenticates the initial logical authority state and physical log
  checkpoint without a circular claim that it contains its own signing result.

### v0.23.6 - Authority Terminal Fences And Epoch Archives

Goal: admit bounded authority terminal-fence/archive formats, exact replay lookup,
models, and inert transition machinery before later governance, retention, and
causal-stability releases can activate production advancement.

Deliverables:

- model and canonical formats complete before terminal entries can leave the
  active map; the v0.23.3 composite `AuthorityStateRoot` schema is reused rather
  than replaced by an incompatible root;
- production fence advancement, active-entry archival/removal, archive-body
  retirement, and logical compaction are disabled in this release; production
  APIs return an explicit unsupported/not-activated result and cannot accept
  locally fabricated placeholder evidence;
- v0.23.6 implements canonical parsing/verification, replay lookup against
  already admitted fixtures, model/reference transitions, and crash-test-only
  inert transition code; v0.52.0 is the sole activation milestone after its
  verifier composes v0.27.0 checkpoints, v0.35.0 retention policy, and complete
  all-active-replica stability/retirement evidence;
- active map retains operations above the covered fence that are `Reserved`,
  `InProgress`, `Ambiguous`, `Disputed`, equivocation-bearing, sequence-gap, or
  policy-retained completed; at or below the fence, terminal entries live in an
  archive and every unresolved/gap/equivocation/hold state lives in the exact
  sparse exception map rather than pinning all later sequences active;
- canonical fence scope is domain-separated by realm, authority domain, issuer,
  replica ID and incarnation, and reservation-sequence domain so unrelated
  issuers, restored replicas, or operation classes cannot advance one another's
  replay boundary;
- authenticated `CoveredFence(H)` means every reservation sequence in the
  covered interval through `H` is represented exactly once as an allocated
  archived terminal entry, an explicitly burned-unused terminal entry, or an
  exact authenticated exception entry; continuity applies to coverage, not to
  terminal resolution;
- terminal-fence map records scope, `H`, covered interval start/count,
  allocation/burn commitment, archive epoch/manifest/root and terminal count,
  exception-map root/count, exact disjoint interval-coverage proof, stable-
  frontier evidence root, and previous fence value;
- fence advancement proves every sequence newly covered is allocated or
  explicitly burned, every allocated sequence is terminally archived or an
  exact exception, archive/exception/burn counts cover the interval, ordering is
  canonical, and no sequence is omitted, duplicated, or present in both archive
  and exception sets;
- sequence at or below `H` and absent from the exception map is terminal and is
  rejected before provider/key-agent execution even if archive bodies are not
  local; changing operation-ID randomness/request bytes cannot bypass the fence;
- sequence at or below `H` and present in the exception map loads its exact gap,
  ambiguous, disputed, equivocation, retention-hold, or unresolved state and can
  never be interpreted as an unallocated/fresh operation;
- sequence above `H` follows active-map lookup and ordinary reservation rules;
  an absent active entry does not let a caller claim an old covered sequence;
- sparse exception keys bind fence scope and exact reservation sequence, while
  values bind operation ID or an exact allocated-gap commitment, request/status/
  result commitments, ambiguity/dispute/equivocation evidence, retention reason,
  and last transition; a never-allocated sequence requires an authenticated
  burned-unused terminal archive entry, and exception omission invalidates
  coverage;
- resolving an exception atomically archives its terminal entry, removes that
  exact exception, updates archive/exception/count commitments, and leaves `H`
  unchanged or advances it; resolution never rolls the fence backward or
  reopens the sequence;
- canonical predecessor-linked `AuthorityTerminalEpochArchive` binds scope,
  inclusive sequence range, exact sorted terminal entries and result/evidence
  commitments, entry/page counts, v0.17.2 page-set root, prior archive root,
  retention class, and producing authority checkpoint;
- result commitments and evidence needed for genesis, historical signatures,
  provider reconciliation, emergency recovery, erasure/destruction evidence,
  legal hold, or configured audit policy remain authenticated and reachable;
  archival unavailability may block historical verification but never permits
  replay or fabricates successful verification;
- modeled/test-only transition uses one WAL transaction to copy-on-write publish
  archive pages, exception/fence pages, active-map removals, new archive
  manifest, and resulting composite `AuthorityStateRoot`; production cannot call
  this transition until v0.52.0 activation, and partial publication/recovery in
  fixtures keeps the old root;
- full rebuild streams active, fence, exception, and archive manifests under
  cumulative limits and independently recomputes every component/count/root plus
  exact interval coverage/disjointness;
- no Bloom filter, cuckoo filter, approximate set, cache hit/miss, wall clock,
  archive availability, or caller assertion may decide authoritative replay
  acceptance;
- live fence/root mutation and deletion/retirement of replaced active pages or
  detailed archive bodies remain unavailable until v0.52.0 activates them using
  v0.27.0 signed checkpoints, v0.35.0 retention policy, and all-active-replica
  causal stability or governed replica retirement; this release creates no pre-
  governance or quorum-in-place-of-missing-replica mutation authority;
- fence advancement binds the acknowledged stable frontier, per-replica
  sequence watermark, required parent frontier, and replica incarnation from
  v0.52.0; hidden higher-sequence stale-lineage events, cloned replica state, or
  a retired incarnation cannot be legitimized by terminal compaction;
- archive compaction preserves predecessor roots and fence/exception meaning;
  restoration behind an admitted fence/checkpoint is rollback evidence and
  cannot reopen archived operation sequences;
- tests cover a permanent low-sequence exception with millions of later terminal
  operations, archive absence/substitution, fence rollback/jump, allocation or
  burn omission, archive/exception count mismatch, duplicate cross-set sequence,
  exception omission/resolution, sequence gap, wrong issuer/incarnation, crafted
  old-sequence new ID, ambiguous/equivocated fresh-operation attempt, result-
  reference loss, stale root, partial page/WAL publication, rebuild exhaustion,
  and old-checkpoint restore.

Verification:

- bounded terminal-fence/archive state-machine model;
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-crypto`
- independent active/fence/exception/archive root vectors;
- process-kill copy-on-write archive/rebuild suite.

Exit criteria:

- Formats and bounded models demonstrate that later active authority state can
  become proportional to unresolved work above the fence plus exact covered
  exceptions/retention state, but no production fence is advanced in this tag.
- Every replay decision is exact and authenticated; missing archives or caches
  can reduce historical availability but cannot weaken replay refusal.
- Terminal compaction cannot discard ambiguity, equivocation, gaps, governed
  retention evidence, or commitments needed to verify reachable history.
- Production mutation remains visibly inactive until v0.52.0; v0.23.6 success
  cannot be interpreted as live compaction availability.

### v0.24.0 - Key Lifecycle And Anti-Replay

Goal: define key-lifecycle schemas, verification mechanics, and state-machine
rules before live governance activates them.

Deliverables:

- key epoch transition, revocation, compromise, and retirement schemas;
- verification mechanics for lifecycle statements without making parsed
  transitions authoritative;
- immutable genesis bootstrap key epoch;
- explicit rule that no live rotation or revocation becomes authoritative until
  v0.25.0 activates governance;
- revocation and compromise evidence;
- per-key monotonic sequence admission;
- stale-key and stale-epoch rejection;
- duplicate sequence and replay detection;
- bounded role and principal-threshold evaluation;
- algorithm retirement integration;
- key-lifecycle state-machine model.

Verification:

- `cargo test -p sagnir-crypto`
- bounded key-lifecycle model check.

Exit criteria:

- Lifecycle schemas and verification mechanics reject stale, replayed,
  malformed, or algorithm-retired transitions in tests.
- The immutable genesis bootstrap epoch is the only authoritative key state
  before v0.25.0; no self-authorized live transition is admitted.

### v0.25.0 - Live Governance Enforcement

Goal: activate signed realm governance only after signature providers, canonical
transcripts, identity schemas, and key lifecycle rules exist.

Deliverables:

- cryptographically verified genesis administrator authority;
- joint admission of the immutable genesis bootstrap key epoch, initial
  governance root, and initial key-lifecycle state as one genesis-bound
  operation;
- no circular requirement for the initial governance root to authorize itself;
- signed actor and device enrollment;
- signed membership and role transitions;
- threshold administrative actions over independent principals;
- signed ownership transfer and emergency recovery;
- live invitation issuance, acceptance, consumption, expiry, revocation, and
  supersession transitions using the v0.19.0 lifecycle schema;
- authenticated governance event chain and interim governance root;
- transition result commits the exact previous and resulting governance roots;
- no dependency on the later checkpoint schema for v0.25.0 validity;
- first-contact trust binding for imported governance history;
- unauthorized enrollment, stale administrator, split governance, threshold
  inflation, genesis substitution, and recovery replay tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-policy`
- governance state-machine model.

Exit criteria:

- No membership, role, policy authority, or key-registry mutation is accepted
  solely because it is well-formed.
- An invitation can create authority only once, within its exact signed scope,
  while its issuer and governance epoch remain admitted.
- Every live governance mutation verifies against the admitted genesis and
  current governance root.
- The initial governance and key-lifecycle state is admitted directly from the
  authenticated genesis commitment; only subsequent transitions require the
  current governance authority.
- Governance transitions are authenticated before checkpoints exist, but
  checkpoint anchoring is deferred explicitly to v0.27.0.

### v0.26.0 - Canonical Signed Event Envelope

Goal: define authoritative events before an event DAG or WAL records them.

Deliverables:

- canonical event envelope and kind registry;
- actor, device, replica, sequence, parent, and realm binding;
- bounded payload commitment;
- policy, key, and crypto epoch binding where authoritative;
- distinction between authoritative state events and diagnostic command events;
- malformed, unknown-critical-kind, replay, and event-kind-confusion tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-crypto`

Exit criteria:

- The signed event DAG consumes one canonical event type whose authority and
  scope are explicit.

### v0.27.0 - Checkpoint Schema And Signed Event DAG

Goal: commit all authoritative realm roots and detect rollback or equivocation.

Deliverables:

- signed per-replica sequence chains;
- event DAG parent and frontier commitments;
- checkpoint schema committing to object/state, alias/frontier, event, fact,
  policy, key-registry, crypto-epoch, current logical `AuthorityStateRoot`, and
  current physical `AuthorityLogCheckpoint` commitments;
- checkpoint authority roots must descend from the genesis authority anchor and
  every previously admitted checkpoint; missing committed suffixes, deleted
  ambiguous/equivocation evidence, terminal-status rollback, or resurrection of
  consumed/cancelled operation IDs fails against genesis, a later checkpoint,
  or an independently retained witness;
- checkpoint signing freezes a closed prior authority frontier; its preallocated
  signing reservation is present there, while its execution/result transition
  enters the next frontier, preventing signature self-inclusion without making
  the signing operation disappear from authority history;
- compaction after a signed checkpoint may publish a predecessor-linked physical
  `AuthorityLogCheckpoint` with a different layout/chain root only when the
  logical `AuthorityStateRoot` is identical; the next signed realm checkpoint
  or witness anchors that physical transition;
- exact v0.25.0 governance root anchored as part of the key-registry and policy
  interpretation state;
- continuity check from genesis through the authenticated governance event
  chain to the checkpointed governance root;
- previous-checkpoint commitment;
- equivocation evidence for conflicting events at one sequence;
- missing-suffix and rollback diagnostics;
- independent checkpoint vectors and bounded event-DAG model.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-crypto`
- checkpoint vector validator;
- event-DAG model check.

Exit criteria:

- Every trusted realm checkpoint binds all roots required to interpret state.
- Every trusted checkpoint authenticates both logical operation state and its
  physical authority-log frontier without requiring a signature to contain its
  own execution result.
- A checkpoint cannot substitute, skip, or fork the previously authenticated
  governance root without producing an invalid continuity proof.
- Rollback and tampering are detectable relative to an admitted later
  checkpoint or witness.

### v0.28.0 - Authoritative Time Model

Goal: define ordering, expiry, and timestamp trust for offline operation.

Deliverables:

- causal and checkpoint ordering as authoritative ordering;
- advisory wall-clock timestamp representation;
- local monotonic-clock rules for unlock-session TTL;
- offline key validity and revocation rules;
- expiry behavior when trustworthy time is unavailable;
- canonical authenticated time-statement format binding realm, authority ID and
  key epoch, monotonically increasing authority sequence, prior statement or
  consistency root, asserted interval or instant, uncertainty bound, issuance
  checkpoint, expiry, policy domain, nonce, and signature;
- time statements attest ordering or bounded time only within their declared
  uncertainty and policy domain; they never replace causal ancestry or prove
  that an offline event did not exist;
- timestamp-authority trust roots, enrollment, scope, threshold role, key
  rotation, revocation, retirement, compromise, and historical verification;
- append-only authenticated authority log with inclusion and consistency proofs
  so sequence omission, rollback, split view, and equivocation produce durable
  evidence;
- quorum and administrative/provider diversity rules for profiles that require
  more than one authority, including exact interval intersection and refusal
  behavior when quorum statements conflict or do not overlap;
- fail-closed behavior when required authorities are unavailable, stale,
  revoked, equivocal, outside scope, or beyond policy uncertainty;
- offline verifier freshness limits bound to the last admitted authority
  checkpoint, maximum statement age or sequence lag, and policy; exceeding the
  limit yields unknown/quarantine rather than valid or expired;
- privacy-preserving request protocol using opaque realm/subject handles,
  request batching or padding where configured, no plaintext path/object/policy
  disclosure, and prohibition on stable subject correlators in public logs;
- stapled authority checkpoint and append-only consistency proof format for
  every expiry- or revocation-sensitive artifact, including quota rights,
  invitations, recipient state, target-only translation attestations, resume
  tokens, receipts, and selective disclosures;
- artifact verification checks the stapled statement/checkpoint against the
  verifier's latest admitted authority and revocation roots; a stale staple
  cannot prove current non-revocation beyond policy freshness;
- canonical revocation statement format and consistency proof shared by time-
  sensitive authority assertions;
- clock rollback, skew, suspend/resume, restored snapshot, unavailable
  authority, stale statement, conflicting intervals, missing quorum, authority
  key rotation/revocation, split view, equivocation, privacy leakage, and
  offline-freshness tests.

Verification:

- `cargo test -p sagnir-core`
- time-policy state-machine tests.
- independent time-statement, revocation, and consistency-proof vectors;
- authority split-view and quorum model.

Exit criteria:

- Canonical validity never depends silently on an unauthenticated local wall
  clock.
- Required authoritative time and revocation decisions have canonical,
  monotonic, replay-resistant evidence with explicit freshness and failure
  semantics.
- A timestamp authority cannot silently fork, roll back, or extend its sequence
  without producing invalid consistency or equivocation evidence.

### v0.29.0 - Typed Ingest Contract

Goal: establish the type-state boundary before durable ingest APIs are written.

Deliverables:

- distinct untrusted, canonical, hash-verified, reference-derived,
  causally-closed, signature-verified, policy-admitted, and committed states;
- concrete capability families such as `VerifiedCanonicalObject`,
  `VerifiedObjectGraph`, `SignatureVerifiedEvent`, `AdmittedTransaction`, and
  `CommittedState`, with exact target/root/context parameters rather than one
  generic validation token;
- consuming transitions between each stage;
- no public constructors that skip stages;
- diagnostic structural-validation results named separately from proofs;
- target, realm, policy, crypto, and checkpoint binding;
- compile-fail tests for bypass and cross-stage reuse.

Verification:

- `cargo test --workspace`
- API compile-fail test suite.

Exit criteria:

- Storage and world APIs cannot be designed around raw bytes, caller-supplied
  graph claims, or generic validation booleans.
- Shared store APIs accept only the stage-specific typed capability required by
  the operation; key agents and frontends cannot mint or substitute it.

### v0.30.0 - Durability Profile And WAL Recovery Model Gate

Goal: extend the v0.23.3 durability assumptions and model from capability
authority records to object/fact/world/checkpoint transactions before adding
those record kinds or their writer behavior.

Deliverables:

- TLA+/PlusCal or equivalent WAL recovery model;
- initial machine-readable durability-profile contract covering local admitted,
  degraded, read-only inspection, and unsupported storage;
- model parameter for active durability profile and its file-data, metadata,
  rename/no-replace, file-sync, directory-sync, parent-sync, barrier/write-cache,
  mount, and kernel-honesty assumptions;
- the model cannot prove a stronger invariant by assuming a sync/publication
  guarantee that the selected profile does not provide;
- v0.32.1 later broadens platform/filesystem detection and test matrices but
  does not define the model's durability semantics retroactively;
- object, fact, world, alias, and checkpoint extension atomicity invariants while
  preserving v0.23.3 operation reservation/result semantics on one log;
- write, rename, file-sync, and directory-sync failure points;
- stale checkpoint and missing-body states;
- counterexamples for partial commit and alias-before-body behavior;
- bounded model-check command required by the release gate;
- versioned model-run manifest records model/tool digest, exact bounds, write/
  rename/sync failure points, fairness assumptions, state-space reductions,
  explored states/transitions/depth, completion versus timeout, resources, and
  counterexample traces;
- CI smoke and pentest/release profiles have distinct declared bounds and a
  smaller or timed-out run cannot replace the required release artifact.

Verification:

- bounded WAL model check;
- model invariant review.

Exit criteria:

- The WAL format and writer milestones begin only after the model has no known
  counterexample within admitted bounds.
- Every model result names its durability profile; changing profile assumptions
  invalidates rather than silently reuses the result.

### v0.31.0 - Extended Chained WAL Commitment Format

Goal: extend the v0.23.3 base authority-log frames with bounded multi-domain WAL
records and chained transaction commitments before writing those new records.

Deliverables:

- inherit v0.23.3 WAL magic, base format/version, transaction/reservation IDs,
  critical-kind refusal, CRC-32C parameters, SHA3-256 commitment suite/domain
  transcripts, logical authority-state root, physical log-checkpoint format, and
  operation records unchanged;
- versioned extension registry for object/fact/world/alias/checkpoint records;
- segment/log identity and segment generation;
- frame kind;
- transaction ID;
- reserved monotonic LSN and per-segment frame sequence;
- transaction/LSN exhaustion behavior, with fail-closed refusal before wrap;
- log-incarnation transition format for initialization, restored snapshot,
  cloned store, truncation recovery, checkpoint cutover, and explicit fork;
- an LSN, transaction ID, or frame sequence is never used alone as an AEAD
  nonce; encrypted WAL operation remains dormant until v0.92.0 admits a clone-
  safe construction under the v0.31.1 contract;
- reuse of an old LSN/sequence under a new log incarnation cannot repeat an
  effective AEAD nonce/key pair or alias an old committed frame;
- payload length;
- prior-frame commitment;
- CRC-32C uses the Castagnoli polynomial `0x1EDC6F41` (reflected
  `0x82F63B78`), reflected input/output, initial value `0xFFFF_FFFF`, final XOR
  `0xFFFF_FFFF`, and the check value `0xE306_9283` for ASCII `123456789`;
- CRC-32C is serialized little-endian and covers the exact canonical bytes of
  magic, version, segment/log identity, generation, LSN/sequence, transaction
  ID, kind, payload length, prior commitment, and payload; the checksum field
  is excluded and the frame has no implicit/alignment padding;
- strict frame and transaction byte/count/work limits before CRC/hash work;
- ordered payload digest commitment;
- previous committed transaction or checkpoint commitment;
- commit marker binding ordered frame digests, frame/byte counts, expected old
  realm frontier, resulting realm frontier, and previous committed transaction
  or checkpoint;
- encrypted-realm WAL authentication profile using admitted AEAD or keyed MAC in
  addition to non-adversarial torn-write/corruption detection;
- explicit statement that CRC and an attacker-recomputable local hash chain are
  not proof against a malicious storage controller;
- malformed frame, hardware/software differential, and CRC-32C known-answer
  tests;
- exhaustion, truncation, snapshot restore, cloned-store, incarnation rollback,
  nonce-reuse, old-epoch, and cross-segment substitution tests.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- WAL frames reject malformed length, kind, sequence, and checksum metadata.
- Reordering, removal, insertion, or replay changes the transaction commitment.
- Adversarial tampering is claimed detectable only relative to an admitted
  signed checkpoint or trusted witness; a store controller can recompute
  unauthenticated local chains.
- General WAL record kinds extend the v0.23.3 commitment transcripts; they do
  not redefine or algorithm-negotiate the authority-log prefix.

### v0.31.1 - Clone-Safe Encrypted WAL Activation Contract

Goal: prevent rollback or undetected store cloning from causing forbidden WAL
nonce/key reuse when encrypted realms activate later.

Deliverables:

- v0.31.0 and v0.32.0 implement plaintext/non-secret WAL framing, chaining,
  CRC-32C, recovery, and reserved encrypted-profile parsing only; production
  encrypted WAL write/replay is unavailable until v0.92.0 admits its exact
  suite, nonce/per-record-key construction, and vectors;
- parser distinguishes `unsupported encrypted profile` from malformed,
  authentication failure, and locked-key-unavailable states and never falls
  back to plaintext or an experimental nonce mode;
- log incarnation remains domain/replay context but is not treated as proof
  that a restored snapshot, copied filesystem, VM clone, or duplicated block
  device was detected;
- every admitted encrypted WAL profile remains nonce/key safe when two
  undetected clones continue from identical durable counters and incarnation;
- admitted constructions must use sufficient independent OS-CSPRNG randomness,
  an approved misuse-resistant/nonce-robust AEAD, independent per-record keys,
  or an external non-rollback uniqueness anchor, with exact failure assumptions
  and no counter/incarnation-only safety claim;
- random construction failure stops before emitting a frame; a cloned random
  generator/VM state is covered by the v0.23.0 entropy threat model and cannot
  be papered over by incrementing the log incarnation;
- external anchors, when required by a profile, define availability, rollback,
  partition, stale-read, and disaster-recovery behavior and fail closed when
  freshness cannot be established;
- restored snapshot, silent filesystem copy, VM clone, duplicated process,
  repeated randomness, counter/incarnation collision, external-anchor rollback,
  crash, and concurrent writer model/tests;
- v0.92.0 must instantiate this contract and publish known-answer, differential,
  clone, and failure-injection vectors before encrypted WAL becomes writable.

Verification:

- `cargo test -p sagnir-store`
- encrypted-profile fail-closed parser fixtures;
- clone/snapshot nonce-allocation state-machine model;
- duplicate-state and repeated-randomness fault injection.

Exit criteria:

- Early WAL releases do not depend operationally on cryptography admitted only
  at v0.92.0.
- No encrypted WAL profile relies on successful clone detection or a log-
  incarnation increment as its sole nonce-uniqueness argument.
- Unsupported encrypted WAL bytes cannot be written, replayed as authoritative,
  or silently downgraded.

### v0.32.0 - WAL Writer And Recovery

Goal: make committed local transactions recoverable.

Deliverables:

- begin transaction;
- acquire one shared-store writer lease and reserve a monotonically ordered
  transaction/LSN before append;
- append frame;
- commit transaction;
- sync WAL bytes and commit frame before reporting durability;
- ignore only a provably incomplete final transaction;
- quarantine checksum failure, authentication failure, sequence discontinuity,
  unexpected segment transition, or malformed bytes inside committed history;
  recovery never scans forward for a convenient next frame;
- replay committed frames;
- extend the existing v0.23.3 transaction substrate in place with object/fact/
  world/alias/checkpoint record kinds and recovery rules; there is no legacy-log
  import, dual writer, or authority-source switch, and existing operation IDs/
  statuses remain byte-for-byte authoritative;
- encrypted WAL write and authenticated/decrypted replay paths remain
  unavailable until v0.92.0 instantiates the v0.31.1 clone-safe activation
  contract; unsupported encrypted profiles never fall back to plaintext;
- locked-realm recovery may validate public framing, lengths, CRC, and clearly
  public segment continuity only; authenticated/decrypted replay, semantic
  interpretation, and authoritative alias/root/checkpoint publication wait for
  the required WAL key epoch;
- locked status distinguishes `framing checked`, `authentication pending`,
  `replay pending`, and `quarantined` without reporting unverified history as
  committed state;
- rekey retains old WAL authentication/decryption keys while any non-retired
  segment or recovery checkpoint can require them;
- old WAL keys retire only after a durable new checkpoint proves all old-key
  transactions are represented, required rollback/recovery policy is satisfied,
  and no admitted retained segment depends on the old epoch;
- replay idempotently into immutable content-addressed files using no-replace
  publication;
- publish aliases, roots, and checkpoints only after all referenced bodies are
  durable, using expected-old-value CAS, file sync, atomic rename, and directory
  sync;
- retire a WAL segment only after a durably published checkpoint proves every
  committed effect in that segment is represented;
- file and directory synchronization at required durability boundaries;
- recovery tests for torn writes and every write/rename/sync boundary;
- refusal when a committed alias lacks any referenced immutable body;
- Loom or equivalent writer-lease and publication schedule tests;
- process-kill recovery after every append, commit, sync, replay, CAS, rename,
  directory-sync, checkpoint, and segment-retirement boundary.

Verification:

- `cargo test -p sagnir-store`

Exit criteria:

- Startup can recover committed operations and ignore incomplete ones.
- Recovery cannot apply a committed frontier or alias before all transaction
  bodies are present and verified.
- Recovery is deterministic and idempotent: repeated restart yields all and
  only committed transactions, never a skipped committed frame or partial
  authority update.
- Locked recovery never advances authoritative state from CRC-only evidence,
  and rekey cannot destroy the only key needed to authenticate retained WAL.

### v0.32.1 - Filesystem Durability Profiles And Publication Boundaries

Goal: implement and broaden the v0.30.0 durability-profile contract across
admitted filesystems instead of claiming guarantees that hardware or kernels
cannot provide.

Deliverables:

- named machine-readable durability profiles for tested local filesystems,
  explicitly degraded filesystems, read-only inspection, and unsupported
  storage;
- each profile states assumptions for file data, metadata, rename, no-replace,
  file sync, directory sync, parent-directory sync, write caches, barriers,
  copy-on-write behavior, network mounts, removable media, and kernel honesty;
- verify and preserve the v0.10.1 rule that `.saga/` creation synchronizes the
  already-open parent root directory after the new entry and required initial
  state are durable on every newly admitted durability profile;
- every newly created nested directory has a defined parent-sync boundary;
  syncing only `.saga/` cannot be presented as proving the root entry survived;
- store-open preflight detects known unsupported or degraded filesystem/mount
  capabilities where the operating system exposes them and warns or refuses
  according to policy;
- lying disks, unsafe volatile write caches, hostile kernels, and unobservable
  network-filesystem semantics are explicit residual assumptions, not covered
  by an `absolute durability` claim;
- crash harness injects short writes, `EINTR`, allocation failures, write
  failures, rename failures, file-sync failures, directory-sync failures, and
  process death at every publication boundary;
- admitted integration matrix for ext4, XFS, Btrfs, APFS, and NTFS where test
  infrastructure is available, with unsupported configurations documented
  rather than silently generalized;
- recovery result reports the active durability profile and any degraded
  guarantee without treating degraded storage as cryptographic proof failure;
- root/store handle identity and attachment are rechecked before and after
  critical publication so mount or namespace replacement cannot upgrade a
  degraded path into an admitted one.

Verification:

- platform filesystem crash-injection suite;
- parent-root and nested-parent directory durability fixtures;
- repeated deterministic recovery test for every injected boundary;
- durability-profile capability and refusal tests.

Exit criteria:

- Sagnir can state exactly which crash-durability property was tested on the
  active storage profile and where hardware/kernel assumptions begin.
- A successful initialization cannot omit the parent-directory sync needed to
  make the `.saga/` directory entry durable under its admitted profile.
- Unsupported or degraded storage never inherits a stronger durability claim
  merely because ordinary tests happened to pass.

### v0.33.0 - Loose Object Store

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

### v0.34.0 - Local Fsck And Verification Modes

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
- detect and validate only cache formats already admitted by earlier
  milestones;
- ignore unknown non-authoritative cache data safely;
- never require a cache for integrity or successful fsck;
- report that deterministic proof-cache deletion and rebuild remain unavailable
  until v0.69.0;
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
- Cache absence, unknown cache bytes, or safely ignored non-authoritative cache
  formats cannot make an otherwise valid store fail integrity verification.

### v0.35.0 - Canonical Realm And World Policy

Goal: make transition validity signed, deterministic, and history-bound before
daily mutation commands depend on policy.

Deliverables:

- canonical realm and world policy object;
- deterministic policy language and evaluator version;
- total or provably terminating policy constructs;
- fuel or instruction, recursion, allocation, output, and collection-size
  limits;
- no floating point, locale-dependent behavior, unspecified iteration order, or
  unpinned Unicode and regular-expression semantics;
- signed policy epoch transitions;
- historical evaluation under the policy epoch and evaluator version committed
  by the transition;
- evaluator compatibility, migration, and retirement rules that preserve
  verification of old history;
- seal, merge, promotion, evidence, and retention requirements;
- policy root committed into state and checkpoints;
- no filesystem, environment, network, or untrusted wall-clock inputs during
  canonical evaluation;
- malformed, nondeterministic, stale-epoch, and unauthorized-policy tests;
- policy-complexity, fuel exhaustion, allocation, recursion, Unicode-version,
  regex-version, and evaluator-upgrade tests.

Verification:

- `cargo test -p sagnir-policy`
- deterministic cross-platform policy vectors;
- policy-transition state-machine model.

Exit criteria:

- Peers given the same admitted state and evaluator version reach the same
  canonical policy decision.
- Policy authority traces to realm governance rather than a local file.
- Evaluator upgrades cannot retroactively reinterpret a historical transition.

### v0.36.0 - Local Acceptance Policy

Goal: let each device enforce stricter local trust and materialization rules
without redefining realm validity.

Deliverables:

- local acceptance policy file;
- profile defaults for `standard`, `solo`, `team`, and `regulated`;
- ingest, quarantine, decrypt, retention, and materialization rules;
- explicit comparison with canonical realm/world policy;
- behavior for peers with stricter local policy;
- refusal to use local policy to make an invalid realm transition valid;
- invalid, downgrade, and policy-confusion tests.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-store`

Exit criteria:

- Local policy can refuse otherwise valid state but cannot authorize state that
  canonical realm policy rejects.
- `saga save` and later mutation commands have a policy engine available before
  they are introduced.

## Phase 3: Worktree And Source State

### v0.37.0 - Byte-Preserving Worktree Path Model

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

### v0.38.0 - Worktree Path Scanner

Goal: classify source paths safely across supported operating systems.

Deliverables:

- relative path scanner;
- `.saga/` control path exclusion;
- parent traversal rejection;
- Windows separator rejection policy;
- root-bound directory handle traversal;
- Linux resolver uses `openat2` with `RESOLVE_BENEATH`,
  `RESOLVE_NO_MAGICLINKS`, and the configured symlink/mount policy when the
  running kernel supports the admitted semantics; fallback and other platforms
  retain and walk native directory handles component by component with
  no-follow behavior;
- symlink and reparse-point policy;
- resolver returns an opened directory/file handle or a non-authoritative
  display result, never a reusable path-detached "verified path" token;
- opened-handle capability is bound to the exact root, relative path, file
  identity, type, mount/volume identity where available, and policy for its
  lifetime;
- no reusable zero-sized symlink proof capability;
- special-file rejection;
- tracked symlink-as-data semantics without silently following the target;
- sparse-file and hard-link classification policy;
- replacement-race tests around scan and materialization;
- case-folding, Unicode normalization, Windows 8.3 alias, reparse point,
  hard-link, mount/volume boundary, and concurrent component replacement tests;
- path tests for Linux, Windows-style separators, BSD, MacOS, Android, and iOS.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Sagnir never treats `.saga/` control data as source content.
- Windows-style separator inputs are rejected consistently before control-path
  materialization.
- A path admitted under one root cannot be reused as proof for another root or
  after the underlying file identity changes.
- Documentation states that a malicious process under the same user identity
  may ptrace, signal, or mutate user-owned state; stronger isolation requires a
  separate service account, sandbox, or privileged key agent and is not implied
  by handle-relative traversal alone.

### v0.39.0 - Ignore Rules

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

### v0.40.0 - Blob And Tree Builder

Goal: build deterministic source tree objects from tracked files.

Deliverables:

- blob object creation;
- open-handle-based streaming file hashing;
- pre/post file identity, size, and metadata validation;
- retry or refusal when a file changes during hashing;
- bounded reads from the retained handle rather than reopening the display path;
- hard-link deduplication policy without confusing path identity;
- sparse-file logical-content policy;
- symlink target bytes stored as link data;
- tree entry sorting;
- incremental tree construction under memory budgets;
- executable metadata policy;
- empty directory policy;
- concurrent mutation, hard-link, sparse-file, symlink, and special-file tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Equivalent worktrees produce equivalent tree object bytes.
- A file changed during snapshot cannot be sealed under a digest from a mixed
  or unstable read.
- Later materialization writes a temporary file through the already-opened
  destination directory and atomically publishes it; Sagnir never mutates an
  existing hard-linked worktree file in place.

### v0.41.0 - Incremental Worktree Index

Goal: make large-worktree status and sealing efficient without treating cache
metadata as integrity evidence.

Deliverables:

- rebuildable content/stat cache keyed by stable file identity and metadata;
- filesystem watchers used only as invalidation hints;
- racy-timestamp defense;
- deterministic fallback to open-handle hashing;
- parallel I/O scheduler with memory and open-file limits;
- cache generation and corruption detection;
- stale watcher, timestamp collision, replaced-file, cache corruption, and
  bounded-parallelism tests.

Verification:

- `cargo test -p sagnir-worktree`
- million-file synthetic index benchmark scaffold.

Exit criteria:

- Cache hits can accelerate status but can never prove content integrity.
- Deleting or corrupting the index causes a bounded deterministic rebuild.

### v0.42.0 - State Root Object

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

### v0.43.0 - Status Command

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

### v0.44.0 - Text Diff

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

### v0.45.0 - Binary And Large File Bounds

Goal: protect status and diff from unbounded memory behavior.

Deliverables:

- binary detection;
- large file size limits;
- bounded read behavior;
- optional content-defined chunk object format for large files;
- chunk boundary, maximum chunk count, and total expanded-size bounds;
- same-compartment private deduplication rule;
- no dependency on pack-only delta compression for large-file reuse;
- clear binary diff output;
- tests for large, sparse, binary, adversarial chunk-boundary, and chunk-fanout
  files.

Verification:

- `cargo test -p sagnir-worktree`

Exit criteria:

- Large or binary files do not cause unbounded diff allocations.

## Phase 4: Worlds

### v0.46.0 - World Metadata

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

### v0.47.0 - World Aliases

Goal: map human world names to immutable world states.

Deliverables:

- TLA+/PlusCal or equivalent alias CAS and concurrent-head model completed
  before alias update code;
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

- bounded alias/CAS model check;
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-world`

Exit criteria:

- Mutable names point only to existing immutable world states.
- Concurrent alias updates preserve every admitted head and never silently
  discard divergent history.

### v0.48.0 - World Open And List

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

### v0.49.0 - World Switch Materialization

Goal: materialize another world into the worktree safely.

Deliverables:

- tree diff between materialized states;
- staged file update plan;
- durable materialization journal;
- backup, resume, and rollback plan;
- honest partial-materialization state after interruption;
- `saga world switch`;
- write, rename, sync, interruption, and recovery tests.

Verification:

- `cargo test -p sagnir-worktree`
- `cargo test -p sagnir-cli`

Exit criteria:

- Switching worlds is detectably and recoverably transactional at the Sagnir
  level; it does not claim that a multi-file filesystem update is globally
  atomic.

### v0.50.0 - Dirty Worktree Protection

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

### v0.51.0 - Materialization Recovery Journal

Goal: make interrupted checkout, switch, undo, and restore operations resumable
without overstating filesystem atomicity.

Deliverables:

- canonical local materialization journal;
- expected old and intended new worktree state commitments;
- per-path staged, published, restored, and failed states;
- durable progress markers and directory synchronization;
- startup resume, rollback, and manual-repair diagnostics;
- crash injection at every write, rename, file-sync, and directory-sync point.

Verification:

- `cargo test -p sagnir-worktree`
- materialization crash-consistency suite.

Exit criteria:

- Every interrupted multi-file materialization is detected at startup and can
  be resumed, rolled back, or reported for bounded manual repair.

### v0.52.0 - Replica Retirement And Causal Compaction

Goal: bound causal context growth without hiding unseen concurrent state.

Deliverables:

- signed replica retirement transition;
- causal-stability certificate bound to one membership epoch and exact replica
  set;
- acknowledged frontier and sequence watermark from every active replica in
  that membership epoch;
- acknowledgement transcript signs the stable frontier, replica sequence
  watermark, replica incarnation, and required parent frontier for that
  replica's next event;
- every later event from the replica must causally descend from the acknowledged
  stable frontier;
- higher-sequence events based on a pre-stability parent are stale-lineage
  forks even without sequence-number reuse;
- explicit prohibition on replacing a missing active replica's acknowledgement
  with a governance quorum;
- governance-authorized retirement for every missing replica before stability
  can be certified;
- retirement transition with an exact cutoff frontier and sequence watermark;
- post-retirement rule that later submissions from the retired replica are
  rejected from the compacted lineage or enter an explicit recovery/fork
  process;
- new replica incarnation or epoch required after retirement, restored state,
  cloned device state, or loss of the last admitted sequence/frontier;
- concurrent writer exclusion for two processes using one replica identity;
- explicit Byzantine, indefinitely offline, retired, and later-returning
  replica behavior;
- retirement grace period and pre-retirement-state admission rule;
- invalidation when governance equivocation, membership rollback, or a
  conflicting stability certificate is detected;
- bounded frontier and concurrent-head representation;
- tombstone and retirement-evidence retention rules;
- safe dotted-context or Merkle-clock compaction;
- causal-stability certificate is the required authority for advancing the
  matching v0.23.6 terminal-operation fence and archive eligibility; its replica
  set/frontiers/watermarks/incarnations must cover every affected operation
  scope, and a governance quorum cannot stand in for a missing acknowledgement;
- activate the formerly inert v0.23.6 fence/archive transition only after one
  typed admission result verifies the exact v0.27.0 checkpoint ancestry,
  v0.35.0 canonical retention/legal-hold decision, membership epoch, every
  active-replica acknowledgement or governed retirement cutoff, covered interval
  proof, and expected old composite authority-state root;
- absence, staleness, unsupported version, or ambiguity in any prerequisite
  keeps production advancement disabled for that scope; local configuration,
  operator privilege, resource pressure, or a governance quorum alone cannot
  substitute for the typed activation evidence;
- activated transition atomically publishes active/archive/exception/fence pages
  and the new composite root through the v0.23.6 WAL contract, then records the
  exact stability/checkpoint/retention evidence roots used for the decision;
- the first production advancement in a store includes a critical
  `TerminalFenceActivation` record that sets the activation feature bit, binds
  the exact v0.52.0 evidence/schema version, raises minimum authoritative
  verifier and writer versions to at least v0.52.0, and commits the feature floor
  into store state plus the resulting signed checkpoint;
- v0.11.0 transactional upgrade machinery stages matching `.saga/FORMAT`
  minimum-version metadata and the critical WAL/checkpoint activation, then
  publishes one recoverable activation result; before final publication the old
  root remains authoritative and writes are fenced, while afterward no old
  writer can reopen the pre-activation root;
- the critical activation record is self-refusing for pre-v0.52 binaries even if
  `.saga/FORMAT` publication is interrupted or maliciously rolled back; format
  metadata, WAL feature floor, checkpoint feature floor, and current root must
  agree before writable or fully verified status is reported;
- older readers within the retained decoder range may use an explicit read-only
  historical-inspection mode for independently verifiable objects/checkpoints
  predating activation, but cannot verify/publish the activated authority root,
  advance aliases, write operations, or report the current store as fully
  verified;
- terminal fence/archive transition records the exact stability or retirement
  evidence root so later restoration, stale-lineage submission, or hidden
  higher-sequence operation fails against both causal and replay boundaries;
- activation tests prove v0.23.6-only binaries/fixtures refuse production
  advancement, missing checkpoint/policy/replica evidence fails closed, and the
  first admitted v0.52.0 transition preserves replay refusal across restart;
- mixed-version/golden-store tests cover pre-activation v0.23.6 readers, first
  activation, crash at every format/WAL/checkpoint publication boundary,
  `.saga/FORMAT` rollback, activation-bit stripping, old-writer reopen, explicit
  historical read-only inspection, and current-root full-verification refusal;
- replica-creation quotas and governance-backed Sybil controls;
- actor- and device-level aggregate replica and duplicate-creation budgets whose
  accounting survives new replica incarnations, device restoration, replica
  retirement, and private-locator epoch rotation;
- escrowed bounded-counter quota-right model: each aggregate actor/device budget
  is split into signed non-overlapping rights allocations assigned to exact
  replica incarnations, and an offline replica may admit quota-consuming state
  only by consuming rights it already holds;
- quota-right allocation binds realm, policy and membership epoch, aggregate
  quota identity, actor/device principal, replica incarnation, amount, consumed
  interval or token set, issuance frontier, expiry frontier or admitted
  timestamp-authority statement, retirement rule, and governing authorization;
- quota-right expiry inherits the v0.28.0 authoritative-time model: canonical
  expiry is bound to a membership/checkpoint frontier or an admitted timestamp
  authority and never to an unauthenticated local wall clock;
- an offline spend is evaluated at its signed creation frontier, not network
  delivery time; a spend created before expiry but delivered afterward remains
  eligible only when its causal ancestry proves the pre-expiry frontier and it
  did not observe or descend from the expiry transition;
- a replica unable to observe an expiry transition may create a candidate, but
  merge quarantines it when the signed causal/time evidence cannot establish
  pre-expiry creation;
- expiry concurrent with spend or transfer creates explicit quota conflict
  unless causal ordering or the admitted timestamp statement deterministically
  places the operation before expiry;
- clock rollback, skew, suspend/resume, or restored local state cannot extend a
  quota right or make an expired right authoritative;
- quota-right transfer is a governed causal transition that debits the sender's
  unconsumed rights before crediting the receiver, binds expected allocation
  roots by compare-and-swap, and cannot be replayed or double-spent across
  partitions;
- retirement may reclaim a replica's remaining rights only from a signed
  surrender, a causal-stability acknowledgement that commits the replica's
  final spent-right root, or an explicit retirement cutoff that invalidates
  every unseen spend from that replica after the admission grace period;
- when no surrender, final spent-right acknowledgement, or invalidating cutoff
  is available, the replica's remaining rights are permanently burned rather
  than redistributed; governance cannot infer that a missing replica left its
  rights unused;
- merge verifies disjoint consumed-right intervals or tokens; a candidate or
  dependent transition without valid unspent rights remains durably
  quarantined behind an explicit aggregate-quota conflict and never becomes
  authoritative merely because each partition's local counter was below the
  aggregate limit;
- governance may increase an aggregate budget or redistribute unused rights,
  but cannot retroactively manufacture rights for already overdrawn offline
  candidates without an explicit policy transition and conflict resolution;
- resolving an overdraw through newly granted rights creates a new signed
  ratification and admission transition at the new causal point; it never
  rewrites the original event as valid at its earlier frontier;
- the original overdrawn event remains immutable evidence of initial
  quarantine, every dependent transition is re-evaluated under the ratification
  result, and rejection preserves the signed event without admitting its state
  effects;
- governance enrollment and retirement transitions carry the aggregate quota
  identity, allocations, spent-right commitments, and unresolved conflicts
  forward; creating a new replica identity cannot reset limits or inherit
  unspent rights without an explicit policy-authorized transfer;
- quota schemas support encrypted scoped disclosure: allocations, spent-right
  references, conflicts, replica topology, and activity metadata are prohibited
  from blind-store metadata, filenames, telemetry, ordinary logs, and locked
  status in sealed-private realms;
- tests proving compaction cannot erase an unseen concurrent head;
- retirement, rejoin, stale replica, and malicious replica-fanout tests;
- unseen concurrent head, missing acknowledgement, governance-quorum
  substitution, rollback, conflicting certificate, and late pre-retirement
  state tests;
- malicious replica test that acknowledges stability and later reveals a
  previously signed but omitted change;
- hidden higher-sequence stale-parent event, cloned device, restored replica,
  reused incarnation, two-process same-replica, partitioned quota consumption,
  double-spent rights, replayed transfer, overlapping allocation, offline
  overdraw quarantine, governance increase and ratification, dependent
  re-evaluation, signed surrender, final spent-root acknowledgement, retirement
  cutoff, unseen pre-cutoff spend, burned rights, refused redistribution,
  pre-expiry creation with late delivery, unobserved expiry, concurrent
  spend/transfer and expiry, clock rollback/skew, and private quota-metadata
  leakage tests.

Verification:

- `cargo test -p sagnir-world`
- causal-compaction state-machine model.
- escrowed quota-right bounded-counter and partition-merge model.

Exit criteria:

- Device churn does not make frontiers grow without bound.
- Production authority active state can become proportional to unresolved work
  above each covered fence plus exact exceptions/retention state rather than
  total lifetime operations, without making an old sequence usable.
- Once the first fence activates, every writable or fully authoritative verifier
  honors the durable v0.52.0 feature/version floor; older tools can only enter
  explicitly limited historical read-only inspection where supported.
- Compaction requires sufficient stability evidence and preserves every
  potentially concurrent admitted head.
- "Sufficient" means acknowledgement from every active replica in the committed
  membership epoch.
- A governance quorum may retire a missing replica but cannot attest on that
  replica's behalf.
- A replica that acknowledges a frontier and later reveals an omitted prior
  change produces fork or equivocation evidence; the change is not silently
  merged into compacted history.
- A higher sequence number does not repair stale lineage: every event after the
  fence must include the acknowledged frontier in its causal ancestry.
- Replica state loss or cloning requires an explicit new incarnation rather
  than resuming the old identity from an uncertain watermark.
- Replica and locator churn cannot bypass actor/device aggregate resource
  limits or erase prior quota consumption.
- Disconnected replicas cannot collectively exceed an aggregate quota using
  locally observed counters: authoritative admission consumes disjoint signed
  escrow rights, and overdraw remains explicit quarantined conflict.
- Retirement never turns uncertainty about offline spending into reusable
  capacity: rights are surrendered, fenced by acknowledged spent state,
  invalidated by an explicit cutoff, or burned.
- Later governance may ratify a quarantined operation through new signed
  history, but cannot make the operation valid retroactively at its original
  causal position.
- Quota expiry is decided by causal/checkpoint order or an admitted timestamp
  authority; local wall-clock behavior and delivery delay cannot extend or
  retroactively invalidate a provably pre-expiry spend.

## Phase 5: Changes And Sealing

### v0.53.0 - Change Begin

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

### v0.54.0 - Change Revision Object

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

### v0.55.0 - Seal Command

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
- This release seals local draft history only; it does not claim protected-world
  proof admission or promotion readiness.

### v0.56.0 - Save, Amend, And Log

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
- Profiles requiring proof obligations unavailable at this milestone fail
  closed instead of silently downgrading to an unprotected save.

### v0.57.0 - Operation Ledger

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

### v0.58.0 - Undo

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

### v0.59.0 - Typed Ingest Enforcement

Goal: implement the v0.29.0 type-state contract across storage, worlds, changes,
proofs, and policy.

Deliverables:

- distinct untrusted, canonical, hash-verified, reference-derived,
  causally-closed, signature-verified, policy-admitted, and committed types;
- consuming transitions between each stage;
- no public constructors that skip stages;
- crate-boundary APIs that preserve stage and target binding;
- quarantine-to-trusted re-admission from original bytes only;
- diagnostic structural-validation results named separately from proofs;
- compile-fail tests for invalid stage reuse and bypass attempts.

Verification:

- `cargo test --workspace`
- API compile-fail test suite.

Exit criteria:

- Durable acceptance APIs cannot receive raw bytes, caller-supplied graph
  entries, or generic validation flags.
- Structural graph validation is never reported as cryptographic proof.

### v0.60.0 - Threshold Principals And Proof Of Possession

Goal: prevent one principal or rogue key from satisfying a multi-party policy.

Deliverables:

- principal and role identity committed independently from device keys;
- threshold counting by admitted principal or role;
- duplicate-principal and delegated-device collapse;
- proof-of-possession where required by the selected aggregate or
  multisignature design;
- role conflict and self-review hooks;
- rogue-key, duplicate-device, delegated-key, and threshold-inflation tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Adding devices or keys for one actor cannot increase an independent-principal
  threshold unless canonical governance explicitly creates a new principal.

### v0.61.0 - Signature Set Verification

Goal: verify admitted bounded signature sets against governance and policy.

Deliverables:

- transcript reconstruction from the exact target;
- cryptographic verification for every admitted signature;
- principal, role, key-epoch, revocation, and sequence checks;
- threshold evaluation over independent principals;
- deterministic duplicate and invalid-signature diagnostics;
- mixed-suite, stale-governance, partial-threshold, and invalid-component tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- A structurally valid envelope cannot satisfy policy unless its transcript,
  cryptography, governance authority, principal count, and epochs all verify.

### v0.62.0 - Checkpoint And Transition Transcript Integration

Goal: apply canonical transcripts to checkpoints, aliases, changes, merges, and
policy transitions.

Deliverables:

- signed checkpoint construction;
- signed alias compare-and-swap statement;
- signed sealed-revision and world-transition statements;
- signed policy, membership, key, and crypto epoch transitions;
- exact source/target frontier and result-root binding;
- cross-action, cross-realm, stale-frontier, and scope-substitution tests.

Verification:

- `cargo test -p sagnir-crypto`
- transcript vector validator.

Exit criteria:

- Every authoritative mutation type has an implemented canonical transcript and
  cannot reuse a signature from another mutation type.

### v0.63.0 - Provider Isolation Contract And Threat Gate

Goal: fix the capability, process, platform, and failure contract for isolated
key operations before v0.63.1 implements locked-memory and key-agent hardening.

Deliverables:

- reviewed extension of the v0.23.0 opaque provider abstraction for isolated
  signing, KEM, wrapping, AEAD, KDF, recovery, and secret-storage operations;
- narrow capability and IPC schema for each operation, with exact transcript,
  handle/session, suite, realm action, audience, expiry, and result bindings;
- explicit prohibition on general repository read/write, WAL, object, alias,
  checkpoint, filesystem-path, shell, network, or ambient user-session authority
  in a key agent;
- process, privilege, service-account, sandbox, OS-keystore, TPM, and HSM
  boundary options with a platform capability matrix;
- threat model for same-UID process access, ptrace/debug APIs, inherited handles,
  environment and command-line leakage, core dumps, swap, fork, suspend/resume,
  crash state, and compromised main-process requests;
- provider/key-agent lifecycle and deterministic failure state machine covering
  startup, authentication, request admission, cancellation, crash, reconnect,
  stale session, key retirement, and ambiguous completion;
- key confirmation and bounded wrong-key diagnostics that cannot become a
  secret oracle;
- disposable IPC/reference prototype isolated under the v0.92.1 rules; no
  prototype endpoint or magic is admitted for production authority;
- schema, capability-confusion, replay, overreach, provider crash, agent
  disconnect, fork, dump-configuration, wrong-key, and unsupported-platform
  fixtures.

Verification:

- `cargo test -p sagnir-crypto`
- isolated-provider protocol vectors;
- capability/dependency-direction compile-fail suite;
- provider/key-agent failure-state model.

Exit criteria:

- The implementation milestone has one narrow, reviewed authority and failure
  contract instead of inventing process isolation around an already-live API.
- A key agent cannot become a repository writer, broad store client, or ambient
  authority merely because it possesses a secret-provider handle.
- Unsupported platform controls are explicit and cannot silently inherit a
  stronger isolated-provider profile.

### v0.63.1 - Secret Memory And Isolated Provider Hardening

Goal: harden the v0.23.0 through v0.23.5 provider-secret, authenticated-release,
operation-capability, and plaintext-declassification boundaries with locked
memory, process isolation, and explicit portable-zeroization limits before the
vault exists.

Deliverables:

- implement the v0.23.1 provider-only handles for KEM secret keys/shared
  secrets, AEAD keys, DEKs, KEKs, KDF outputs, and recovery material without a
  caller-readable secret byte path;
- provider operations accept canonical operation capabilities and return public
  results or cross the explicit v0.23.1/v0.23.2 authenticated plaintext
  declassification boundary;
  provider-only bytes are never returned as `Vec<u8>`, closure byte views,
  printable/debuggable values, serializable DTOs, command arguments,
  environment values, or long-lived async task state;
- declassified authenticated plaintext uses audited bounded consumers and is
  treated as intentionally copyable after release; closure/lifetime scope may
  reduce exposure but cannot be a copy-prevention security claim;
- caller-owned import buffers have an explicit ownership/cleanup contract;
  scrubbing a provider copy is never described as scrubbing the caller's
  original bytes;
- audited secret arena or provider-owned memory uses locked/guarded pages where
  supported and reports unsupported page-lock, fork, dump, or swap controls;
- every normal and error path cleans provider scratch buffers through the
  admitted sanitization crate and provider-specific cleanup APIs;
- inventory and tests cover moves, stack/heap temporaries, allocator copies,
  FFI/provider buffers, registers where observable, panic/unwind/abort,
  process termination, core dumps, swap, fork, and suspend/resume limitations;
- no claim of complete immediate erasure from portable safe Rust: compiler
  optimization, previous stack frames, provider internals, registers, allocator
  history, swap, dumps, and terminated processes remain explicit residual risks;
- optional short-lived isolated signer/key-agent process keeps private keys out
  of the main `saga`/`sagad` process and returns only context-bound public
  outputs;
- isolated key-agent IPC implements v0.23.4 authenticated sequence, durable
  operation/result state, exact retry, and explicit ambiguous-completion
  semantics rather than trusting an in-memory `!Clone` request;
- key-agent execution/result journal remains physically and logically distinct
  from the v0.23.3 Sagnir authorization WAL and reconciles only through bound
  operation/request/result commitments; neither journal can impersonate the
  other's authority;
- privilege and same-UID attack model states when ptrace, process memory access,
  signals, or direct file mutation require service-account, sandbox, HSM/TPM,
  OS-keystore, or privileged-agent isolation;
- compile-fail tests reject `Clone`, `Debug`, serialization, unrestricted
  provider-secret extraction, key-handle/operation-capability confusion,
  thread/task escape where prohibited, and use after session close;
- fault-injection tests cover allocation failure, provider error, cancellation,
  panic boundary, agent disconnect, fork, dump configuration, and cleanup
  failure without producing an authorized result.

Verification:

- `cargo test -p sagnir-crypto`
- secret-handle compile-fail suite;
- provider scratch/copy lifetime audit fixtures;
- isolated key-agent integration tests where supported.

Exit criteria:

- Mainline callers authorize every admitted signing, KEM, wrapping, AEAD, KDF,
  recovery, and vault operation through opaque handles and one-use operation
  capabilities, not raw provider-secret buffers.
- Sagnir precisely states which copies it can clean and never equates best-
  effort zeroization with proof that all historical copies vanished.
- Provider failure, cleanup failure, or agent loss cannot produce partial
  authority or silently downgrade to an in-process raw-key path.

### v0.64.0 - Governed Rotation And Emergency Recovery

Goal: implement governance-backed rotation and recovery without creating a
second unauthenticated authority path.

Deliverables:

- ordinary and emergency key rotation transactions;
- threshold recovery authorization;
- mint and consume only v0.23.4 `RecoveryCapability` values from the admitted
  threshold ceremony; bootstrap, unlock, and authoritative capabilities cannot
  be converted into recovery authority;
- end-to-end recovery ceremony covering declaration, realm freeze or restricted
  mode, independent participant verification, share collection, threshold
  authorization, isolated provider execution, replacement-key admission,
  recipient and session invalidation, epoch advancement, checkpointing, and
  closure;
- out-of-band identity and realm-fingerprint confirmation for every recovery
  participant;
- no reconstructed secret written to ordinary files, command arguments, logs,
  shell history, or long-lived process memory;
- dry-run, abort, timeout, participant replacement, unavailable participant,
  compromised participant, lost device, and total-site-loss procedures;
- post-recovery verification that old administrator, device, recipient,
  session, and automation authority is rejected;
- recovery evidence package suitable for independent audit without disclosing
  recovery shares or replacement secrets;
- ownership transfer workflow;
- stale administrator and split-governance refusal;
- recovery event and checkpoint commitments;
- post-recovery key and policy epoch advancement;
- documentation that recipient removal cannot revoke already acquired keys.

Verification:

- `cargo test -p sagnir-crypto`
- governance and recovery state-machine tests;
- end-to-end software-fixture recovery ceremony.

Exit criteria:

- Emergency recovery is explicit, threshold-governed, auditable, and cannot be
  replayed as an ordinary administrative action.
- A documented ceremony can recover a realm from admitted loss scenarios,
  rotate every affected authority boundary, and prove that stale authority no
  longer works without exposing recovery material.

### v0.65.0 - Checkpoint Anchoring And Rollback Detection

Goal: make signed checkpoint comparison and witness anchoring operational.

Deliverables:

- checkpoint persistence and comparison;
- optional peer, removable-media, TPM, secure-enclave, or hardware witness
  anchors;
- witness classification as advisory evidence unless canonical governance
  explicitly assigns authority;
- witness quorum and administrative-domain diversity policy;
- split-view gossip and conflicting-witness evidence;
- highest-seen checkpoint tracking;
- restored-snapshot and deleted-suffix diagnostics;
- equivocation evidence retention without recursive fork expansion;
- missing-suffix and rollback diagnostics;
- witness unavailability and stale-witness behavior;
- restored snapshot, insufficient diversity, split view, conflicting witness,
  stale witness, and checkpoint-substitution tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-crypto`
- event-DAG model tests.

Exit criteria:

- Deleted suffixes, stale snapshots, and actor equivocation are detectable when
  compared with an admitted later checkpoint or witness.
- Sagnir documents the limits of purely local rollback detection honestly.
- Advisory witnesses strengthen rollback evidence but cannot redefine canonical
  realm validity.

### v0.66.0 - Target-Bound Verification Results

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

### v0.67.0 - Integrity Proof

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
- Authority graph classes enforce their admitted DAG rules, while dependency
  and impact cycles remain representable as deterministic SCCs.

### v0.68.0 - Proof Artifact And Soundness Suite

Goal: define exactly what each proof proves and make proof artifacts portable.

Deliverables:

- canonical proof artifact envelope;
- maximum encoded and decompressed proof bytes;
- maximum node count, tree depth, sibling count, causal-frontier width, and
  recursive composition depth;
- hash-operation, signature-operation, batch-fallback, and verification-time
  budgets;
- cancellation and deterministic resource-exhaustion results;
- inclusion and absence proofs;
- append-only consistency proofs;
- changed-cone and causal-closure proofs;
- completeness claim with explicit scope and assumptions;
- verifier-version binding;
- proof soundness and non-goal statements;
- malformed, truncated, substituted, scope-confusion, oversized, deep,
  high-frontier, decompression-bomb, and batch-fallback-cost tests.

Verification:

- `cargo test -p sagnir-proof`
- proof vector validator.

Exit criteria:

- Every proof kind states its target, assumptions, coverage, and limitations.
- A proof cannot claim full-world completeness when it covers only a changed
  cone or bounded chunk.
- Proof parsing and verification remain bounded independently of object or
  bundle limits.
- Private-realm proof artifacts are encrypted by default unless an intentional
  disclosure policy produces a scoped public artifact.

### v0.69.0 - Proof Cache And Incremental Verification

Goal: reuse verified immutable subtrees without accepting stale security state.

Deliverables:

- cache key bound to target root, verifier version, policy root and epoch,
  crypto epoch, and verified frontier;
- generation-number invalidation;
- persistent verified-subtree cache;
- changed-cone cache reuse;
- cache format generation and versioning;
- `saga fsck` integration for stale-cache diagnostics;
- deterministic cache deletion and rebuild;
- stale, substituted, partially written, and epoch-change cache tests;
- cache deletion and deterministic rebuild behavior;
- concurrent reader/rebuilder/publication schedule exploration proving a stale
  or partial generation cannot become authoritative.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-store`
- Loom or equivalent proof-cache publication schedule tests.

Exit criteria:

- Cached results can accelerate one-file changes in large worlds without
  surviving any relevant verifier, policy, crypto, or frontier change.
- Cache rebuild is deterministic, optional for integrity, and available only
  after this milestone admits the cache format.

### v0.69.1 - Typed Parameterized Obligation Format

Goal: represent policy requirements as canonical scoped objects before compound
admission depends on a fixed implementation bitmask.

Deliverables:

- versioned canonical obligation template body with obligation kind, schema
  version, realm, world/transition target, subject/principal class, causal
  frontier, policy root/epoch, crypto epoch where relevant, issuance context,
  parameters, composition, and evidence-consumption semantics; template and
  instance IDs/signatures are envelope fields excluded from this body;
- typed parameter schemas for threshold/count, accepted roles/principal classes,
  evidence/proof kind, witness class and independence, audience/purpose, path or
  object scope, freshness/revocation requirement, algorithm/provider assurance,
  resource bound, and other admitted built-in requirements;
- canonical discharge requirement identifies acceptable evidence object kinds,
  target/scope binding, signer/issuer authority, policy/crypto epoch, freshness
  evidence, minimum assurance, and multiplicity/independence rules;
- explicit composition operators for all/any/threshold and bounded nested
  groups, with canonical ordering, duplicate handling, maximum depth/count, and
  no ambiguous equivalent encodings;
- expiry and validity use causal/checkpoint or admitted v0.28.0 authority
  semantics, never an unauthenticated local wall clock;
- discharge result is target-bound and states satisfied, unsatisfied, stale,
  unavailable/unknown, malformed, revoked, or policy-incompatible with exact
  evidence references;
- unknown critical obligation kinds fail closed; optional advisory obligations
  are explicitly distinct and cannot be promoted to mandatory or vice versa by
  parser behavior;
- the existing bounded bitmask remains a derived in-memory summary for admitted
  parameter-free built-in classes only; it is not canonical policy language,
  cannot encode parameters, and cannot satisfy an obligation without verifying
  the canonical object;
- v0.69.2 obligation identity and signatures bind the canonical template body,
  instance context, parameters, and composition so a threshold, role,
  freshness, audience, target, or scope cannot be stripped, widened, or
  substituted;
- canonical/malformed vectors for parameter ordering, duplicate roles/evidence,
  threshold edges, nested composition, stale/revoked discharge, cross-target,
  cross-frontier, cross-policy, cross-audience, and bitmask/object mismatch;
- bounded parser, evaluator, and composition work integrated with v0.12.1.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-proof`
- independent obligation object/discharge vectors;
- Kani or equivalent bounded composition and state proof;
- corpus-backed obligation parser fuzz smoke.

Exit criteria:

- Policy can express parameterized threshold, role, freshness, audience, target,
  witness, and proof requirements without allocating a permanent bit per
  semantic variant.
- No derived bitmask or generic boolean can satisfy a canonical obligation
  whose parameters and discharge evidence were not verified.
- Obligation expiry and freshness remain deterministic under offline operation.

### v0.69.2 - Obligation Identity And Evidence Consumption

Goal: prevent caller-selected aliases, nested threshold structure, or evidence
reuse from changing which canonical obligations are actually discharged.

Deliverables:

- `ObligationTemplateId` is the admitted hash over domain
  `sagnir:obligation-template:v1` and the length-framed canonical template body
  containing parameters, composition, evidence-consumption policy, scope,
  epochs, and validity context but explicitly excluding every template/instance
  ID and signature field;
- `ObligationInstanceId` is the admitted hash over domain
  `sagnir:obligation-instance:v1` and exact length-framed
  `(template_id, issuance_operation_id, reserved_instance_ordinal_or_nonce)`;
  its own field and every signature are excluded from the preimage;
- template/instance hash algorithm and suite/version are explicit and cannot be
  inferred from digest length; canonical test vectors freeze field framing,
  ordering, and domain separation;
- `IssuanceOperationId` is preallocated before the issuing transition through
  the v0.23.3 durable reservation lifecycle under domain
  `sagnir:obligation-issuance-operation:v1`, binding realm, issuing authority,
  actor/device and replica incarnation, durable sequence, purpose, and an
  independent 256-bit random nonce;
- repeated byte-identical obligations that must remain distinct reserve a typed
  instance ordinal or nonce under that issuance operation before the instance
  ID or containing transition is computed; arbitrary caller-selected aliases
  are never obligation identity;
- the issuing signature binds both IDs, the complete canonical template body,
  issuance operation ID, containing transition, reserved ordinal/nonce, target,
  and policy context; the transition binds issuance operation ID, template ID,
  instance ID, ordinal/nonce, target, and policy context, and verifiers
  recompute both IDs before accepting the signature;
- canonical composition states whether one evidence object may discharge more
  than one child, whether consumption is exclusive, and which evidence kinds
  permit declared reuse;
- threshold evaluation tracks canonical principal, actor/device, key, witness,
  proof, review, and evidence identities across the entire nested obligation
  graph, not only within one immediate group;
- the same principal, key, witness, approval, or proof cannot be counted more
  than once through duplicate leaves, aliases, nested groups, alternate paths,
  or wrapper evidence unless that obligation schema explicitly permits and
  scopes reuse;
- independence requirements state which identity dimensions must differ and
  cannot be satisfied by multiple keys/devices controlled by one principal
  unless policy explicitly selects that counting model;
- canonicalization and normalization never merge obligations that differ in
  evidence exclusivity, reusable classes, consumption scope, instance identity,
  threshold counting, or independence semantics;
- discharge transcript records the deterministic evidence-to-obligation
  assignment, consumed/reused status, principal-count projection, alternatives
  considered, and unresolved conflicts;
- deterministic matching/allocation algorithm has bounded work and canonical
  tie ordering; over-budget matching returns `Unknown`/`Incomplete`, never a
  convenient partial threshold;
- self-including/precomputed ID, caller-chosen ID, template/instance domain
  confusion, omitted/substituted/reused issuance operation or ordinal,
  transition-hash cycle regression, signature substitution,
  duplicate evidence, nested-group double count, one-principal-many-keys,
  wrapper alias, canonical-merge, assignment-order, and over-budget tests.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-proof`
- independent obligation identity and evidence-assignment vectors;
- Kani or equivalent bounded nested-threshold proof.

Exit criteria:

- Obligation template and instance identities use non-circular, domain-separated
  preimages with every derived ID and signature excluded; the preallocated
  issuance operation breaks any instance-ID/transition-hash dependency, and
  distinct instances require an explicitly governed reservation rather than an
  arbitrary alias.
- Evidence reuse and independence are explicit canonical policy, and nested
  composition cannot count one authority more times than permitted.
- Different evaluation orders produce the same bounded evidence assignment and
  discharge result.

### v0.70.0 - Compound Policy Admission

Goal: combine canonical realm validity and local acceptance into one
non-contradictory admission result.

Deliverables:

- canonical realm/world policy evaluation;
- local acceptance policy evaluation as a separate stricter layer;
- validated compound admission result combining integrity, signatures, causal
  closure, policy decision, and discharged obligations;
- consume that exact typed result to mint a v0.23.4
  `AuthoritativeOperationCapability` for one signing, rotation, promotion, or
  publication action; no earlier or partial validation state can mint it;
- canonical v0.69.1 obligation template/instance evaluation plus v0.69.2
  non-circular identity, evidence-consumption, independence, and typed discharge
  results;
- impossible-state prevention for `allow` with unsatisfied obligations;
- `Allow` is unconstructible while any required obligation is absent, failed,
  stale, target-mismatched, or evaluated under another policy, crypto, realm,
  checkpoint, or frontier context;
- evaluator-version and policy-root binding;
- activate realm-selected v0.12.4 protocol work-cost tables only through a
  signed canonical policy transition with explicit frontier/checkpoint;
- explicit denial source and missing-obligation diagnostics;
- invalid policy tests;
- exhaustive or bounded verification of all 65,536 derived built-in summary
  bit patterns and impossible aggregate-result states, without treating the
  summary as canonical parameterized policy.

Verification:

- `cargo test -p sagnir-policy`
- Kani or equivalent compound-admission state proof.

Exit criteria:

- Draft, review, staging, and production policies can differ locally.
- Relaxed behavior is explicit through profile selection; strict environments
  can require signatures, evidence, review, and promotion checks.
- Promotion code consumes one validated admission result rather than checking
  independent enums that can contradict each other.
- Post-genesis authoritative provider operations have no minting path outside
  the consumed compound-admission result.

### v0.71.0 - World Transition Object

Goal: represent every world-state move as an immutable, signed transition.

Deliverables:

- TLA+/PlusCal or equivalent transition, merge, and promotion model completed
  before transition mutation code;
- all parent world-state IDs;
- causal frontier before the operation;
- selected merge bases;
- base, source, target, and result state roots;
- deterministic transition and merge algorithm versions;
- conflict and resolution references;
- proof, policy, and crypto commitments;
- canonical signed transcript integration.

Verification:

- bounded transition and merge model check;
- `cargo test -p sagnir-world`
- `cargo test -p sagnir-crypto`

Exit criteria:

- No world mutation can be represented as an unexplained alias overwrite.

### v0.72.0 - Deterministic Merge Base And Fast-Forward

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

### v0.73.0 - Deterministic Tree Merge

Goal: create reproducible source-state results for divergent worlds.

Deliverables:

- three-way tree merge;
- deterministic path ordering;
- add/add, delete/modify, rename/delete, rename/rename, directory/file,
  type-change, executable-bit, and symlink-target handling;
- case-folding, Unicode-normalization, reserved-name, trailing-dot, and
  platform-unmaterializable conflict detection;
- target-platform capability set committed to materialization preflight rather
  than silently changing the canonical merge;
- binary and oversized-file merge policy;
- merge algorithm version committed into the transition;
- independent replay and fixture tests.

Verification:

- `cargo test -p sagnir-worktree`
- `cargo test -p sagnir-world`

Exit criteria:

- Identical merge inputs and algorithm versions produce identical result trees
  on every supported platform.
- A canonical merged tree that cannot be materialized on a target platform
  remains valid canonical state but produces an explicit portability conflict
  instead of path loss or substitution.

### v0.74.0 - Conflict And Resolution Objects

Goal: preserve unresolved source, policy, compartment, and evidence conflicts
as explicit state.

Deliverables:

- typed conflict object;
- explicit resolution object;
- source-text, rename, type, policy, compartment, and evidence conflict kinds;
- case-folding, Unicode normalization, reserved-name, directory/file,
  rename/delete, rename/rename, and symlink-target conflict kinds;
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

### v0.75.0 - Promotion Preflight

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

### v0.76.0 - Promotion Commit

Goal: move proven source state between worlds.

Deliverables:

- `saga promote`;
- signed promotion statement and canonical event;
- deterministic later compilation into a fact after fact formats are admitted;
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
- Promotion authority does not depend on the later fact envelope.

## Phase 7: Facts And Evidence

### v0.77.0 - Local Fact Envelope

Goal: represent bounded evidence without blurring cryptographic, observed,
derived, and heuristic trust classes.

Deliverables:

- fact ID;
- subject and predicate model;
- explicit trust class: attestation, direct observation, deterministic
  derivation, or heuristic inference;
- evidence references;
- confidence only for heuristic inference;
- derivation rule and source commitments for deterministic facts;
- signer and transcript references for attestations;
- causal link list;
- recorded-validity and current-trust status;
- bounds tests.

Verification:

- `cargo test -p sagnir-fact`

Exit criteria:

- Facts can be validated before entering the local fact log.
- A confidence score cannot make a heuristic or observation equivalent to a
  cryptographic attestation.

### v0.78.0 - Fact Log

Goal: append and replay local facts.

Deliverables:

- fact log frame;
- fact append;
- fact replay;
- revocation and supersession records;
- key-compromise and policy-epoch trust reevaluation;
- distinction between valid-when-recorded and currently trusted;
- duplicate fact behavior;
- corrupt fact log tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-store`

Exit criteria:

- Local facts survive process restart and corruption is detected.

### v0.79.0 - Test Evidence Recording

Goal: bind command results to sealed revisions or state roots.

Deliverables:

- `saga test record`;
- executable identity and tool version;
- argument commitment;
- environment allow-list and commitment;
- input state root;
- dependency and toolchain commitments;
- exit code capture;
- timeout and sandbox policy;
- output capture bounds;
- bounded output and artifact digests;
- tests for pass, fail, and timeout.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- Test evidence binds the executable context, input state, relevant
  environment, result, and bounded artifacts without treating terminal text as
  proof.

### v0.80.0 - Review Evidence Recording

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

### v0.81.0 - Diagnostic Why Query

Goal: provide an explicitly non-authoritative local provenance preview before
the canonical event/fact/query foundations in v0.83.0 through v0.85.1 exist.

Deliverables:

- path provenance query;
- change-to-path index;
- local provenance-record lookup that returns a `ProvenancePreview`, never a
  canonical `Fact`;
- `saga why`;
- output is labelled `diagnostic preview` and binds available local source IDs
  but makes no completeness, minimality, policy-discharge, or canonical fact
  claim;
- diagnostic results cannot satisfy policy, create proof/fact objects, authorize
  promotion, or be signed as an authoritative explanation;
- stable explanation output tests.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- A user can inspect available local provenance without mistaking the result for
  the canonical snapshot-bound query introduced by v0.85.1.
- Missing indexes, event/fact foundations, or budget exhaustion produce
  explicit incomplete diagnostics, never authoritative absence.

### v0.82.0 - Diagnostic Local Impact Traversal

Goal: preview local blast radius without claiming complete causal impact before
the structured event log, fact compiler, indexes, and query contract exist.

Deliverables:

- forward causal traversal;
- `DiagnosticTaintFinding` preview;
- `QuarantineCandidate` preview;
- `saga impact`;
- output is labelled `diagnostic preview`, distinguishes discovered edges from
  unknown/incomplete traversal, and cannot drive policy or automatic quarantine
  authority;
- tests for key, dependency, change, fact, and model identifiers.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-cli`

Exit criteria:

- Sagnir can preview downstream local state that may need review or quarantine;
  authoritative `must affect`/`may affect`, SCC, completeness, and snapshot
  semantics activate only through v0.85.1.
- Types and APIs introduced here cannot be passed as canonical `Fact` values;
  that name and authority are reserved for v0.84.0 compiler output.

## Phase 8: Causal Memory And Explanation

### v0.83.0 - Structured Event Log

Goal: separate noisy command events from stable canonical facts.

Deliverables:

- diagnostic command-event envelope distinct from the authoritative v0.26.0
  signed event envelope;
- diagnostic event kind registry;
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
- Diagnostic command events cannot substitute for signed authoritative state
  events.

### v0.84.0 - Fact Compiler

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

### v0.85.0 - Causal Graph Indexes

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

### v0.85.1 - Deterministic Fact Language And Snapshot Query Contract

Goal: bound causal derivation and make every explanation/query reproducible
against one immutable source-state snapshot before user-facing explanations
become authoritative evidence.

Deliverables:

- canonical facts are derived only from admitted signed events and verified
  canonical objects, retaining every source ID, compiler/evaluator version,
  policy/crypto epoch where relevant, and derivation rule ID;
- a versioned stratified, terminating Datalog-like subset or equivalently
  reviewed monotone fact language with explicit type, arity, ordering, and
  resource semantics;
- recursive derivation is admitted only within declared strata and bounded
  fixpoint work; negation is stratified and cannot observe a partially computed
  lower stratum or uncommitted remote state;
- budget exhaustion, cancellation, missing input, unavailable proof, or
  interrupted fixpoint yields `Unknown`/`Incomplete` with no authoritative
  derived fact-set root; partially derived facts remain isolated scratch state
  and cannot satisfy policy, obligations, proofs, promotion, or canonical
  indexes;
- non-monotone aggregates, unrestricted function symbols, dynamic code,
  wall-clock predicates, environment reads, network calls, and unbounded
  recursion are excluded from canonical fact evaluation;
- semantic causal cycles are either refused by the rule stratifier or represented
  as deterministic dependency SCCs; an acyclic event DAG alone is not treated
  as proof that fact derivation terminates;
- rebuildable subject/predicate, path/symbol, reverse-provenance, world-frontier,
  and forward-impact projections with versioned schemas and deterministic
  rebuild roots;
- query result transcript binds realm, snapshot/state root, causal frontier,
  fact/index roots, query kind, normalized query, deterministic plan digest,
  evaluator, v0.12.3 accounting, and v0.12.4 protocol-fixed or signed canonical-
  realm-selected cost-table versions, policy/redaction context, original
  ceilings, deterministic consumed counters, evidence IDs, result digest,
  uncertainty class, cancellation state, and missing/redacted evidence;
- pagination cursor is authenticated and binds the same snapshot, plan, sort
  order, filters, audience, budget class, and expiry; cursor expiry uses causal/
  checkpoint state or admitted v0.28.0 authority evidence rather than local
  wall clock, and a cursor cannot continue against a newer snapshot, stale
  authority context, or wider disclosure scope;
- `saga why` computes deterministic bounded top-k minimal causal evidence sets
  under a canonical ordering and declared maximum candidates, search nodes,
  depth, deterministic v0.12.3 work counters, and logical memory units; a local
  monotonic deadline can only cancel with `Incomplete` and no fact/query root;
- why output states whether minimality and enumeration completeness were proven
  within the searched finite graph; over-budget or truncated alternatives carry
  explicit continuation/truncation and `Unknown`/`Incomplete` markers rather
  than implying that omitted explanations do not exist;
- `saga explain` replays the exact decision/transition transcript, obligations,
  discharge evidence, missing requirements, and redactions;
- `saga impact` traverses typed forward dependencies, distinguishes `must
  affect` from `may affect`, and collapses dependency SCCs deterministically;
- borrowed/zero-copy query views are permitted only over immutable fully
  validated owned bytes such as `Arc<[u8]>` or sealed authenticated pack pages;
  direct mmap of attacker-mutable/truncatable files is not an admitted trusted
  query boundary because it can change after validation or cause `SIGBUS`;
- immutable sorted index segments have validated offset tables, bounded slices,
  page checksums/authenticated roots, and generation binding before borrowed
  access;
- typed parameterized obligation and discharge interactions, stratification
  boundaries, fixpoint work, SCCs, top-k tie ordering, exponential-alternative
  truncation, cursor replay, causal/time expiry, snapshot mismatch, index
  corruption, mutable mmap substitution, partial-fact policy bypass, and budget
  exhaustion receive exhaustive/bounded tests where practical; exhaustive
  16-bit summary tests remain the v0.70.0 aggregate-state check and are not a
  substitute for canonical obligation-object vectors.

Verification:

- `cargo test -p sagnir-fact`
- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-proof`
- Kani or equivalent obligation and evaluator-state bounded proofs;
- production/reference evaluator differential suite;
- query transcript and pagination vectors.

Exit criteria:

- Fact derivation terminates deterministically within admitted budgets and
  cannot construct `Allow` while required obligations remain unsatisfied.
- Incomplete fixpoints and truncated explanation searches are first-class
  unknown results and cannot be consumed as complete facts or proven absence.
- Sequential and parallel evaluation under one cost-table version produce the
  same result, truncation boundary, completion status, and logical counters;
  deadline cancellation never publishes a fact or query root.
- Every why, explain, impact, trace, or paginated query result identifies the
  exact immutable snapshot and evidence from which it was derived.
- Rebuildable indexes and borrowed views improve performance without becoming
  an alternate source of truth or trusting mutable mapped storage.

### v0.86.0 - Explanation Object

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

### v0.87.0 - Explain Command

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

### v0.88.0 - Trace Command

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

### v0.89.0 - Context Packs

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

### v0.90.0 - Ask Query Scaffold

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

### v0.91.0 - Vault Key Hierarchy And Derivation

Goal: define cryptographic key separation before encrypted bytes are written.

Deliverables:

- key-encryption-key, realm, compartment/world, private-locator, private-index
  commitment, and erasure-unit key hierarchy;
- explicit erasure-unit granularity: object, path group, compartment, epoch, or
  retention class;
- an erasure unit owns one encryption instance and its DEK, not the immutable
  logical object or semantic commitment;
- one logical object referenced by scopes with incompatible retention,
  compartment, legal-hold, or redaction policy uses separate encryption
  instances and independently wrapped DEKs;
- copy-on-write encryption instance creation when an existing shared instance
  would cross an erasure-policy boundary;
- random independently generated data-encryption key for every declared erasure
  unit;
- every locally persisted DEK uses either an independently destroyable
  per-erasure-unit KEK or hardware/software key slot, or a parent wrapping epoch
  dedicated to a set whose complete surviving DEKs can be transactionally
  rewrapped during erasure;
- directly wrapping an erasable DEK under a long-lived surviving compartment,
  realm, recipient, or recovery key is insufficient unless erasure rotates that
  parent wrapping epoch and rewraps every surviving DEK before the old epoch is
  destroyed;
- ordinary filesystem unlink, overwrite, truncate, free-space discard, TRIM, or
  deletion of a wrapped-key record is never evidence that the wrapper is
  unrecoverable from journals, snapshots, copy-on-write blocks, SSD
  wear-leveling, filesystem history, crash dumps, or forensic images;
- local wrapper-erasure assurance records the storage and key-slot mechanism,
  independently destroyable KEK or wrapping epoch, surviving-DEK rewrap root,
  old-epoch destruction evidence, and residual media assumptions;
- one DEK may have multiple wrappers only for recipients or recovery authorities
  admitted to the same erasure unit and compatible retention policy;
- erasure succeeds only after every wrapper, recovery share, escrow copy, and
  admitted derivation path capable of recovering that DEK is destroyed or
  cryptographically revoked;
- removing one recipient, path, or reference cannot erase an encryption
  instance still required by another admitted scope;
- retention and legal-hold conflicts fail closed and block redaction or DEK
  destruction;
- v0.23.0 OS-CSPRNG boundary used for every DEK, private-locator key, wrapping
  key, salt, nonce seed, and randomized encryption input;
- no fallback from failed operating-system entropy to counters, timestamps,
  deterministic test RNGs, or ordinary pseudorandom generators;
- fork/reseed, VM-clone, suspend/resume, and early-boot handling inherited from
  the cryptographic provider boundary;
- reviewed puncturable-key construction allowed only as a separately admitted
  alternative to independently wrapped keys;
- surviving realm, compartment, epoch, or recipient ancestor keys cannot
  rederive a destroyed erasure-unit data key;
- domain-separated derivation only for non-erasable purpose keys such as
  metadata indexing, WAL authentication context, wrapping context, and
  proof-disclosure context;
- private-locator key lifecycle separated from semantic commitments,
  encryption-key rotation, and data-key rotation;
- private-index commitment keys are distinct from private-locator, encryption,
  wrapping, signature, proof-disclosure, and erasure keys and have their own
  governed epoch, compromise, rotation, retention, and retirement lifecycle;
- ordinary rekeying does not require semantic commitment or private-locator
  changes;
- key identifier and crypto epoch binding;
- key-destruction provider capability contract describing operation identity,
  idempotency, post-crash status query, confirmation evidence, refusal,
  unreachable and permanently ambiguous outcomes; no provider is used for an
  erasure claim before v0.122.0 implements the durable operation;
- bounded KDF and derivation parameter admission;
- cross-purpose key-reuse rejection;
- known-answer derivation and wrapping vectors;
- erased-key reconstruction tests using every surviving key and metadata
  combination;
- journaled filesystem, copy-on-write snapshot, retained block image,
  recovered-deleted-wrapper, surviving parent key, incomplete rewrap, and old
  wrapping-epoch recovery tests;
- shared blob, shared tree, overlapping compartment, multiple wrapper,
  incompatible retention, legal hold, copy-on-write, and partial redaction
  tests;
- semantic commitment and private-locator stability tests across ordinary
  encryption rekeying;
- unavailable, repeated, compromised, fork-cloned, and VM-cloned entropy
  fault-injection tests.

Verification:

- `cargo test -p sagnir-crypto`
- key-derivation vector validator.

Exit criteria:

- Compromise or misuse of one derived purpose key does not silently authorize a
  different cryptographic purpose.
- Destroyed erasure-unit data keys cannot be reconstructed from retained
  ancestor keys, wrapped-key metadata, private-locator keys, or later crypto
  epochs.
- Local wrapper deletion counts as cryptographic erasure only when an
  independently destroyable local KEK/key slot is proven destroyed or the
  parent wrapping epoch is rotated, every surviving DEK is rewrapped, and the
  old epoch is proven unavailable; otherwise status is residual or unverified.
- Logical-object deduplication never merges encryption instances whose erasure,
  retention, legal-hold, compartment, or recipient policies are incompatible.
- Private lookup locators and ciphertext confidentiality can rotate under
  separate governed lifecycles while immutable semantic identity remains
  unchanged.
- Vault creation and key rotation fail before persistence when production
  entropy requirements are not met.

### v0.92.0 - AEAD Nonce And Live Key Session Safety

Goal: prevent nonce reuse, wrong-key acceptance, and unsafe live-key handling
before encrypted pages or objects are written.

Deliverables:

- instantiate the v0.31.1 clone-safe encrypted WAL activation contract before
  enabling encrypted WAL write or authenticated replay;
- admitted misuse-resistant or nonce-robust AEAD evaluation;
- instantiate v0.23.2 whole-record authenticate-before-release for every
  admitted AEAD suite; no provider open API emits pre-tag plaintext, while
  chunked encrypted operation remains disabled until v0.92.2;
- crash-safe nonce uniqueness or per-record key/nonce derivation;
- concurrent-writer allocation rules;
- counter rollback and exhaustion behavior;
- restored-snapshot and undetected filesystem/VM clone nonce safety;
- key confirmation and deterministic wrong-key refusal;
- process memory, core-dump, fork, swap, and suspend policy;
- optional privilege-separated local key agent integration;
- nonce-reuse, crash, snapshot restore, silent clone, repeated-randomness,
  concurrent writer, exhaustion, and wrong-key tests;
- encrypted WAL known-answer/differential vectors binding exact suite, frame,
  key epoch, nonce/per-record key, log context, and authentication result.

Verification:

- `cargo test -p sagnir-crypto`
- AEAD known-answer and misuse vector suite;
- nonce-allocation state-machine model.

Exit criteria:

- A crash, concurrent writer, restored filesystem snapshot, undetected clone,
  or counter rollback cannot cause two records under one key to reuse a
  forbidden nonce.
- Wrong keys fail authentication without exposing parsed plaintext.
- Whole-record bad tags release zero plaintext; chunked operation is unavailable
  until v0.92.2 admits exact per-suite nonce/key and retry rules.
- Encrypted WAL remains unavailable unless this milestone's construction and
  clone-safety evidence satisfy v0.31.1.

### v0.92.1 - Private Index Format Admission Stop

Goal: resolve private-index identity, unique representation, compartment-root
composition, and placement ownership before v0.93.0 admits durable formats.

Deliverables:

- reviewed algorithm/format admission document with no production
  implementation or compatibility claim;
- disposable non-production reference prototypes are required for canonical
  vectors, parser experiments, fuzz targets, models, and benchmarks;
- prototypes cannot write or migrate durable realm data, cannot be reachable
  from production crate feature graphs or release binaries, cannot emit
  authoritative manifests or signatures, and carry no wire/storage
  compatibility promise;
- prototype bytes use an unmistakable experimental magic/version and are
  rejected by production decoders;
- after admission, prototypes are deleted/replaced or explicitly promoted
  through reviewed production implementation, independent vectors, migration
  policy, and release-gate coverage; admission does not silently bless
  prototype code;
- canonical logical entry key
  `(locator_epoch, private_locator, semantic_commitment,
  encryption_instance_id)`;
- one semantic commitment may own multiple policy-separated encryption
  instances without aliasing, overwriting, or linear instance scans;
- canonical encryption-instance identifier bytes:
  `(format_version, hash_algorithm, 32-byte digest)`, where the digest is the
  admitted domain-separated hash over length-framed format version, realm ID,
  opaque compartment handle or neutral-domain handle, semantic commitment,
  erasure-unit ID and class, preallocated creation-operation ID, and an
  independently random 256-bit instance nonce from the v0.23.0 OS-CSPRNG;
- the creation-operation ID reuses the v0.23.3 shared durable reservation
  lifecycle under its own domain and exists before the signed instance-creation
  transition, preventing a circular transition-hash dependency; that transition
  binds the resulting instance ID, every hash input, actor/replica authority,
  policy root, causal parents, and resulting semantic/index roots;
- canonical creation-operation ID bytes
  `(format_version, hash_algorithm, 32-byte digest)`, where the digest is a
  domain-separated hash over realm ID, actor/device principal, replica ID and
  incarnation, reservation sequence, independently random 256-bit reservation
  nonce from the v0.23.0 OS-CSPRNG, operation kind, and intended compartment or
  neutral-domain handle;
- reservation sequence is monotonically allocated under the replica's
  concurrent-writer lock and the random nonce prevents restored/cloned local
  state from producing an alias; both are bound into the signed replica event
  chain;
- before use, a creation-operation ID is durably recorded as an authenticated
  `Reserved` operation with exact intended purpose, policy/membership epoch,
  causal frontier, and expiry/no-expiry rule;
- reservation lifecycle is `Reserved`, `Consumed`, or `Cancelled`; consumption
  atomically binds exactly one instance-creation transition, while cancellation
  is an authenticated transition preserving the ID and reason;
- abandoned, crashed, cancelled, expired, or uncertain reservations are never
  reusable for another operation; recovery may idempotently resume the exact
  intended operation or cancel it, but cannot recycle the ID;
- replay of an already consumed reservation is idempotent only for the exact
  canonical creation transition and result; any different transition, instance,
  policy, or compartment is reservation equivocation and is rejected;
- duplicate reservation IDs with different canonical reservation records,
  nonce/sequence rollback, cloned replica state, two-process allocation, crash
  before/after reservation durability, and cancel/consume races produce
  explicit conflict evidence rather than aliasing;
- instance nonces and IDs are never derived from plaintext, clocks, counters,
  storage positions, ciphertext, DEKs, wrapping keys, or local process IDs;
- collision and duplicate handling: an already admitted byte-identical
  instance ID is idempotent only for the exact same creation operation and
  canonical fields; any differing operation or fields produce a security
  conflict and no index overwrite;
- lifecycle state, erasure-policy class binding, and maximum instance fanout;
- instance ID remains stable across DEK rewrap, ciphertext re-encryption,
  recipient changes that preserve the erasure unit, repacking, receipt renewal,
  relocation, and endpoint projection changes;
- a new instance ID is mandatory when creating a new erasure unit, crossing an
  incompatible retention/legal-hold/redaction/recipient policy boundary,
  reintroducing redacted content, replacing an erased instance, or intentionally
  creating an independently erasable copy;
- duplicate-semantic-identity creation and encryption-instance creation are
  separate resource events: a new instance for an existing commitment consumes
  no duplicate-identity right but must consume separately governed
  instance-fanout capacity;
- offline instance creation uses distinct signed escrow rights or another
  reviewed bounded offline admission mechanism; duplicate-identity rights and
  instance-fanout rights are non-interchangeable;
- history-independent authenticated-index algorithm decision and proof/model
  that one canonical entry set has one logical root across insertion, deletion,
  merge, split, union, rebalance, and bulk-build order;
- bounded normalization, update, proof, and rebuild amplification decision;
- explicit fallback decision to a uniquely represented trie or key-derived
  tree if the candidate B+ tree cannot satisfy unique representation within
  admitted amplification budgets;
- one logical root and private-index commitment-key epoch per compartment;
- canonical opaque compartment-handle bytes
  `(format_version, handle_epoch, 32-byte digest)`, where the digest is a
  domain-separated keyed hash over realm ID, internal compartment ID,
  independently random 256-bit handle nonce, handle epoch, and purpose;
- handle collision admission compares the encrypted internal compartment
  binding; equal bytes for a different compartment or nonce are a security
  conflict and never alias two roots;
- compartment handles rotate through a signed realm-manifest transition binding
  old/new handles, unchanged compartment identity and root, reason, epoch,
  policy, and checkpoint; old handles remain historical references but are
  retired from current discovery;
- handle rotation limits future correlation but cannot hide linkages already
  observed by recipients or storage endpoints; cross-epoch translation mappings
  remain encrypted and audience-scoped;
- canonical authenticated realm manifest over opaque compartment handles and
  compartment logical-root commitments;
- the realm manifest uses a fixed-depth or equivalently count-hiding
  authenticated map so a compartment-scoped inclusion/consistency proof does
  not reveal other compartment locators, entry counts, tree shape, names, or
  membership;
- partial-access proof format binds realm, manifest epoch, opaque compartment
  handle, compartment logical root, commitment-key epoch, policy root,
  checkpoint, and authorization scope;
- independent compartment-root migration, compromise recovery, and
  commitment-key rotation without requiring access to unrelated compartments;
- canonical signed logical-root manifests are shared authoritative
  realm/compartment state;
- canonical projection function from one admitted semantic-ledger state and
  projection-version to the complete sorted set of forward and reverse logical
  index entries, including every semantic commitment, encryption instance,
  locator epoch, object kind, erasure-policy class, and compartment root;
- projection replay uses v0.12.3 deterministic work accounting, the v0.12.4
  protocol-fixed or signed canonical-realm-selected evaluator cost table, and
  canonical partition/order rules so sequential and parallel evaluators produce
  identical roots, counters, truncation/refusal boundaries, and completion
  status;
- every logical-root manifest binds projection version, semantic-state root,
  forward root, reverse root, total entry/instance counts, and either a
  canonical full-rebuild proof or a chain of deterministic delta-transition
  proofs from the prior admitted manifest;
- for 1.0, a canonical full-rebuild proof is an authenticated deterministic
  replay certificate, not a succinct or zero-knowledge proof: it binds
  projection version and executable evaluator digest, exact semantic-ledger
  checkpoint/range, ordered Merkle-chunk roots and counts, declared resource
  bounds, independently recomputed forward/reverse roots and counts, transcript
  root, verifier result, and checkpoint;
- verification obtains or already possesses every ledger chunk committed by
  the certificate, validates canonical bytes/signatures/causal admission,
  executes the frozen projection function, and compares independently produced
  roots/counts with the manifest; trusting the publisher's computed root or
  transcript without replay is insufficient for a full-view verifier;
- rebuild-certificate soundness assumes collision and second-preimage
  resistance of admitted hashes, unforgeability of admitted signatures,
  canonical decoding, deterministic evaluator execution, complete ledger
  availability for the bound range, and correct verifier implementation; it
  does not claim succinct-proof soundness;
- delta proof binds exact added, removed, and retained ledger transitions,
  prior/new semantic roots, prior/new index roots, and projection output; it
  cannot omit, invent, duplicate, or substitute an entry;
- delta certificates are canonical replayable transition transcripts binding
  prior rebuild/checkpoint certificate, ordered admitted ledger transitions,
  affected entry keys and old/new values, unaffected authenticated range
  boundaries, evaluator digest, resource counters, and resulting roots/counts;
- format and policy impose hard maximum delta-chain length, cumulative delta
  bytes, transitioned ledger entries, affected index entries, verification
  work, and logical memory units under v0.12.3 deterministic accounting and the
  v0.12.4 protocol-fixed or signed canonical-realm-selected cost table; elapsed
  time is operational cancellation only, and deterministic over-limit chains
  require a new full rebuild certificate before further authoritative
  publication;
- mandatory full-rebuild cadence is bound by maximum admitted checkpoint gap,
  delta count, cumulative bytes/work, projection-version change, evaluator
  change, detected inconsistency, witness policy, and protected-world policy,
  whichever triggers first;
- delta-chain compaction never discards the last admitted full-rebuild
  certificate, manifests/checkpoints required for historical verification, or
  evidence of an invalid or equivocal projection;
- full-view verifiers independently replay the projection or validate the
  complete delta/rebuild proof before signing or accepting a manifest;
- partial-access recipients can verify their compartment inclusion,
  consistency, authorization, and signer/witness policy, but cannot independently
  prove completeness of hidden compartments; UI and proof results state this
  trust boundary explicitly;
- protected policy may require threshold full-view projection witnesses from
  independent governance roles before a manifest is authoritative for
  partial-access recipients;
- canonical projection-witness statement and signing transcript bind realm,
  compartment/realm-manifest scope, witness principal and key epoch, projection
  version/evaluator digest, semantic-state root/checkpoint, forward/reverse
  roots and counts, rebuild/delta transcript root, replay mode, completed
  resource counters, policy root/epoch, nonce, prior witness statement, and
  signature;
- a full-replay witness independently obtains and executes the committed ledger
  projection; an evidence-validation witness verifies a supplied replay
  certificate without independent ledger acquisition, and policies distinguish
  these non-equivalent assurance levels;
- threshold policy declares required full-replay/evidence-validation counts,
  principal independence, administrative/operator domain diversity, hardware or
  process isolation where required, and prohibited shared control;
- witness enrollment inherits governance and key-transparency controls; one
  actor, device, organization, beneficial operator, or administrative domain
  cannot satisfy multiple independent slots through nominal identities;
- witness key rotation, revocation, compromise, retirement, stale checkpoint,
  conflicting statement, and equivocation preserve immutable evidence and
  trigger fail-closed threshold reevaluation;
- projection evaluator implementations have stable implementation identity,
  build/provenance digest, supported projection-version range, and admission
  status distinct from the semantic projection version and executable evaluator
  digest already bound by replay evidence;
- implementation disagreement never resolves by publisher preference, witness
  majority, or arrival order: the manifest and derived state are quarantined,
  both canonical transcripts and roots are preserved, and signed disagreement
  evidence identifies implementation/build digests, inputs, resource bounds,
  outputs, and checkpoint;
- an evaluator defect is remediated through a signed security/admission
  transition naming affected projection versions, evaluator digests,
  checkpoints and manifests, defect classification, replacement implementation
  and projection version, replay requirements, and quarantine/release policy;
- remediation preserves original ledger bytes, manifests, witness statements,
  signatures, and replay certificates as historical evidence; corrected roots
  are new signed projection-version transitions and never rewrite the roots
  that historical actors actually signed;
- threshold unavailability yields `projection assurance unavailable` and
  quarantine/refusal for policies requiring witnesses; it never lowers the
  threshold or treats publisher signatures as witness substitutes;
- witnesses sign only after their declared replay mode succeeds, and concurrent
  witness statements over different roots at one bound checkpoint produce
  witness-equivocation evidence;
- a malicious authorized publisher that omits or invents an entry produces
  invalid projection evidence or publisher/witness equivocation evidence rather
  than an apparently valid complete index;
- ciphertext placement and reverse-resolution manifests are explicitly scoped
  to one replica incarnation, device, or storage endpoint and are never
  canonical shared semantic state;
- placement projection identity binds endpoint, replica/device incarnation,
  crypto and pack epochs, covered logical-root manifest, generation, and
  compare-and-swap predecessor;
- sync compares canonical logical roots and compartment proofs while
  reconciling endpoint placement projections separately;
- one endpoint's placement root cannot overwrite, replace, or win over another
  endpoint's projection by arrival order;
- independent canonical vectors for entry keys, instance identifiers,
  compartment roots, realm manifests, partial-access proofs, logical-root
  manifests, projection/rebuild/delta proofs, opaque compartment handles and
  rotations, and endpoint placement projections;
- format-specific reference decoders, seed corpus, and fuzz targets for
  creation-operation reservations, instance IDs, composite keys, logical
  nodes/proofs, rebuild/delta replay certificates, witness statements,
  compartment handles, realm manifests, partial-access proofs, logical-root
  manifests, and endpoint placement projections;
- early go/no-go benchmark thresholds for sealed-private no-op status, one-file
  edit projection/update, one DEK unwrap, logical inclusion/absence proof
  verification, partial-access proof verification, and full index rebuild at
  small and representative large fixtures;
- format admission fails or selects another design when p95 latency, peak
  memory, I/O amplification, proof size, or rebuild throughput exceeds the
  declared threshold; v0.132.0 later supplies comprehensive production-scale
  characterization;
- threat-model review covering instance fanout, compartment-count leakage,
  operation-ID reuse, projection-certificate amplification, witness Sybil and
  equivocation, endpoint projection confusion, root substitution, and
  noncanonical tree shape;
- explicit implementation stop: v0.93.0 and v0.97.0 may not persist private
  index bytes until this admission is complete.

Verification:

- independent private-index and realm-manifest vector validator;
- independent creation-operation, projection-certificate, and witness-statement
  vector validator;
- unique-representation and amplification model review;
- partial-access disclosure fixture review;
- endpoint-placement reconciliation model;
- format-specific fuzz smoke suite;
- admission benchmark runner and recorded threshold artifact.

Exit criteria:

- The private index represents every encryption instance without conflating
  instance fanout with duplicate semantic identity.
- One canonical entry set has one admitted logical root under all operation
  histories, or a uniquely represented alternative is selected.
- A compartment-only recipient can verify its compartment's inclusion and
  continuity without learning other compartments' identities, counts, or
  structure.
- Placement is endpoint-local projection state and cannot alter or overwrite
  canonical logical state.
- Instance IDs are collision-resistant, context-bound, stable under projection
  changes, and replaced exactly when erasure identity changes.
- A signed manifest cannot establish index completeness without deterministic
  ledger projection evidence; partial-access verification reports its reliance
  on admitted full-view signers or witnesses.
- Projection evidence has one canonical replay meaning with bounded chains,
  verification work, storage, and mandatory rebuild cadence; it is not
  represented as a succinct proof.
- Required witnesses have explicit replay assurance, independence, lifecycle,
  threshold, and unavailability semantics resistant to nominal Sybil
  enrollment.
- Projection implementation disagreement and evaluator defects fail closed,
  preserve original signed evidence, and can be resolved only through an
  admitted signed projection-version migration.
- Creation-operation reservations are crash-safe, idempotent for one exact
  operation, and never silently reused after abandonment or cancellation.
- The selected formats pass early usability and scale rejection thresholds
  before implementation begins.

### v0.92.2 - Chunked AEAD Nonce And Retry Instantiation

Goal: instantiate v0.23.2 chunk authentication with exact nonce/key uniqueness,
exhaustion, retry, and rollback behavior for every admitted AEAD suite.

Deliverables:

- canonical encrypted-record header binds format/version, exact AEAD/KDF suites,
  record/encryption-instance, v0.23.3 reservation, and v0.23.4 provider
  operation IDs, realm/compartment,
  audience/purpose, key epoch, random seed or per-record key commitment, chunk
  size/count, total plaintext/ciphertext lengths, and manifest/root commitment;
- each suite profile selects and freezes one reviewed construction: an
  independently random per-record nonce seed with domain-separated checked
  chunk-nonce derivation, independently derived per-record subkey plus checked
  chunk nonce, or another admitted construction with equivalent uniqueness and
  misuse analysis;
- seed/subkey creation uses v0.23.0 OS-CSPRNG/provider-only derivation and stops
  before record publication on entropy, derivation, reservation, or persistence
  failure;
- chunk nonce/subkey derivation binds exact record identity, suite/key epoch,
  operation ID, chunk domain label, and fixed-width checked chunk index; no
  truncation, endian ambiguity, index wrap, or cross-record/domain collision;
- final completeness record uses a cryptographically separate `final` domain/
  nonce or subkey that cannot collide with any data-chunk index and authenticates
  ordered chunk root, exact count, total lengths, and completion;
- maximum chunk count is below both integer/derivation exhaustion and the
  admitted suite's per-key/per-nonce security limits; encoders fail closed before
  the last safe index and decoders reject declared/observed counts beyond it;
- encrypting different plaintext or associated data under an existing effective
  chunk nonce/key pair is prohibited even when an operation is retried, resumed,
  restored, or cloned;
- exact retry may return already journaled ciphertext only when record identity,
  operation ID, plaintext/AD commitments, suite/key epoch, seed/subkey domain,
  and every output byte match; otherwise it abandons the uncertain domain and
  reserves a fresh operation/record identity and nonce seed/subkey;
- an ambiguous crash never blindly re-encrypts under an old nonce domain; prior
  completion is queried through v0.23.4, or the old record is fenced and a new
  identity is allocated with explicit supersession evidence;
- resume begins only at an authenticated chunk boundary, derives the expected
  next nonce/subkey from public bound state inside the provider, and serializes
  no raw AEAD state, provider key, per-record subkey, seed-derived secret, or
  uncommitted plaintext;
- restored snapshot and undetected clone behavior inherits v0.92.0: random seed/
  record identity, misuse-resistant profile, or external non-rollback anchor
  must preserve the suite's safety without assuming clone detection;
- pre-final consumers satisfy v0.23.2 rollbackable side-effect-free staging;
  external IPC/hooks, worktree writes, and irreversible publication activate
  only after final completeness authentication and typed ingest;
- known-answer/differential vectors cover first/last/final nonce domains,
  endian/index boundaries, maximum count, same/different plaintext retry,
  resume, crash ambiguity, supersession, repeated randomness, snapshot/clone,
  chunk reorder/duplicate/omission, and final-record substitution.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-sync`
- per-suite chunk nonce/subkey known-answer vectors;
- Kani or equivalent index/exhaustion/domain-separation proof;
- crash/retry/resume/clone state-machine tests.

Exit criteria:

- Every admitted chunk and final record has one non-colliding effective
  nonce/key domain under exact checked limits.
- Retry/resume cannot rewrite different plaintext under an existing nonce/key or
  serialize provider-only cryptographic state.
- No pre-final authenticated prefix causes an external or irreversible side
  effect, and chunked encrypted operation remains disabled for suites without a
  complete admitted profile.

### v0.93.0 - Sealed Private Locator And Storage Identity

Goal: avoid known-plaintext membership leaks before any encrypted realm can be
created.

Deliverables:

- immutable semantic commitment format used by canonical references,
  transitions, proofs, and signatures inside the encrypted semantic ledger;
- normative random-blinded semantic commitment over length-framed format,
  realm, compartment, object schema and type, canonical plaintext, and an
  independently random 256-bit blinding value;
- the blinding value comes from the v0.23.0 OS-CSPRNG boundary, is stored only
  inside the encrypted semantic ledger, and is never derived from plaintext,
  object hashes, locators, clocks, counters, or ancestor keys;
- semantic-commitment domain separation from public object hashing, private
  lookup locators, encryption, wrapping, signatures, proofs, and every other
  cryptographic purpose;
- deterministic compartment-keyed private locators provide authorized lookup
  and deduplication without making the immutable semantic commitment
  deterministic to outsiders;
- independent known-answer vectors for semantic commitments, private locators,
  ciphertext IDs, and every domain-separation label;
- epoch-specific keyed private lookup locator format;
- ciphertext storage ID format;
- distinct non-interchangeable types for semantic commitment, private locator,
  and ciphertext storage ID;
- one candidate canonical persistent authenticated B+ tree structure for the
  1.0 private locator index; it freezes only after the unique-representation
  gate passes, after which alternative B-tree, trie, register, split, ordering,
  or root algorithms are incompatible critical format versions rather than
  implementation choices;
- canonical composite key encoding inherited from v0.92.1:
  `(locator_epoch, private_locator, semantic_commitment,
  encryption_instance_id)` so every policy-separated encryption instance is
  its own logarithmically searchable leaf entry rather than one linear
  multi-value register value;
- encryption-instance IDs use the exact v0.92.1 context-bound random-nonce
  construction and replacement lifecycle; implementations cannot substitute a
  local counter, UUID variant, ciphertext hash, DEK ID, or storage position;
- canonical leaf value binds only stable candidate information: immutable
  semantic commitment and encryption-instance identity through the composite
  key, plus logical object kind and stable erasure-policy class;
- ciphertext storage IDs, ciphertext records, pack generations, storage
  receipts, and current positions are prohibited from logical leaves and live
  exclusively in mutable endpoint placement and reverse-resolution indexes;
- exact bytewise composite-key ordering, node fanout bounds, minimum occupancy,
  leaf/internal layouts, separator derivation, split point, merge/redistribution
  rule, empty root, node encoding, logical root derivation, and canonical bulk-
  build algorithm;
- normative history-independent normalization algorithm making the logical root
  a pure function of the complete canonical entry set, independent of insertion,
  deletion, merge, split, union, rebalance, or bulk-build history;
- compatibility freeze is blocked unless an independent proof or executable
  model establishes unique representation for every permutation of one entry
  set; if bounded path-copy B+ tree updates cannot satisfy that property,
  v0.93.0 must select a reviewed uniquely represented trie or key-derived tree
  before durable bytes are admitted;
- internal and leaf logical nodes commit exact key ranges, child logical
  commitments, entry counts, tree height, locator epoch, and structure version;
- inclusion, non-inclusion, range, and total-count proofs are logarithmic in the
  admitted candidate count under the declared fanout and height bounds rather
  than requiring linear predecessor/successor traversal;
- three non-interchangeable private-index identity layers:
  - deterministic domain-separated keyed logical node commitments and logical
    root over canonical plaintext node encodings, visible only inside authorized
    encrypted semantic views;
  - randomized AEAD encrypted node envelopes whose ciphertext may differ across
    replicas, rewrites, and re-encryption while authenticating the logical node
    commitment;
  - public ciphertext storage IDs over randomized encrypted envelopes, visible
    to blind storage and never used as the logical tree identity;
- "same canonical entry set produces the same root" applies only to the private
  keyed logical root; randomized ciphertext roots and public storage IDs are not
  required or expected to converge byte-for-byte;
- a dedicated private-index commitment key derives logical node commitments
  under exact realm, compartment, structure-version, and commitment-key epoch
  domains;
- a signed and checkpointed private-index manifest binds the logical root,
  semantic-state root, locator epoch, commitment-key epoch, structure version,
  projection version, forward/reverse entry and instance counts, projection-
  evidence root, policy root, membership epoch, and governing authorization;
- manifest acceptance requires the v0.92.1 canonical ledger projection,
  full-rebuild proof, or complete delta-transition proof; signature authority
  alone cannot make an omitted or invented logical entry valid;
- possession of the private-index commitment key permits verification and
  commitment calculation but does not authorize publication or admission of a
  new canonical index root;
- historical manifests remain verifiable across commitment-key rotation, while
  compromise triggers a governed new epoch and explicit root migration rather
  than silent root replacement;
- canonical per-replica duplicate-creation admission quotas by locator epoch,
  with quota parameters committed by realm policy so an authorized replica
  cannot consume every candidate slot for one locator while offline;
- each quota-consuming candidate binds and spends one valid v0.52.0 escrow right
  from the creating replica's signed allocation; candidate identity includes the
  spent-right reference so merge can detect replay or double-spend;
- quota accounting binds actor, device, replica identity, replica incarnation,
  locator epoch, locator, admitted event sequence, causal frontier, and the
  v0.52.0 governance-backed aggregate quota identity; it cannot be reset by
  replay, reconnect, process restart, new replica incarnation, device
  restoration, replica retirement, or ordinary key rotation;
- canonical limits include per-replica and aggregate per-device/per-actor
  duplicate-creation budgets so a principal cannot evade resource controls by
  enrolling or rotating through many replicas;
- separate canonical instance-fanout limits and signed offline instance rights
  bound to semantic commitment, compartment, erasure-policy class, actor,
  device, replica incarnation, and membership epoch;
- adding an encryption instance to an existing semantic commitment consumes
  instance-fanout capacity but no duplicate-semantic-identity right; creating a
  new independently blinded semantic commitment consumes duplicate-identity
  capacity even if it initially has only one instance;
- duplicate-identity and instance-fanout rights cannot be transferred,
  substituted, replayed, or counted across one another;
- candidates exceeding canonical per-replica admission quotas are rejected
  before becoming admitted history; locally configured tighter resource limits
  may place otherwise valid unadmitted candidates in authenticated quarantine
  but cannot silently discard or rewrite already admitted history;
- a candidate that exceeds aggregate actor/device rights during merge, depends
  on a double-spent right, or arrives without a valid allocation remains in
  authenticated quota-conflict quarantine together with its dependent
  transition until governance-authorized resolution;
- immutable content-addressed nodes use bounded fanout, key/value count, encoded
  bytes, height, and proof size at the private logical layer; encrypted envelopes
  and storage IDs are replaceable projections, not canonical logical node IDs;
- deterministic concurrent union and split semantics preserve every admitted
  candidate, produce the same private logical root for the same canonical entry
  set, and create explicit conflict evidence if incompatible structure versions
  or quota states are presented;
- updates use path copying and bounded rebalancing without rewriting unrelated
  immutable nodes;
- normalization amplification is explicitly bounded and measured for insert,
  delete, union, split, merge, and bulk-build; an algorithm whose
  history-independent normalization requires unbounded or whole-index rewrite
  amplification cannot pass this milestone;
- format-level maximum read, write, node, proof, and rebalance amplification
  budgets are committed by structure version and cannot be raised by untrusted
  input;
- duplicate-amplification detection reports repeated independently blinded
  creation of byte-identical plaintext by one replica, actor, device, or
  coordinated replica set without exposing the plaintext or duplicate relation
  outside authorized encrypted views;
- concurrent offline creation of equal canonical plaintext may produce one
  private locator and multiple independently blinded semantic commitments; all
  already signed identities and references remain valid and reconciliation
  cannot rewrite either history;
- optional encrypted duplicate-equivalence evidence binding two or more
  semantic commitments, verification that their openings produce byte-identical
  authenticated canonical plaintext, locator epoch, verification transcript,
  and selected future-reference policy without embedding another plaintext
  copy;
- duplicate-equivalence evidence never makes an old signature cover another
  commitment and never aliases commitments inside historical references;
- before admitted equivalence evidence, new references preserve the exact
  semantic commitment selected by their creating history;
- after admitted equivalence evidence, realm policy either preserves both
  identities or selects one immutable representative for future references
  only; adding later duplicates requires a new signed equivalence transition and
  cannot silently change the selected representative;
- every representative-selection transition binds the expected previous
  equivalence-set root, expected prior representative or explicit absence,
  every candidate semantic commitment, and the creating causal frontier as a
  compare-and-swap precondition;
- concurrent transitions that select different representatives create explicit
  equivalence-conflict heads; Sagnir preserves each transition, refuses
  last-writer-wins, and requires an authorized multi-parent resolution before
  representative-based future deduplication resumes;
- representative selection is an explicit policy-authorized decision and never
  chooses the numerically lowest semantic commitment, blinding value, private
  locator, ciphertext ID, signature, or transition hash; attacker-controlled
  randomness cannot grind representative priority;
- conflict resolution binds all known equivalence heads and cannot erase a
  signed duplicate identity or make an old reference resolve to a different
  commitment;
- keyed-locator candidates are decrypted, authenticated, canonically decoded,
  and compared before equal-plaintext deduplication is accepted;
- different plaintext values sharing one keyed locator are retained as distinct
  bucket entries, reported as a locator-collision security event, and never
  treated as equivalent;
- bounded search-tree nodes, logarithmic proofs, bounded per-operation
  traversal, canonical admission quotas, deterministic union/split semantics,
  and governed locator-key rotation or incident handling for pathological
  collisions or duplicate amplification;
- v1 translation evidence is the encrypted authenticated mapping plus
  authenticated-index inclusion and canonical-plaintext verification, not a
  publicly verifiable zero-knowledge proof;
- actual zero-knowledge locator-translation proofs remain prohibited until a
  separately reviewed proof system is admitted under the post-1.0
  hidden-witness milestone;
- exact visibility matrix: semantic commitments and translation mappings are
  visible only inside an unlocked authorized semantic ledger or an explicitly
  recipient-encrypted disclosure; blind stores receive ciphertext storage IDs,
  opaque pack commitments, bounded availability metadata, and no semantic
  commitments;
- semantic commitments and translation mappings are prohibited from blind-store
  metadata, unauthenticated protocol fields, telemetry, diagnostics, logs,
  filenames, public proof summaries, and public storage receipts;
- quota allocations, spent-right references, conflict roots, replica topology,
  and actor/device activity are encrypted and scoped to authorized policy,
  governance, or audit views and are prohibited from blind-store metadata,
  telemetry, diagnostics, filenames, public receipts, and locked status;
- ordinary re-encryption, envelope replacement, receipt renewal, repacking, and
  ciphertext relocation may change placement roots but cannot change the
  private locator logical root or its signed semantic-state binding;
- substitution resistance proving a locator translation cannot map to different
  canonical plaintext;
- verification recomputes the semantic commitment from canonical plaintext and
  its encrypted blinding value before a reference, transition, proof, or
  signature is accepted;
- deduplication identity policy scoped to one key domain;
- randomized encryption requirement;
- non-revealing private-locator formatting;
- no public plaintext object hash in sealed-private metadata;
- known-plaintext dictionary attack, low-entropy content, same-plaintext
  cross-compartment and cross-realm correlation, formatting,
  identity-confusion, translation substitution, locator/ciphertext interchange,
  concurrent offline duplicate, duplicate-equivalence substitution,
  conflicting representative selection, representative grinding,
  different-plaintext locator collision, search-node truncation or fork,
  malformed range, non-logarithmic proof, union/split divergence, amplification
  overflow, missing/duplicate encryption instance, instance-key substitution,
  instance-ID collision/context substitution, wrong replacement lifecycle,
  creation-reservation sequence/nonce rollback, cross-operation replay,
  abandoned/cancelled reservation reuse, consume/cancel race,
  duplicate-right/instance-right confusion, instance-fanout exhaustion,
  semantic-ledger projection omission/invention, malicious manifest signer,
  partial-access completeness overclaim, missing ledger chunk, forged replay
  transcript, excessive delta chain/bytes/work, mandatory rebuild bypass,
  witness replay-mode misstatement, nominal witness Sybil, unavailable
  threshold downgrade, stale/revoked witness, witness equivocation,
  insertion/deletion/union/split/bulk-build permutation divergence,
  normalization amplification, unauthorized root publication, commitment-key
  compromise and rotation, historical-manifest verification, placement-only
  re-encryption/repack/relocation, per-replica and aggregate quota replay,
  new-incarnation evasion, locator-rotation quota reset, authorized-replica
  bucket exhaustion, private quota-metadata leakage, public-output leakage, and
  semantic-root preservation tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-crypto`
- independent commitment and locator vector validator.
- independent canonical B+ tree node, split/merge, proof, logical-root,
  encrypted-envelope, and storage-ID vector validator.
- independent history-independent representation and operation-permutation
  validator;
- independent creation-reservation, projection replay/delta, and witness
  validator.

Exit criteria:

- Sealed-private formats can hide whether known plaintext content is present.
- Equal plaintext receives unlinkable semantic commitments across compartments
  or independent creation events unless authorized private-locator deduplication
  intentionally resolves to an existing admitted object.
- Equal private locators do not imply equal signed identity, and neither
  duplicate reconciliation nor deduplication mutates previously signed graph
  references.
- Resource bounds refuse or quarantine new over-quota candidates before
  admission and never silently discard an identity that is already admitted.
- Offline aggregate quota enforcement uses disjoint signed rights; merge-time
  overdraw or double-spend remains explicit quarantine and cannot become
  authoritative by counter reconciliation.
- Candidate and encryption-instance inclusion and absence are logarithmic
  because each `(locator, semantic commitment, encryption instance)` tuple is a
  separate canonical tree key.
- Deterministic convergence applies to the private logical root only; randomized
  encrypted node envelopes and blind-store ciphertext IDs reveal no equality
  requirement.
- The admitted logical structure has one representation for one canonical entry
  set under every construction and update history, or the B+ tree design is
  replaced before compatibility freezes.
- Re-encryption, repacking, relocation, and receipt renewal change only
  placement projections; they do not alter the locator logical root.
- A logical root is authoritative only through its signed checkpointed manifest,
  never merely because an actor can compute keyed commitments.
- A signed manifest is authoritative only after admitted projection evidence
  proves its forward/reverse roots are complete for the bound semantic ledger
  state; partial-access recipients receive explicit signer/witness trust status.
- Reservation non-reuse, bounded replay/delta verification, mandatory rebuilds,
  and required witness threshold are enforced by implementation tests before
  encrypted index persistence.
- Conflicting representative selections remain explicit multi-head state until
  an authorized resolution binds every admitted conflict parent.
- Possession of blind-store metadata, logs, public proofs, storage receipts, or
  ciphertext IDs does not provide a semantic-commitment dictionary oracle.
- Historical signatures and canonical graph references bind immutable semantic
  commitments, never rotatable private locators or ciphertext IDs.
- No user-facing encryption command exists before these three identity layers
  and their authenticated mapping are admitted.

### v0.94.0 - Vault Metadata Model

Goal: represent encrypted realm mode and privacy guarantees before writing
ciphertext.

Deliverables:

- vault enabled marker;
- sealed-private mode marker;
- crypto epoch reference;
- protected metadata and identifier policy;
- explicit prohibition on representing metadata-visible encryption as
  sealed-private;
- encrypted realm status model;
- invalid mode, downgrade, and contradictory metadata tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-store`

Exit criteria:

- Sagnir can distinguish open and sealed-private realms before touching object
  encryption.
- Privacy guarantees are explicit machine-readable state, not CLI wording.

### v0.95.0 - Private Metadata And Padded Storage

Goal: protect semantic metadata and reduce observable storage-shape leakage
before encrypted realm creation.

Deliverables:

- two-layer public commitment and encrypted semantic ledger model;
- public layer commits only to ciphertext packs, protocol epochs, and deliberate
  availability or policy predicates; it contains no semantic object commitment,
  private locator, translation mapping, path, actor, world, fact, or graph-edge
  identity;
- encrypted paths, worlds, actors, facts, graph edges, policies, and proofs;
- fixed-size or bucketed record classes;
- epoch batching and pack compaction;
- configurable object-count and transfer-size padding;
- normative privacy profiles such as baseline private, padded private, and
  high-security cover-traffic mode, with machine-readable profile IDs and
  parameters committed by local/realm policy where authoritative;
- per-profile observable leakage contract covering exact envelope/header fields,
  ciphertext and pack size buckets, object/record count bounds, crypto/key/pack
  epochs, endpoint/receipt identifiers, operation frequency, request/response
  timing resolution, batching windows, access-order leakage, repeated-read/write
  linkability, and error/refusal distinguishability;
- filesystem leakage contract covering file and directory names, entry count,
  creation/removal/rename churn, allocation size, sparse extents where visible,
  modification/change/access/birth timestamps, journal/CoW effects, and pack
  replacement cadence visible to a malicious local storage provider;
- profile-specific maximum padding overhead, minimum/maximum batch size,
  maximum batching latency, cover-traffic bandwidth/storage budget, dummy
  request ratio, read/write coalescing window, and acceptable availability or
  latency tradeoff;
- explicit statement of which protections require continuously or
  schedule-driven cover traffic and which leakage remains when cover traffic is
  disabled, interrupted, rate-limited, or distinguishable;
- repeated operation linkability matrix for no-op status, one-file edit, object
  read, proof verification, unlock/lock, repack, receipt renewal, sync, and
  failed lookup under each profile;
- malicious local storage-provider model observing all ciphertext bytes,
  filenames, directory operations, filesystem metadata/timing, block and pack
  offsets, read/write frequency, process-visible I/O where in scope, crashes,
  and retention of old ciphertext copies, while lacking authorized plaintext
  keys;
- measurable leakage tests and traces compare observed size/timing/frequency/
  access-pattern distributions against profile bounds rather than relying only
  on prose review;
- deterministic test fixtures and statistical methodology with declared sample
  counts, tolerated variance, clock resolution, false-positive threshold, and
  reproducible seeds for timing/traffic-shape assertions;
- profile downgrade, unsupported cover traffic, exceeded padding/latency budget,
  and local-provider capability mismatch produce explicit warning or refusal
  according to policy;
- runtime privacy-profile state machine with canonical states `Healthy`,
  `Degraded`, `Unavailable`, and `Recovering`, plus reason codes and
  policy-bound transition rules;
- `Healthy` requires every profile-mandated padding, batching, cover-traffic,
  timing-resolution, queue-capacity, storage-provider, and clock capability to
  be operating within the declared measurement window and budget;
- continuous or schedule-driven health monitors cover traffic production,
  dummy/real traffic ratio, padding and bucket conformance, batching delay,
  queue pressure, bandwidth/storage budget, timer/clock capability, and local
  provider features without inspecting or logging private operation content;
- detected contract violation enters `Degraded`; inability to establish the
  required capabilities or measurements enters `Unavailable`; restart,
  partition, provider recovery, and budget restoration enter `Recovering`
  until the profile's declared observation window and recovery probes succeed;
- protected profiles fail closed before new protected traffic while
  `Degraded`, `Unavailable`, or `Recovering`, except for a separately declared
  bounded recovery channel whose observable behavior is included in the
  profile contract; less strict profiles may warn only when their policy
  explicitly permits degraded operation;
- encrypted authenticated health records bind profile/policy epoch, prior and
  new state, exact best-known affected interval, monitor observations, failed
  capability/budget, recovery evidence, local monotonic sequence, and
  checkpoint; blind stores and public logs receive no activity-bearing health
  detail;
- traffic emitted during a known or subsequently bounded degradation interval
  is permanently labelled with the weaker observed assurance and is never
  retroactively claimed to satisfy the stronger profile after recovery;
- authorized `saga vault status` reporting exposes current health and encrypted
  local history at policy-approved granularity, while locked, remote, public,
  and unauthenticated status surfaces use fixed/coarsened responses that do not
  become an activity, outage, or recovery oracle;
- metadata leakage inventory and profile-specific defaults;
- tests that public summaries do not expose redacted plaintext fields and
  profile fixtures do not exceed declared observable bounds;
- restart, crash, suspend/resume, network partition, clock degradation,
  exhausted cover-traffic budget, stalled scheduler, provider loss/recovery,
  false recovery, and status-oracle tests for every supported profile.

Verification:

- `cargo test -p sagnir-vault`
- encrypted metadata fixture review.
- privacy-profile trace and leakage-budget test suite;
- malicious local storage-provider simulation.

Exit criteria:

- Locked or blind-storage views expose only documented minimal commitments and
  availability metadata.
- Public proof summaries contain only deliberate predicates over explicitly
  disclosed statements; they do not expose private semantic commitments or
  translation mappings.
- Every privacy profile states measurable leakage and overhead limits, including
  what remains visible without cover traffic and to a malicious local storage
  provider.
- Sagnir does not describe padding or batching as protection against timing or
  access-pattern correlation unless the selected profile's measured contract
  supports that claim.
- Protected privacy profiles cannot remain or return `Healthy` without current
  measured evidence; degraded intervals remain auditable without making their
  existence publicly observable.

### v0.96.0 - Encrypted Object Envelope

Goal: define encrypted object bytes and authenticated metadata.

Deliverables:

- encrypted object magic;
- AEAD algorithm identifier;
- nonce field;
- ciphertext length field;
- authentication tag field;
- public associated-data binding only for envelope format and suite, opaque
  storage realm handle, crypto epoch, key ID, pack/segment, and record position;
- each externally visible associated-data field is listed in the v0.95.0
  profile leakage contract, including linkability and rotation behavior; no
  field is treated as harmless merely because it is non-plaintext;
- encrypted authenticated header binding compartment, object schema and type,
  immutable semantic commitment, blinding value, and compression/delta format;
- private-index node envelope variant binding the v0.93.0 canonical logical node
  commitment, logical structure version, locator epoch, randomized nonce,
  encrypted canonical node bytes, and envelope crypto epoch;
- randomized re-encryption of one unchanged logical node produces a different
  ciphertext storage ID while decrypting to the same authenticated private
  logical commitment;
- no semantic commitment, blinding value, private locator, or canonical object
  metadata in the externally visible envelope;
- malformed envelope, logical-commitment substitution, deterministic-ciphertext
  leakage, wrong locator epoch, and ciphertext/logical identity confusion tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-crypto`

Exit criteria:

- Encrypted object metadata is bounded and context-bound before decryption.
- Blind storage addresses randomized node envelopes by ciphertext storage ID
  and cannot observe or require equality of private logical node commitments.

### v0.97.0 - Authenticated Encrypted Pages And Index

Goal: support bounded random access without decrypting an entire realm.

Deliverables:

- authenticated encrypted page or segment format;
- encrypted manifest and index;
- authenticated encrypted forward index from private locator and locator epoch
  using the exact v0.93.0 canonical persistent authenticated B+ tree and
  composite `(locator_epoch, private_locator, semantic_commitment,
  encryption_instance_id)` keys;
- immutable private logical index nodes with committed key ranges, bounded
  fanout and height, deterministic encoding, child logical commitments, entry
  counts, locator epoch, and structure version;
- randomized encrypted node-envelope index from private logical commitment to
  one or more current ciphertext storage IDs and placements;
- one canonical signed logical-root manifest per compartment plus the v0.92.1
  authenticated realm manifest over opaque compartment-root references;
- compartment-scoped inclusion and continuity proofs reveal no other
  compartment locator, entry count, tree shape, name, or membership;
- endpoint-local encrypted placement manifests suitable for storage
  transactions, each scoped to one replica incarnation, device, or storage
  endpoint rather than shared canonical state;
- logical leaves contain no ciphertext ID, pack generation, receipt, or storage
  position; the encrypted placement and reverse indexes own all mutable
  resolution from stable encryption-instance identity to current ciphertext;
- signed checkpointed logical-root manifest validation is required before an
  index root can become authoritative, and commitment-key possession alone
  grants no write authority;
- manifest validation replays the canonical ledger-to-index projection or
  verifies the complete v0.92.1 delta/rebuild proof before accepting forward
  and reverse roots;
- logarithmic inclusion, absence, range, and total-count proofs;
- deterministic concurrent union, split, and path-copy update semantics that
  preserve every admitted candidate and converge to one private logical root for
  one canonical entry set without requiring identical ciphertext envelopes;
- implementation of the v0.93.0 history-independent normalization algorithm,
  including canonical rebuild after deletion and merge, with refusal of any
  alternative valid-looking shape whose root differs for the same entry set;
- maximum read, write, node, proof, and rebalance amplification budgets enforced
  before allocation or mutation;
- authenticated encrypted reverse logical index keyed by
  `(semantic_commitment, encryption_instance_id)` to private locator epoch,
  logical object kind, erasure-policy class, and compartment root;
- endpoint-local placement/reverse-resolution index from
  `(semantic_commitment, encryption_instance_id)` to randomized node/object
  envelopes, ciphertext records, pack generations, receipts, and current
  positions;
- canonical forward and reverse logical indexes update atomically with the
  encrypted semantic-ledger transaction and are rebuildable from admitted
  canonical encrypted records;
- every local transaction emits deterministic projection deltas suitable for
  independent replay, while periodic full rebuilds prove no accumulated
  omission or invented entry;
- endpoint placement and reverse-resolution projections update atomically per
  endpoint, bind the canonical logical-root manifest they cover, and are
  rebuildable without becoming canonical realm state;
- logical-node updates and signed logical-root publication do not require
  identical randomized envelopes or placement roots on every replica;
- randomized envelope writes, public storage IDs, endpoint reverse-resolution
  entries, and placement roots never become B+ tree child commitments or
  overwrite another endpoint's projection by arrival order;
- admitted locator candidates that exceed an operator's local operation budget
  remain durable and discoverable through resumable authenticated search;
  local resource pressure cannot truncate the candidate set, rewrite its
  total-count commitment, or force linear proof verification;
- graph traversal and signature verification resolve semantic commitments
  and exact encryption instances through the reverse logical index and never
  scan every private locator or every instance of one commitment;
- signed pack/root commitment;
- isolated compression contexts that never combine attacker-controlled input
  with secrets;
- deduplication and delta bases scoped to one compartment and key domain;
- deduplication and delta reuse additionally scoped to a compatible erasure
  unit, retention class, and legal-hold policy;
- authentication before decompression or plaintext parsing;
- expanded-size and decompression-ratio limits;
- ciphertext storage hash computed during encryption;
- compression-oracle, cross-compartment base, random-read, substitution,
  truncation, forward/reverse mismatch, duplicate bucket corruption, stale pack
  position, wrong endpoint projection, cross-endpoint overwrite, compartment-
  root omission/substitution, partial-proof disclosure, instance omission,
  instance aliasing, instance-ID collision, malicious signed manifest,
  projection omission/invention, unavailable replay chunk, delta/rebuild
  mismatch, overlong delta chain, cumulative proof/work overflow, skipped
  mandatory rebuild, witness threshold/Sybil/revocation/equivocation failure,
  node omission, range overlap or gap, excessive height, noncanonical split or
  separator,
  composite-key ordering mismatch, candidate-register linear scan regression,
  union/split divergence, logical/ciphertext root confusion,
  randomized-envelope root mismatch, local-budget interruption, amplification
  overflow, operation-history permutation divergence, unauthorized manifest,
  placement-only re-encryption/repack/receipt/relocation root stability,
  million-object reverse lookup, and decompression-bomb tests.

Verification:

- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-store`

Exit criteria:

- Status and index lookup can read bounded authenticated regions.
- Secret and attacker-controlled bytes never share a compression context.
- Shared semantic content with incompatible erasure policy remains in separate
  encryption instances even when plaintext deduplication would otherwise be
  possible.
- Reverse lookup remains bounded and authenticated when one locator has
  multiple independently signed semantic commitments.
- Forward lookup has logarithmic inclusion and absence proofs with bounded
  immutable nodes and declared amplification limits.
- Authorized replicas converge on the private logical root while remaining free
  to use different randomized encrypted envelopes and ciphertext storage IDs.
- Authorized replicas converge on canonical compartment and realm manifests
  while retaining independent endpoint-local placement projections.
- Every semantic commitment can resolve logarithmically to each admitted
  encryption instance through exact composite forward and reverse keys.
- Full-view verification proves logical-index completeness from admitted
  semantic state; partial-access verification states and enforces its
  signer/witness trust dependency.
- Unauthenticated ciphertext never reaches decompression or canonical parsing.

### v0.98.0 - Passphrase Unlock Baseline

Goal: support one local unlock method for development and tests.

Deliverables:

- passphrase-based key wrapping metadata;
- Argon2id or a later explicitly admitted memory-hard password KDF with exact
  suite/revision identity, bounded memory/time/parallelism/salt/output parameters,
  and policy floors/ceilings informed by current standards at implementation;
- key-encryption-key metadata;
- realm-master-key wrapping model;
- activate the v0.23.4 `UnlockCapability` minting path only for a bounded unlock
  ceremony over admitted wrapping metadata and retained encrypted-realm/header
  authority; it cannot sign or publish state;
- denial-of-service limits for untrusted KDF parameters;
- domain-separated labeled context binding realm, compartment/scope, crypto
  epoch, slot purpose, KDF suite, and wrapper suite;
- passphrase values enter only through internal test/provider APIs in this
  release; production CLI/bootstrap/unlock ingestion remains unavailable until
  v0.98.2 admits secret sources and cleanup behavior;
- no passphrase in logs or debug output tests;
- standards known-answer, malformed parameter, weak-parameter downgrade,
  over-budget, cancellation, and wrong-passphrase response-shape tests.

Verification:

- `cargo test -p sagnir-crypto`

Exit criteria:

- A passphrase can unlock a test realm key without becoming the realm key.
- This cryptographic baseline does not yet provide a production passphrase input
  channel.

### v0.98.1 - Bounded Unlock Target And Oracle Resistance

Goal: implement the v0.23.4 unlock schema as a narrow authenticated discovery
boundary before recipient workflows or the public unlock command depend on it.

Deliverables:

- unlock ceremony starts from retained store/genesis handles and verified exact
  store, realm, genesis, format, and bootstrap-policy identities; display paths
  or caller-supplied realm IDs cannot select authority;
- authenticated outer/header commitment and canonical bounded public header are
  verified before capability minting, including suite, crypto epoch, slot table
  commitment, compartment handles, ciphertext range map, and maximum discovery
  scope disclosed by the selected privacy profile;
- passphrase wrapper or recipient slot is selected only from that committed
  bounded header using its exact slot/wrapper commitment; callers cannot supply
  replacement ciphertext, associated data, nonce, KDF parameters, slot bytes,
  range, instance ID, or destination;
- one `UnlockCapability` binds retained store/genesis identity, outer/header and
  slot/wrapper commitments, exact suite/epoch, compartment/max scope, admitted
  ciphertext ranges or encryption-instance IDs, operation ID, limits, audience/
  purpose, and one typed plaintext result class;
- initial capability permits only the minimum authenticated metadata/policy/
  lifecycle discovery declared by the public bootstrap header; broader
  compartment/source decryption requires a new capability minted after the
  discovered state completes AEAD authentication, canonical decoding,
  signature/checkpoint, lifecycle/revocation, and policy validation;
- decrypted discovery bytes enter isolated v0.23.2/v0.92.2 staging as untrusted
  `ProtectedStateCandidate`; they cannot satisfy policy, select their own crypto
  context, retroactively authorize the decrypt that exposed them, widen scope,
  publish state, or choose an output sink;
- successful validation produces a typed `ValidatedProtectedState` bound to the
  original capability/store/genesis/header/checkpoint and exact bytes; failure
  destroys staging and returns no partial parser/policy result;
- response behavior for unknown slot, wrong passphrase/key, malformed wrapper,
  revoked/expired recipient, tag failure, range mismatch, unsupported suite,
  and unauthorized compartment follows bounded policy-declared timing/work/
  error classes without revealing key validity, recipient membership, slot
  occupancy, plaintext prefix, or policy contents;
- attempt/rate/resource limits are local anti-abuse controls and do not alter
  canonical policy; protected deployments may require key-agent/HSM rate limits
  or external authorization before expensive KDF/unwrap work;
- arbitrary-ciphertext, chosen-AD, chosen-slot, range widening, instance
  substitution, output redirection, self-authorizing policy, stale checkpoint,
  revoked recipient, response-shape differential, cancellation, crash, and
  staging-cleanup tests.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-vault`
- unlock capability and protected-state typestate compile-fail suite;
- oracle response-shape and chosen-ciphertext integration tests;
- crash/cancellation/staging cleanup fault injection.

Exit criteria:

- Unlock decrypts only ciphertext committed by the retained realm/header and
  only into its one bounded typed result class.
- Decrypted policy/lifecycle state cannot authorize its own exposure or widen
  the capability that exposed it.
- Failures do not turn unlock into a general decryption, key-validity,
  recipient-membership, slot-occupancy, or plaintext oracle.

### v0.98.2 - Secret Input And Generated Credential Boundary

Goal: admit production passphrase/bootstrap-secret ingestion without exposing
secrets through process metadata, shell history, configuration, or diagnostics.

Deliverables:

- sealed non-convertible `SecretInputSource` classes are
  `InteractiveNoEcho`, `OwnedSecretDescriptor`, `CredentialProvider`, and
  `GeneratedSecret`; callers select a source but never pass secret bytes through
  an ordinary string/argument/config API;
- passphrase/bootstrap-secret values are rejected from command arguments,
  environment variables, ordinary stdin/redirection, realm/local config,
  response files, shell completion, logs, tracing, telemetry, panic text, error
  context, diagnostic bundles, and debug/display serialization;
- interactive input requires a controlling terminal/console, disables echo using
  native platform APIs, applies exact bounded byte/character limits, confirms
  newly created secrets through a second independent read, and restores terminal
  state on success, mismatch, cancellation, handled signal, error, and unwind;
- redirected/non-terminal input fails unless the user explicitly selects an
  admitted owned descriptor/handle source; the CLI never silently treats a pipe
  or redirected ordinary stdin as an interactive secret;
- owned descriptor/handle source binds an explicitly supplied descriptor number
  or inherited handle identity, validates admitted ownership/access/inheritance
  properties where the OS exposes them, refuses path reopening and ambient file
  lookup, reads one bounded framed secret, rejects truncation/trailing data, and
  closes or relinquishes it under declared ownership semantics;
- credential-provider source uses an admitted provider identifier and scoped
  short-lived `CredentialSecretLease` or non-exportable
  `ProviderDerivedKeyHandle`; these are distinct non-convertible result classes,
  provider metadata is non-secret, raw output exists only for the sanitized
  lease class, and cancellation/retry cannot duplicate either indefinitely;
- generated source uses the admitted OS-CSPRNG boundary and is the default where
  an admitted credential provider can retain it or an explicit owner-only secret
  output descriptor can receive it; generated bytes never go to ordinary stdout,
  logs, diagnostics, clipboard, or an implicit file;
- generated-secret delivery is a typed ceremony
  `Generated -> DeliveryPending -> Delivered -> PossessionConfirmed`; a
  generated credential cannot become bootstrap material, publish a header, or
  authorize genesis while merely delivered or retained-but-unconfirmed;
- provider delivery confirms possession by retrieving the exact scoped
  credential into an admitted lease or performing a domain-separated challenge
  derivation through a non-exportable provider operation; successful provider
  write/acknowledgement alone is not confirmation;
- owner-only output-descriptor delivery requires a separate explicitly admitted
  confirmation descriptor or provider channel and a fresh domain-separated
  challenge-response bound to the ceremony, generated-credential commitment,
  receiver, and delivery attempt; the secret-derived response is compared in
  sanitized memory and never retained in public metadata or diagnostics;
- sealed `PendingCredentialStorage` admits exactly `ProviderHeld`,
  `VolatileLocked`, and `PlatformSealed`; source selection is explicit before
  generation and cannot silently fall back to a weaker or persistent mode;
- `ProviderHeld` means an admitted credential provider retains the secret under
  an independently established scoped handle and declared retrieval/challenge,
  retention, deletion, recovery, rollback, and assurance semantics; the handle
  and provider root pre-exist and do not depend on the pending realm bootstrap;
- `VolatileLocked` keeps the credential only in bounded guarded/locked process
  memory under v0.63.1 limitations; no recoverable bytes cross a process crash,
  and restart requires exact user/provider reacquisition through an admitted
  source or abort before authority publication;
- `PlatformSealed` permits persistent staging only as authenticated ciphertext
  under an independently provisioned OS-keystore, HSM, or key-agent key whose
  identity, purpose, assurance, availability, rollback, backup, and recovery
  semantics are declared; the exact admitted whole-record AEAD/wrapping suite
  and key handle are explicit, and associated data binds retained store,
  ceremony, profile, credential commitment, receiver, delivery attempt, platform
  key, provider/platform protection context, suite, and staging generation;
- realm/WAL/bootstrap keys, the pending credential itself or any value derived
  from it, public ceremony data, deterministic machine attributes, timestamps,
  counters, ordinary files, and unauthenticated obfuscation cannot protect
  persistent pending credential bytes;
- durable WAL/ceremony state stores only the domain-separated credential
  commitment, storage-mode/provider-or-platform-handle metadata, receiver,
  delivery-attempt ID, status, challenge state, and protected-staging ciphertext
  reference where applicable, never recoverable secret bytes or a self-derived
  staging key;
- pending bytes or provider/platform handles remain available until possession
  confirmation and the consuming bootstrap transaction are durable; only then
  may staging ciphertext be deleted or the scoped handle retired under explicit
  idempotent cleanup evidence, with uncertain cleanup and residual snapshots/
  provider copies reported rather than treated as completed erasure;
- crash recovery resumes only with the exact provider-held, volatile-reacquired,
  or platform-unsealed credential and verifies it against the pending commitment;
  unavailable independent protection, mismatch, rollback, or missing recovery
  path aborts before authority publication and never silently generates a
  replacement;
- possession challenge suite is
  `sagnir:pending-credential-possession:hmac-sha3-256:v1`; it uses the complete
  canonical generated ASCII credential as the HMAC-SHA3-256 key and a fresh
  32-byte OS-CSPRNG challenge; its canonical length-framed transcript binds
  suite/version, retained store and ceremony IDs,
  bootstrap profile, credential commitment, storage/source class, receiver,
  delivery-attempt ID, challenge, challenge sequence, and replay domain;
- the response is exactly 32 bytes, is read through the separately admitted
  confirmation channel or provider operation, and is compared with the locally
  expected response using the admitted constant-time comparison boundary; short,
  long, repeated, stale, cross-store, cross-receiver, cross-attempt, and cross-
  suite responses fail before possession state advances;
- each challenge sequence is durably reserved before release, may be answered
  once, and becomes consumed or failed without reuse; only the non-secret result
  and transcript commitment are retained, never the expected/received MAC;
- generating salts or generated secrets fails closed on OS-CSPRNG failure;
  user-supplied password quality is policy input based only on observable rules
  such as minimum length or a reviewed denylist, and Sagnir never claims to
  measure the actual entropy of a human-selected password;
- interactive input converts an exact sequence of Unicode scalar values to
  strict UTF-8 bytes with no NFC/NFD/NFKC/NFKD normalization, case folding,
  trimming, locale conversion, or lossy replacement; canonically equivalent
  composed/decomposed text therefore remains intentionally byte-distinct;
- Unix-like terminal input must decode as strict UTF-8; Windows console UTF-16
  is converted by exact Unicode scalar decoding and rejects every unpaired/
  invalid surrogate before UTF-8 encoding; no platform-native code page is an
  admitted passphrase encoding;
- one final LF or CRLF produced by Enter is transport framing and excluded from
  the secret, not trimming; standalone CR, embedded CR/LF, NUL, C0 controls, and
  C1 controls are rejected for interactive text while ordinary spaces and all
  other admitted Unicode scalars are preserved exactly;
- owned secret-descriptor framing is exactly little-endian `u32 byte_length`
  followed by that many bytes and immediate EOF; zero/oversize length,
  truncation, trailing newline/data, length overflow, and multiple frames fail
  closed, and arbitrary non-UTF-8 bytes are admitted only for a wrapper/KDF suite
  that explicitly declares an opaque-byte secret class unavailable to terminal
  input;
- generated opaque bootstrap credentials use exactly 32 OS-CSPRNG bytes encoded
  as ASCII `saga1_` followed by RFC 4648 base64url without padding; they are
  intended for exact credential-manager or owner-controlled storage, not manual
  transcription, memorization, or manual-recovery claims;
  the complete canonical ASCII representation is the KDF input across terminal,
  descriptor, and credential-lease sources, and alternate padding/alphabet/
  case/spelling is rejected rather than normalized;
- source-equivalence vectors prove equal admitted UTF-8/generated bytes produce
  identical KDF input across Unix terminal, Windows console, descriptor, and
  credential lease, while visually similar but byte-distinct input does not;
- exact interactive scalar/byte maxima, descriptor byte maximum, generated
  length, and KDF input maximum are checked before allocation/derivation;
  confirmation compares the exact admitted post-framing byte sequence;
- type-level compatibility matrix separates `LocalKdfSecretSource`
  (`InteractiveNoEcho`, compatible `OwnedSecretDescriptor`,
  `CredentialSecretLease`, `GeneratedSecret`) from
  `ProviderKeyHandleSource` (`ProviderDerivedKeyHandle` plus provisioning
  evidence); no generic source enum can be converted between those classes;
- `PassphraseDerivedBootstrap` accepts only `LocalKdfSecretSource` and always
  executes the exact local KDF; `ExternallyProvisionedEncryptedBootstrap`
  accepts only `ProviderKeyHandleSource`; unlock accepts only the source/result
  class admitted by its exact passphrase-wrapper or recipient/provider suite;
  incompatible combinations fail before prompting, reading, leasing, generating,
  or otherwise acquiring secret material;
- in-memory secret containers are non-`Clone`, non-`Copy`, redacted, and use the
  sanitization crate for every owned buffer and temporary; documentation retains
  the v0.63.1 limits for compiler/OS copies, crash dumps, swap, ptrace, provider
  memory, and intentionally released/generated recovery material;
- source selection, descriptor numbers, provider IDs, KDF profile, and controlled
  status may be logged, but secret length/content, confirmation difference,
  quality-test detail that leaks the value, and derived key material may not;
- cancellation/panic/signal/confirmation-mismatch paths drop sanitized buffers,
  revoke provider leases where supported, restore terminal mode, and return
  stable non-secret errors; process abort/power loss limitations are explicit;
- PTY/console, redirected input, descriptor ownership/framing, environment/
  argument/config rejection, generated-secret destination, credential-provider,
  receiver close/partial delivery, retained-but-unconfirmed credentials,
  provider retrieval/challenge failure, independent confirmation-channel
  substitution, all pending-storage modes, self-derived staging-key rejection,
  unavailable platform key, staged-secret/store/platform-context substitution,
  copied staging on another machine refused unless the exact declared recovery
  context admits it, snapshot rollback, crash after delivery, exact resume,
  cleanup ambiguity, MAC transcript/length/replay/domain vectors, constant-time
  comparison, and regeneration refusal,
  source/profile incompatibility, non-ASCII/composed/decomposed text, NUL/control,
  LF/CRLF/standalone-CR, invalid UTF-8/UTF-16, descriptor non-UTF-8/trailing data,
  generated-secret canonical encoding, maximum length, cross-source equivalence,
  confirmation mismatch, cancellation, panic/unwind, signal, terminal restore,
  allocation failure, and sanitization instrumentation tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-crypto`
- secret-source API compile-fail/redaction suite;
- independent pending-credential HMAC transcript known-answer, malformed,
  response-length, domain-substitution, and replay vectors;
- Unix PTY and Windows console/input-handle integration suites where available;
- cancellation/panic/sanitization fault injection.

Exit criteria:

- Production passphrases and generated bootstrap secrets enter only through one
  admitted source boundary and never through argv, environment, ordinary config,
  logs, diagnostics, or implicit redirected input.
- Generated secrets depend on OS-CSPRNG health; human password entropy remains
  an honest unknown constrained only by explicit policy and KDF cost.
- A generated credential is consumable only after exact possession confirmation;
  delivery loss, confirmation failure, or unrecoverable pending state aborts
  before authority publication and cannot trigger replacement generation.
- Persisted pending credentials are protected only by independently established
  provider/platform keys; volatile mode persists no recoverable bytes, and the
  normative possession transcript is replay-resistant across every bound scope.
- Cross-platform interactive and generated-secret KDF bytes are normative, and
  incompatible source/profile pairs are rejected before secret acquisition.
- Every recoverable exit restores terminal state and sanitizes owned secret
  buffers without overstating protection against privileged/process-memory or
  previously copied-secret attackers.

### v0.99.0 - Device Recipients And Recipient Slots

Goal: define device access and recovery without one shared user secret.

Deliverables:

- per-device recipient keys;
- recipient ID, kind, exact wrapping suite, and wrapped key metadata;
- admitted HPKE-style labeled suite/context construction binds KEM, KDF, AEAD,
  mode, opaque recipient selector, encapsulated key, wrapped DEK, realm,
  compartment, crypto/key epoch, manifest root, slot ordinal, and purpose in
  canonical associated data; bespoke unlabelled concatenation is prohibited;
- post-quantum or hybrid recipient wrapping uses its own admitted specification
  and cannot claim RFC 9180 compatibility merely by copying its field shape;
- recipient slots are padded to declared buckets and shuffled canonically or
  randomly according to the privacy profile so raw slot count and stable order
  are not unintentionally public;
- signed recipient authorization;
- recipient/key-transparency current state is anchored in the exact v0.17.1
  admitted history-independent algorithm and v0.17.2 persistent map-page format,
  supporting canonical membership, absence, and update proofs;
- transparency history is a separate v0.17.0 append-only event commitment,
  supporting append inclusion and prefix/consistency proofs; map proofs cannot
  be presented as log consistency and log proofs cannot prove current absence;
- canonical transparency-map key, leaf, empty value, inclusion, absence, and
  update encodings are distinct from canonical transparency-event, append,
  inclusion, and consistency encodings with non-interchangeable domain labels;
- each leaf binds actor, device or recipient identity, key material, algorithm,
  lifecycle state, governance epoch, sequence, authorization scope, and
  supersession or revocation reference;
- historical key states remain provable, while current absence and revocation
  cannot be represented by silently deleting a leaf;
- signed transparency checkpoint binds both map root/entry count/algorithm
  version and event-log root/size/frontier/append version plus prior checkpoint;
  omitting or substituting either root invalidates the checkpoint;
- this release admits recipient/transparency schemas, map/log proof semantics,
  and dual-root checkpoint bytes, but production key-state mutation remains
  unavailable until v0.99.1 admits one atomic map/log/head transition;
- monitor replay derives the map state from the separately committed event-log
  prefix and compares the resulting map root/count, while peer gossip and split-
  view evidence compare the complete dual-root checkpoint;
- private-realm transparency leaves, map inclusion/absence proofs, event-log
  append/consistency proofs, dual-root checkpoints, monitor gossip, and split-
  view evidence are encrypted and authenticated only to governance-authorized
  monitors;
- blind stores and unauthorized peers receive no actor, device, recipient, role,
  key, revocation, or monitor-membership metadata from private transparency
  traffic;
- optional public transparency requires an explicit open-realm policy and a
  separate disclosure review; private realms never become public merely to use
  monitoring;
- OS keychain, TPM, secure enclave, and hardware-token backend interfaces;
- threshold and offline recovery metadata;
- anonymous recipient slots where feasible;
- key compromise and recovery evidence;
- acknowledgement that removal cannot revoke already acquired keys;
- backend-unavailable, duplicate slot, unauthorized recipient, ambiguous key,
  false absence, stale map root, split view, inconsistent append, rollback, and
  recovery threshold tests;
- map/log root substitution, map-proof-as-consistency, log-proof-as-absence,
  omitted dual-root checkpoint field, event replay/map-root mismatch, and
  independent map/log version transition tests;
- suite/context substitution, slot reorder/ordinal confusion, selector replay,
  cross-realm/compartment/epoch replay, stripping, padding, and recipient-count
  leakage tests.

Verification:

- `cargo test -p sagnir-crypto`
- backend contract tests with software fixtures.

Exit criteria:

- The format supports device-specific access, revocation evidence, and offline
  recovery without embedding a mandatory platform backend.
- Key transparency uses the admitted map only for current membership/absence/
  update state and the append-only event structure only for history/consistency;
  one signed dual-root checkpoint prevents either view from silently hiding or
  rewriting an admitted key transition.
- Private-realm key monitoring does not turn actor or recipient metadata into a
  public directory.

### v0.99.1 - Atomic Transparency State Transitions

Goal: make every authorized recipient/key transition update current map state,
append-only history, and the dual-root state head as one recoverable authority
transaction.

Deliverables:

- canonical `TransparencyStateHead` binds realm, map algorithm/page version,
  map root and entry count, event-log algorithm/version, log root and size,
  governance/key epoch, monotonic transparency transition sequence, and previous
  state-head or signed dual-root checkpoint commitment;
- canonical `TransparencyTransition` binds transition ID/type, expected prior
  state-head, exact prior map root/count and event-log root/size, authorized key/
  recipient mutation, key, previous leaf/value commitment or proven absence,
  resulting leaf/tombstone commitment, resulting map root/count, appended event
  commitment, resulting log root/size, governance/key epoch, transition sequence,
  signer/authorization transcript, and resulting state-head commitment;
- key addition, replacement, scope change, suspension, revocation, recovery, and
  governed removal each define one exact map mutation and one matching event;
  no-op event append, unlogged map mutation, mutation/event subject mismatch, or
  silent leaf deletion is an invalid transparency transition;
- map copy-on-write pages and append-log nodes are prepared as immutable
  unreachable bytes under v0.17.2/v0.17.0 limits, then independently verified
  against the transition's resulting roots/counts before authority publication;
- every transition authorization/signature explicitly inherits v0.23.4: before
  signer contact, Sagnir durably reserves the transition ID, exact complete
  old/new signing transcript, expected state head, provider/key/epoch, suite,
  governance context, and one provider idempotency key, then consumes one sealed
  `AuthoritativeOperationCapability` for that exact request;
- before reservation or signer contact, local admission rejects an expected head
  already known stale; a byte-identical candidate already reserved, signed, or
  terminal returns its existing state without signer contact, and at most one
  in-flight signing reservation exists per `(realm, expected head, transition
  scope)` unless canonical policy explicitly grants a larger bounded concurrency
  value; no policy value may exceed the implementation hard maximum;
- canonical resource policy defines checked hard limits per actor, device,
  replica, and realm for `Reserved`/in-progress, `Ambiguous`, and active
  superseded transparency operations plus prepared bytes/pages/log nodes; exact
  defaults and non-bypassable maxima live in a versioned resource table;
- inbound transparency data has three sealed non-convertible states:
  `UntrustedCandidate` is transient transport/preflight input with no durability,
  verification claim, or authority; `BoundedQuarantinedCandidate` is an exact
  integrity-bound candidate accepted under local quarantine reservations but is
  not authority evidence; `AdmittedAuthorityEvidence` is a locally reserved or
  fully verified/WAL-admitted operation protected by authority retention rules;
- the no-silent-drop rule applies only to `AdmittedAuthorityEvidence`, including
  already admitted ambiguous and superseded operations; an untrusted or merely
  quarantined signed candidate cannot force authority retention by carrying a
  valid signature, claiming an actor, or repeating an admitted transition shape;
- before durable quarantine acceptance, the receiver atomically reserves hard
  item, encoded/captured byte, signature-count, signature-verification-work,
  decode/work, prepared-page/node, and total-quarantine budgets for the exact
  candidate; checked cumulative accounting includes framing, metadata, proofs,
  signatures, pages, and every temporary or expanded representation;
- pre-authentication capacity aggregates across the store, realm hint/opaque
  scope where available, peer/endpoint, bundle, transport connection, and all
  concurrent sessions; after authenticated identity is known, actor, device,
  replica, and realm limits additionally apply, and identities never multiply a
  store/realm aggregate ceiling;
- unavailable capacity returns stable `ResourceLimit` before durable quarantine
  admission; no partial candidate, signature, prepared page, verification result,
  resume checkpoint, or quota charge remains, and the candidate is not reported
  as evaluated, invalid, denied by policy, or authoritative rejection evidence;
- the sender retains responsibility for preserving and later resubmitting input
  that never became `BoundedQuarantinedCandidate`; reconnect, retry, another
  bundle, or another claimed identity receives the same aggregate accounting and
  cannot recover or expand an earlier refused receiver-side allocation;
- a receiver may retain only a separately budgeted bounded abuse digest/receipt
  over transport context and refused bytes; it is explicitly non-authoritative,
  may be summarized/rotated under local policy, and cannot prove candidate
  validity, signature verification, policy rejection, transition existence, or
  replace the refused transition;
- arrival order may decide which untrusted transport attempt receives available
  quarantine capacity, backpressure, or refusal, but cannot select an authority
  transition, resolve competing heads, consume another candidate's authority
  quota, or alter canonical CAS/policy ordering;
- offline/concurrent candidates accepted into bounded quarantine reconcile under
  the same hard aggregate budgets; excess remains sender-held or receives
  `ResourceLimit`, while already admitted ambiguous/superseded evidence is never
  evicted and continues through its terminal/fence/archive path;
- a returned signature and provider-result commitment are persisted through the
  authority transaction substrate before final map/log/head CAS; pre-admission
  refusal, invalid signature, or cancellation leaves the old head authoritative,
  while a lost or uncertain post-admission response records `Ambiguous` and must
  reconcile against the provider/key-agent operation journal before retry,
  cancellation, abandonment, or any new authorization;
- exact provider reconciliation returns the prior result/refusal or remains
  ambiguous and never signs the transcript again merely because a response was
  lost; an unqueryable ambiguity requires governed/manual recovery under v0.23.4;
- one v0.23.3/v0.32.0 WAL transaction compare-and-swaps the exact expected prior
  `TransparencyStateHead`, records the transition/event, publishes the map pages
  and log nodes, and commits the new head; concurrent transitions from one old
  head cannot both become authoritative;
- data/page/node bytes and all required directory entries are durable before the
  commit exposes the new head; recovery yields the complete old pair or complete
  new pair and never a new map with old log, old map with new log, or head whose
  referenced bytes are unavailable;
- duplicate transition/event IDs, repeated transition sequence, stale expected
  head, event-log replay, and exact idempotent retry have deterministic outcomes;
  retries cannot append the event twice or reapply the map mutation;
- if a competing transition wins the expected-head CAS after this transition was
  signed, the exact signature, result commitment, reservation, and stale signed
  transition remain authenticated superseded evidence; they cannot be discarded,
  rewritten, or rebased onto the winning/new head;
- reconciliation records terminal `SupersededAfterSignature` only after the exact
  signing result is known and the expected head is provably no longer current;
  this state preserves operation/transcript/result commitments, signer identity,
  superseding head/transition, and dispute status and can never return to a
  signable, publishable, or ambiguous state;
- each unreachable prepared map page/log node set has a bounded authenticated
  lease tied to its operation, expected head, root commitments, ambiguity state,
  byte/page counts, retention class, and expiry frontier; lease renewal is
  policy-bounded and cannot bypass actor/device/realm quotas;
- prepared bytes remain pinned while needed to resolve signer ambiguity or an
  admitted publication attempt; once safe under ambiguity, dispute, retention,
  and legal-hold policy, canonical map/log bytes may be deterministically rebuilt
  and reverified from retained transition inputs/root commitments, so unreachable
  prepared bytes become GC-eligible instead of permanent active state;
- causally stable `SupersededAfterSignature` operations move through the active/
  covered-fence/exception/archive path admitted by v0.23.6 and activated by
  v0.52.0; archive compaction retains the signed transition, signature/provider-
  result/transcript commitments, supersession proof, quota/accounting lineage,
  and required dispute evidence without retaining unreachable prepared pages or
  log nodes indefinitely;
- `Ambiguous` operations remain active and quota-charged until v0.23.4 provider
  reconciliation or governed/manual recovery reaches an admitted terminal state;
  resource pressure, lease expiry, or archive demand cannot convert ambiguity
  into supersession, success, cancellation, or evidence deletion;
- signed periodic transparency checkpoints consume one admitted state head and
  bind both roots/counts/versions; monitor replay checks the atomic history but
  is detection/evidence, never a repair mechanism for locally split authority;
  checkpoint signing uses the same v0.23.4 durable reservation, sealed
  capability, idempotency, result persistence, ambiguity, reconciliation, and
  stale/superseded-evidence rules plus the same in-flight/resource limits;
- algorithm/version changes use a governed transition binding old/new map and
  log suites together; changing only one side or downgrading either fails closed;
- crash/fault injection covers every page/node write, file/directory sync,
  signer request/result, WAL frame/commit, state-head publication, response loss,
  retry, concurrent compare-and-swap, and recovery boundary;
- tests cover concurrent updates, signer failure, duplicate event, event without
  mutation, mutation without event, wrong previous leaf/absence proof, map/event
  subject mismatch, map-only/log-only version change, stale head, missing page/
  node, signer completion followed by response loss, ambiguity reconciliation
  before and after a competing CAS, stale signed transition retention, checkpoint
  signer ambiguity, repeated exact retry, locally stale pre-sign refusal, stale-
  head signing floods, concurrent authorized replicas, quota exhaustion and
  merge quarantine, unlimited validly signed stale candidates, many identities
  sharing one realm/store budget, reconnect/resubmit floods, quarantine restart
  accounting, partial-candidate cleanup, refusal-to-authority-evidence confusion,
  permanently ambiguous operations, lease expiry/rebuild/GC, stable superseded
  archival and recovery, and interruption between every reservation/result/
  quarantine/publication/archive boundary.

Verification:

- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-store`
- v0.23.4 transparency-signing capability/reservation and ambiguous-provider
  reconciliation suite;
- atomic transparency transition state-machine model;
- dual-root transition/checkpoint independent vector validator;
- process-kill and concurrent-CAS integration suite.

Exit criteria:

- One accepted key transition moves map, append log, and state head together or
  leaves all three at the prior state.
- Recovery never relies on monitor replay to repair a locally split map/log pair.
- Concurrent, duplicate, stale, or partially published transitions cannot hide,
  duplicate, or apply a key event without its exact current-state mutation.
- A lost signer response never mints a second signing authority or silently
  repeats provider work; stale signed results remain evidence and are never
  reinterpreted as signatures over a rebased transition.
- Contention and malicious authorized clients cannot create unbounded local
  signing reservations, active superseded evidence, or unreachable prepared
  bytes; bounded archival removes cold prepared storage without erasing signed
  dispute or provider-operation evidence.
- Untrusted and over-capacity candidates can be refused without retention;
  bounded quarantine cannot grow beyond aggregate local ceilings, and only
  admitted authority evidence receives non-eviction and archival guarantees.

### v0.99.2 - Compartment Translation Format Admission Stop

Goal: admit compartment-root composition, translation disclosure, neutral
identity, and redacted-placeholder formats before v0.100.0 implements them.

Deliverables:

- reviewed algorithm/format admission document with no production
  implementation or compatibility claim;
- inherit the v0.92.1 disposable prototype boundary: reference decoders, fuzz
  harnesses, models, and benchmark prototypes cannot write durable realm data,
  enter production feature graphs, emit authoritative transitions, or create a
  compatibility promise, and use experimental bytes rejected by production;
- prototype code is replaced/deleted or explicitly promoted only through
  post-admission production review and release-gate coverage;
- inheritance of the v0.92.1 realm manifest, compartment logical-root,
  commitment-key epoch, and partial-access proof formats;
- compartment recipient authorization binds the exact opaque compartment
  handle, root epoch, locator-key epoch, index-commitment epoch, encryption
  epoch, permissions, and recovery scope;
- target-only translation attestation canonical format with authorized issuer
  identity and signing epoch, target compartment/root, translation transition,
  audience, purpose, target policy root/epoch, issuance checkpoint, expiry or
  no-expiry rule, revocation state, nonce, and replay domain;
- target-attestation expiry and revocation use the v0.28.0 canonical time and
  revocation substrate, including an issuer-log checkpoint, revocation-map root,
  append-only consistency proof, and policy freshness bound stapled to the
  attestation or verification response;
- issuer key rotation, revocation, compromise, retirement, split view, and
  equivocation preserve historical evidence but fail closed for current
  authority according to the bound policy and checkpoint;
- offline verification can establish only the non-revocation state covered by
  its latest admitted staple; beyond the freshness limit the result is unknown
  or quarantined, never currently valid by assumption;
- exact target-only claims are limited to statements such as "this target root
  was admitted by the named authority under the named transition and policy";
  the attestation does not independently prove hidden source plaintext equality,
  source graph isomorphism, source membership, or bridge completeness;
- policy requiring independent source/target equivalence verification must
  require authorized access to the complete bridge and openings or refuse until
  a post-1.0 admitted hidden-witness proof system exists;
- source-only, target-only, cross-authorized/audit, blind-store, and public
  translation visibility matrix plus revocation and replay behavior;
- separate neutral-domain semantic-commitment, private-locator,
  index-commitment, encryption, wrapping, and recipient-authorization key
  purposes with independent domain labels and epochs;
- neutral recipient and recovery authorization is explicit and never inherited
  merely from access to any compartment;
- neutral encryption instances inherit bounded fanout, retention, legal-hold,
  redaction, erasure, wrapper, receipt, repair, archive, rekey, compromise, and
  recovery rules;
- neutral root and key migration can proceed independently while every
  compartment reference to the old neutral epoch remains historically
  verifiable;
- intentional linkability statement: reusing one neutral semantic identity
  across compartments deliberately reveals equality to every actor authorized
  to observe that neutral identity, and Sagnir cannot later erase previously
  observed cross-compartment correlation;
- canonical typed `RedactedPlaceholder` object format containing target realm
  and compartment domain, placeholder type/version, creating transition,
  causal parents, target path/object role, redaction state predicate, policy
  root/epoch, and encrypted audit-provenance reference;
- target-visible placeholder bytes contain no source semantic commitment,
  private locator, ciphertext selector, path, actor, redaction reason, or stable
  source correlator;
- the placeholder makes no content-equality, availability, completeness,
  durability, repairability, or proof-satisfaction claim and cannot satisfy a
  required body, promised object, build input, test input, or availability
  receipt;
- stale ciphertext, an old body, repair response, archive restore, or offline
  peer cannot replace the placeholder without a new explicitly authorized
  reintroduction transition and new encryption instance;
- governed audit views may resolve the encrypted provenance reference to the
  source redaction tombstone and translation decision without disclosing it to
  target-only recipients;
- independent vectors for compartment authorization, target-only attestation,
  neutral commitments/locators/wrappers, and `RedactedPlaceholder`;
- format-specific reference decoders, seed corpus, and fuzz targets for
  compartment authorization, bridge manifests, target-only attestations and
  revocation staples, neutral-domain objects/keys/wrappers, and
  `RedactedPlaceholder`;
- early go/no-go benchmark thresholds for target-attestation verification,
  revocation/consistency proof verification, one-object and representative
  subtree translation planning, neutral-object DEK unwrap, and placeholder
  materialization refusal;
- format admission fails when p95 latency, peak memory, proof size, or
  translation amplification exceeds declared thresholds;
- disclosure, replay, revocation, cross-domain key substitution, neutral
  correlation, stale/forked revocation staple, offline freshness expiry,
  issuer equivocation, stale-body replacement, and false-proof tests;
- explicit implementation stop: v0.100.0 and v0.117.0 may not persist
  translation, neutral, or placeholder bytes until this admission is complete.

Verification:

- independent compartment/translation vector validator;
- target-only claim and disclosure review;
- neutral-domain lifecycle state-machine review;
- redacted-placeholder non-resurrection model;
- format-specific fuzz smoke suite;
- admission benchmark runner and recorded threshold artifact.

Exit criteria:

- Target-only attestations state exactly which authority assertion they carry
  and never impersonate independent verification of a hidden source relation.
- Neutral objects have a complete key, access, retention, erasure, rekey, and
  recovery lifecycle with explicit intentional linkability.
- Redacted placeholders are canonical non-content objects that cannot satisfy
  body, proof, availability, repair, or completeness requirements.
- Target-attestation current validity is backed by stapled monotonic
  revocation/time evidence with explicit offline freshness limits.
- Translation formats pass early parser-hardening and performance rejection
  thresholds before implementation begins.

### v0.100.0 - Compartment Encryption Boundaries

Goal: establish path, world, and projection access boundaries before encrypted
realm creation.

Deliverables:

- compartment ID and key metadata;
- recipient wrapping of only authorized compartment keys;
- prohibition on giving partial-access recipients a realm master key that can
  derive every compartment;
- one logical root and index-commitment epoch per compartment, composed through
  the v0.92.1 opaque authenticated realm manifest;
- partial-access recipients verify compartment inclusion and consistency
  without receiving other compartment roots, locators, entry counts, tree
  shape, names, or membership;
- compartment-root migration and recovery operate independently and update the
  realm manifest through signed compare-and-swap transitions;
- `saga vault compartment create`;
- `saga vault protect`;
- `saga vault unprotect`;
- partial unlock metadata;
- cross-compartment copy and move are signed transitions, never ordinary
  renames, because compartment identity is part of the semantic commitment;
- the transition binds source compartment, expected source frontier, exact
  source root identity, target compartment, expected target absence or explicit
  permitted replacement identity, target policy root and epoch, operation kind,
  actor authority, causal parents, and the resulting target graph root;
- recursive reachable-graph translation manifest listing every source
  commitment, object kind, source references, target commitment, target
  references, private locator, encryption instance, DEK identifier, ciphertext
  selector, translation dependency, and shared-subgraph identity;
- Merkle-chunked translation manifest with bounded canonical chunk size, entry
  count, reference fanout, dependency depth, total-count commitment, chunk root,
  and final manifest root;
- streaming manifest construction and verification with durable resume
  checkpoints binding source/target/policy CAS inputs, completed chunk roots,
  pending frontier, temporary object roots, and tool/version state;
- cancellation and restart semantics preserve no authoritative partial target;
  temporary translated objects, mappings, openings, and encrypted index entries
  remain transaction-scoped and GC-pinned until atomic final commit or bounded
  cleanup after cancellation;
- the final transition atomically commits the complete manifest root, target
  graph root, target encrypted-index roots, target policy result, and source
  logical-removal decision; no chunk or intermediate root becomes a partial
  authoritative move;
- every descendant whose semantic commitment or reference is compartment-bound
  is rebuilt under the target compartment; translating only the tree root is
  prohibited;
- shared subgraphs are translated once per compatible target-policy domain and
  referenced through one manifest entry; incompatible target retention,
  recipient, legal-hold, or erasure policy produces separate encryption
  instances;
- cycles are rejected when the canonical object type requires a DAG; a future
  type that explicitly admits cycles must use a reviewed strongly connected
  component translation with one atomic component commitment;
- atomic source-to-target authenticated mapping covers every translated
  descendant and proves the resulting target root has no reachable
  source-compartment commitment, locator, ciphertext selector, or
  compartment-bound reference;
- complete bridge manifests are encrypted only to actors authorized for both
  source and target compartments or to an explicitly governed cross-compartment
  audit role;
- target-only recipients receive the target state plus a minimal target-scoped
  attestation that binds the admitted transition and target root without source
  identifiers, source graph shape, shared-subgraph relationships, or source
  history;
- the target-scoped attestation is the v0.99.2 authority assertion signed by an
  admitted issuer and bound to issuer epoch, target root, transition, audience,
  purpose, policy, checkpoint, expiry, revocation, nonce, and replay domain;
- target-only verification reports that the named authority admitted the target
  under the named policy; it never reports independent cryptographic proof of
  hidden source equality or graph isomorphism;
- a policy requiring independently verified source equivalence requires bridge
  access and source openings or refuses pending a post-1.0 hidden-witness proof
  system;
- source-only recipients receive no target commitments, locators, selectors, or
  graph mapping, and blind stores receive neither side of the translation
  relationship;
- translation attestations use audience-, purpose-, compartment-, transition-,
  and epoch-bound encryption so repeated copy/move operations cannot become a
  public source/target correlation oracle;
- compartment-neutral objects use a separate typed random-blinded semantic
  commitment domain that omits compartment identity by explicit format design,
  never by reusing or truncating an ordinary compartment commitment;
- neutral objects use dedicated locator, index-commitment, encryption, wrapping,
  recipient, recovery, and epoch lifecycles admitted by v0.99.2;
- access to one compartment does not imply access to neutral objects, and access
  to neutral objects does not imply access to any compartment;
- neutral encryption instances obey the same retention, legal-hold, redaction,
  erasure, receipt, repair, archive, rekey, compromise, and recovery controls as
  compartment-bound instances;
- policy and UI explicitly report that one neutral identity reused by multiple
  compartments creates intentional cross-compartment linkability that cannot be
  recalled after observation;
- neutral-object eligibility is an allowlisted canonical object type with no
  compartment-bound path, recipient, policy, retention, legal-hold, encryption,
  locator, or actor metadata and no reference to a compartment-bound object;
- neutral objects may reference only admitted neutral objects under the same
  neutral-domain version, and every ordinary compartment object references a
  neutral boundary through an explicitly typed edge whose policy admits that
  exact neutral object kind;
- typed canonical transformation relation by object kind:
  - leaf/blob objects prove authenticated logical content and admitted metadata
    equality while receiving new compartment-bound commitments and encryption;
  - trees and containers prove structural isomorphism under the complete
    source-to-target manifest mapping, preserving canonical entry ordering,
    names, object kinds, multiplicity, and non-compartment metadata;
  - every source reference is matched to exactly the declared target reference,
    and no undeclared or source-compartment reference is introduced;
  - metadata intentionally transformed for target compartment, recipient,
    retention, policy, encryption, or projection is typed, explicitly committed,
    and verified under a named transformation rule;
  - container canonical bytes are expected to change when child commitments or
    transformed metadata change and are never described as byte-identical;
- target creation uses a fresh OS-CSPRNG blinding value, independently random
  DEK for each required encryption instance, target-compartment locator domain,
  and target-policy evaluation for recipients, retention, legal hold,
  redaction, deduplication, and delta reuse;
- authorized verification applies the typed relation: content equality for
  leaves, structural-isomorphism and reference-mapping proofs for containers,
  and explicit metadata-transformation rules, without making source/target
  membership publicly correlatable;
- copy preserves the source object and source references; move records an
  explicit source logical-removal transition after target durability, but does
  not imply cryptographic erasure of the source encryption instance;
- source cryptographic erasure, when requested, is a separate v0.122.0
  redaction operation subject to all remaining references, retention, legal
  hold, backup, receipt, and residual-copy rules;
- source-bound reviews, proofs, test evidence, approvals, signatures, and policy
  decisions remain historical evidence for the source identities and do not
  automatically authorize, review, promote, or prove the newly translated
  target identities; target policy explicitly decides which evidence may be
  cited and which must be regenerated;
- move preparation and commit use compare-and-swap over the expected source
  frontier and exact source root, expected target absence or permitted
  replacement, and target policy root;
- a changed source, occupied or changed target, stale target policy, or
  concurrent move creates an explicit compartment-move conflict head; Sagnir
  preserves all contenders, performs no last-writer-wins overwrite, and never
  removes newer source state by arrival order;
- conflict resolution is an authorized multi-parent transition that names every
  known source/target contender and reruns recursive translation and target
  policy against the selected current roots;
- crash-safe transaction ordering makes the target durable and verifiable
  before source logical removal, and interruption cannot leave a signed move
  whose target object or encrypted indexes are incomplete;
- historical signatures and references continue to bind the source
  compartment identity; the target is new signed history and never presented
  as the object originally signed in the source compartment;
- compartment, cross-compartment deduplication, unauthorized derivation,
  rename-as-move rejection, root-only translation rejection, descendant
  source-reference leakage, false container byte-equality, leaf-content
  substitution, structural non-isomorphism, missing/duplicate reference mapping,
  undeclared metadata transformation, shared subgraph, cycle, source-evidence
  carryover refusal, wrong-target policy, stale source CAS, occupied target CAS,
  concurrent move, multi-head conflict, manifest chunk omission/reorder,
  oversized chunk, interrupted stream, resume-state substitution, cancellation,
  temporary-pin loss, complete-manifest disclosure to target-only/source-only/
  blind actors, repeated-translation correlation, forged minimal attestation,
  false hidden-source proof claim, expired/revoked/replayed target attestation,
  unauthorized neutral recipient, neutral key-domain substitution, neutral
  rekey/recovery, intentional-linkability disclosure,
  neutral/compartment commitment confusion, prohibited neutral metadata,
  neutral-to-compartment reference, partial target, target-before-source,
  source-before-target, copy, move, and move-plus-redaction tests.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`
- independent typed translation-relation and Merkle-manifest vector validator;
- bounded translation construction/resume state-machine model.

Exit criteria:

- A recipient authorized for one compartment cannot derive keys, deduplication
  identities, or delta bases for another compartment.
- Moving content across compartments creates a new target identity and
  encryption instance while preserving the original signed source identity.
- A translated target graph contains no reachable source-compartment identity
  or selector, and source reviews or proofs do not silently authorize it.
- Translation disclosure is least-privilege: only a cross-authorized actor can
  inspect the complete bridge, while target-only, source-only, and blind actors
  receive no unnecessary cross-compartment relationship.
- Compartment-neutral identity is a separate constrained format, not an
  exception that weakens ordinary compartment binding.
- Translation proves leaf content equality and container structural isomorphism
  under an exact reference mapping; it never claims containers with rewritten
  child identities are byte-identical.
- Large translations are bounded, Merkle-chunked, resumable, cancellable, and
  authoritative only after one atomic final root commit.
- A failed or interrupted move never removes the source before a complete,
  policy-admitted target is durable.
- Concurrent source, target, or policy changes produce explicit conflict state
  rather than overwrite or source removal by arrival order.

### v0.101.0 - Sealed-Private Migration And Leakage Accounting

Goal: define safe conversion of an existing open realm before exposing the
encryption command.

Deliverables:

- open-to-sealed migration plan that preserves the original public canonical
  object bodies, IDs, roots, references, and historical signatures as the
  history that was actually signed;
- new random-blinded private semantic graph and encrypted lookup/storage
  projections admitted through a signed migration transition binding the
  original public roots to the new private roots;
- crash-safe staged repack and rollback;
- old plaintext identifier and metadata cleanup policy;
- explicit leakage statement for information previously published, replicated,
  logged, cached, or observed;
- prohibition on claiming that migration recalls prior disclosures;
- old-public-to-new-private commitment map protected inside the encrypted
  ledger and never represented as proof that historical signatures originally
  signed the new private graph;
- interrupted migration, stale index, old-pack retention, rollback, and
  previously disclosed metadata tests.

Verification:

- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-store`
- migration crash-consistency suite.

Exit criteria:

- An open realm can be rewritten into sealed-private form without mixing old
  public identifiers into the new external representation.
- Sagnir states clearly that prior disclosure cannot be undone.

### v0.101.1 - Authority Log Encryption Cutover

Goal: move an existing plaintext authority/WAL history into encrypted operation
without rewriting history, losing logical operation state, or pretending prior
metadata disclosure can be recalled.

Deliverables:

- pre-implementation crash/concurrency model uses canonical states
  `PlaintextOpen -> PlaintextFrontierFrozen -> CutoverAnchorSigned ->
  PlaintextTailSealed -> EncryptedSegmentPrepared ->
  EncryptedSegmentPublished -> EncryptedActive`, with explicit abort-before-
  freeze, ambiguous, quarantine, and governed/manual-recovery outcomes;
- one retained-store exclusive migration lease blocks new reservations and
  ordinary WAL writers before the final plaintext frontier is selected; queued
  operations receive retryable refusal and cannot race the cutover;
- all ordinary plaintext transactions and parent-directory publication are
  synced before freezing a closed authority frontier containing a separately
  reserved cutover-signing operation;
- signed `AuthorityLogCutoverAnchor` binds realm/genesis, old format and
  commitment suite, plaintext store/log incarnation, frozen final sequence and
  physical chain root, complete `AuthorityStateRoot`,
  `AuthorityLogCheckpoint`, signing-operation ID, target crypto epoch/suite,
  and intended encrypted successor-segment identity;
- the cutover anchor follows the v0.27.0 non-self-inclusion rule: it signs the
  frozen frontier where its signing operation is `Reserved`; the signing result
  and terminal status are then committed in a final plaintext tail and exact
  predecessor-linked `PlaintextTailSeal` rather than falsely claiming the anchor
  signed its own result;
- the first encrypted segment uses a fresh admitted v0.92.0 key/nonce domain and
  commits to both the signed cutover anchor and terminal plaintext tail seal;
  the next signed realm checkpoint or retained witness anchors the successor and
  covers the cutover-signing result;
- the encrypted segment begins with a bounded authenticated state-carry manifest
  binding the complete active, terminal-fence, sparse-exception, and epoch-
  archive map descriptors, exact counts, required v0.17.2 page commitments,
  separate encrypted-page storage-manifest root, and prior composite
  `AuthorityStateRoot`; it never embeds the lifetime operation map as one record;
- page transfer/re-encryption is streamed under cumulative byte/page/work
  budgets, verifies every plaintext page path and resulting encrypted page, and
  independently proves the encrypted page set decodes to the identical logical
  composite root/counts before activation; encryption changes physical storage
  commitments, not `AuthorityStateRoot`; consumed, cancelled, ambiguous, or
  equivocated IDs retain exact v0.23.6 terminal/exception semantics and no
  reservation sequence becomes reusable;
- encrypted segment bytes, authentication data, file metadata, directory entry,
  and every required parent directory are durable before `EncryptedActive` is
  published or any encrypted operation is accepted;
- old plaintext segments receive a no-append seal and are never rewritten in
  place; retention policy may retain, compact into predecessor-linked evidence,
  archive, or delete eligible physical segments only after the encrypted
  successor and required audit/witness anchors are durable;
- retention and CLI output explicitly state that provider/key IDs, actors,
  replicas, capability classes, purposes, frequencies, ambiguity/recovery
  activity, and any copied plaintext log bytes observed before cutover cannot be
  recalled by later encryption or deletion;
- locked recovery exposes only admitted public framing, cutover state, required
  key epoch, and quarantine reason; it cannot authenticate, replay, interpret,
  or publish encrypted authority state until that WAL key epoch is available;
- recovery selects state only through exact roots, segment identities, durable
  publication markers, and predecessor commitments, never timestamps, filename
  preference, partial directory presence, or whichever segment parses first;
- crash/fault injection covers every final plaintext append/sync, signing request
  and lost response, tail seal, encrypted segment creation/authentication/sync,
  directory publication, old-segment seal, activation marker, retry, cleanup,
  and locked/unlocked recovery boundary;
- tests cover status-map omission/substitution, consumed-ID resurrection,
  ambiguous-record deletion, wrong epoch/suite, predecessor substitution,
  parallel writer/reservation race, stale clone, rollback to plaintext-active,
  page/manifest/count substitution, cumulative-budget exhaustion, partial
  archive/delete, and inability to unlock the encrypted successor.

Verification:

- bounded authority-log cutover state-machine model;
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-crypto`
- process-kill cutover and locked-recovery integration suite.

Exit criteria:

- Exactly one plaintext or encrypted authority-log generation accepts writes at
  every recoverable state, and activation never depends on heuristics.
- The encrypted successor preserves the complete logical authority state and
  cryptographically descends from the sealed plaintext history without
  rewriting historical transaction bytes.
- Cutover work is bounded by active/exception pages plus authenticated fence and
  archive manifests, not by one unbounded lifetime-operation record.
- Sagnir reports that encryption protects future WAL contents; it never claims
  to erase metadata or bytes already observed before cutover.

### v0.101.2 - Encrypted Genesis Bootstrap Profiles

Goal: create encrypted realms without circularly requiring an authority-log
reservation to provision the key needed to encrypt that first reservation.

Deliverables:

- canonical bootstrap-log profile enum admits exactly
  `MinimalPlaintextBootstrap`, `ExternallyProvisionedEncryptedBootstrap`, and
  `PassphraseDerivedBootstrap`; profile ID, version, public header/evidence
  commitment, leakage class, initial segment mode, bootstrap key epoch, and
  replacement policy are bound into the v0.23.5 ceremony and genesis transcript;
- common modeled lifecycle is `ProfileSelected -> BootstrapMaterialReady ->
  InitialSegmentReady -> GenesisReconciled -> RealmWalKeyProvisioned ->
  NormalKeySegmentPublished -> BootstrapKeyRetired -> EncryptedGenesisActive`,
  with profile-specific skipped states and explicit `Aborted`, `Ambiguous`,
  `Quarantined`, and manual-recovery outcomes;
- when `PassphraseDerivedBootstrap` uses a generated credential, its required
  pre-header sub-ceremony is `Generated -> DeliveryPending -> Delivered ->
  PossessionConfirmed -> BootstrapHeaderPublished -> BootstrapMaterialReady`;
  `BootstrapMaterialReady` cannot be reached from `Delivered` alone, and non-
  generated profiles skip these states without fabricating possession evidence;
- generated-credential profile state binds one exact v0.98.2
  `PendingCredentialStorage` mode before generation: `ProviderHeld` references
  independently established provider custody, `VolatileLocked` requires exact
  reacquisition after restart, and `PlatformSealed` requires an independently
  provisioned admitted platform key; genesis cannot establish or authorize the
  protection on which its own pending credential depends;
- `MinimalPlaintextBootstrap` permits only bounded v0.23.5 bootstrap/genesis and
  normal-WAL-key provisioning reservations, public commitments, and provider
  results in the initial plaintext log; object writes, user changes, sync,
  ordinary signing, and all non-bootstrap operations remain unavailable;
- minimal-plaintext bootstrap automatically executes v0.101.1 immediately after
  genesis and normal WAL-key reconciliation, and `saga` reports success only
  after `EncryptedActive`; CLI output and durable metadata disclose the exact
  limited provider/key, actor/device, operation-class, purpose/frequency, and
  recovery metadata that may have been observable before cutover;
- `ExternallyProvisionedEncryptedBootstrap` accepts only an already-created
  non-exportable WAL-key handle from an admitted HSM, OS keystore, or key agent
  plus independently verifiable signed/attested evidence created before the
  first Sagnir log record and bound to retained new-store identity, ceremony ID,
  profile, suite, key purpose, provider identity, expiry/freshness, and anti-
  replay nonce;
- the external profile accepts only the v0.98.2 `ProviderKeyHandleSource`
  result class and its bound provisioning evidence; a credential secret lease,
  terminal value, descriptor value, or generated opaque credential cannot be
  relabeled as an external non-exportable key;
- externally provisioned evidence is verified without treating the not-yet-
  created realm or authority log as its own trust root; unsupported/untrusted/
  stale provider evidence fails before segment creation, and Sagnir does not
  record a fictitious internal reservation for the external provisioning act;
- `PassphraseDerivedBootstrap` atomically publishes a bounded public bootstrap
  header containing retained new-store/ceremony binding, random salt, exact
  admitted KDF/suite parameters, key epoch/purpose, and header commitment before
  deriving a narrowly scoped initial WAL key and writing the first reservation;
- for a generated credential, bootstrap-header publication additionally consumes
  the exact v0.98.2 `PossessionConfirmed` result bound to this ceremony and
  delivery attempt; neither successful descriptor output nor provider storage
  acknowledgement is sufficient, and confirmation evidence cannot be replayed
  across a store, profile, credential commitment, receiver, or ceremony;
- passphrase-derived bootstrap uses v0.98.0/v0.98.1 KDF/anti-oracle contracts and
  accepts only the v0.98.2 `LocalKdfSecretSource` result class and executes the
  exact local KDF; it fails closed on OS-CSPRNG failure for salt/generated-secret
  creation or on KDF failure, changes no authority state on wrong input, and
  never claims a human passphrase failed an entropy measurement that software
  cannot perform;
- the bootstrap secret never becomes or derives the long-term realm master key,
  private-locator key, data-encryption key, normal realm WAL key, recipient key,
  or recovery secret;
- the pre-log passphrase KDF is a narrow local bootstrap primitive over user
  input and the committed public header, not a provider/key-agent operation that
  requires an authority capability or reservation; deployments requiring remote
  or provider-authorized KDF execution must use externally provisioned bootstrap
  material or refuse this profile;
- passphrase-bootstrap confidentiality is explicitly bounded by passphrase
  entropy and the admitted KDF's measured cost; the public salt/header and any
  retained authenticated ciphertext are a permanent offline guess-verification
  target through AEAD authentication for every party that obtains a copy;
- local attempt/rate limits and bounded live-process failure shapes mitigate only
  online abuse and side channels; they cannot slow or observe an attacker testing
  retained header/ciphertext copies offline;
- later normal-key replacement, migration, bootstrap authorization retirement,
  local header/ciphertext deletion, or KDF parameter increase cannot revoke
  copies already obtained or strengthen the passphrase that protected them;
- a bootstrap passphrase must be independently generated and must not be reused
  for the realm master key, recipient/device keys, recovery material, private-
  locator/commitment keys, another realm, or any external account/secret;
- realm/local policy may refuse passphrase-derived bootstrap for regulated or
  high-assurance profiles and require externally provisioned non-exportable high-
  entropy key material; refusal cannot be overridden by relabeling the same
  passphrase-derived bytes as hardware-backed;
- CLI/config/status output distinguishes `passphrase-derived bootstrap
  protection` from `externally provisioned non-exportable key protection`, shows
  the admitted KDF profile and offline-guessing warning, and never reports both
  as equivalent assurance;
- every profile provisions a fresh normal realm WAL key through its admitted
  initial log, publishes a predecessor-linked encrypted successor segment under
  that key, and retains the old bootstrap/plaintext evidence and any required
  bootstrap key handle until a signed checkpoint plus recovery/retention policy
  permits retirement; key replacement never rewrites old segment bytes;
- bootstrap-key retirement means the key/handle loses all write and current-
  state authority; it is not called cryptographic erasure while retained header,
  passphrase, provider recovery, or archive material can recover it, and old
  encrypted bootstrap segments remain verifiable only while their admitted key-
  recovery path or a sufficient successor checkpoint/archive is retained;
- direct encrypted genesis is claimed only for the external or passphrase
  profiles after their prerequisites pass; unavailable provider attestation,
  keystore support, KDF, secure entropy, or durable-header semantics causes
  explicit unsupported/refused status and never plaintext fallback;
- profile selection is explicit in command/config output and dry-run; automation
  cannot silently choose a more revealing profile, while a user may explicitly
  select minimal plaintext after receiving its stable leakage warning;
- generated high-entropy bootstrap input is preferred by default when v0.98.2
  can place it in an admitted credential provider or explicit owner-only secret
  output descriptor; otherwise interactive user input is explicit and no secret
  value is accepted from argv/environment/config;
- crash recovery before header publication resumes only with the exact pending
  generated credential from `ProviderHeld`, exact `VolatileLocked` reacquisition,
  or authenticated `PlatformSealed` staging under its pre-existing platform key;
  unavailable/mismatched keys or material, staging rollback/substitution, or
  missing recovery semantics abort without authority publication, and recovery
  never generates a new credential under the prior ceremony or reports an
  unconfirmed delivery as usable;
- the consuming bootstrap transaction durably binds the possession-transcript
  commitment and exact pending storage generation before publishing the header/
  authority result; provider handle retirement or platform staging deletion runs
  idempotently only afterward and records confirmed, pending, unavailable, or
  ambiguous cleanup without rolling authority back or claiming secret erasure;
- platform matrix records which HSM/keystore, passphrase, and durability profiles
  are supported on Linux, Windows, BSD, MacOS, Android, and iOS; unknown behavior
  is refused rather than inferred from a similar platform;
- crash/fault injection covers profile/header selection, external evidence
  verification, salt/header write and parent sync, KDF, first encrypted/plaintext
  segment creation, provider lost response, genesis signing/publication, normal
  WAL-key provisioning, generated credential delivery/confirmation and pending-
  storage-mode recovery/cleanup, successor publication, bootstrap-key retirement,
  retry, wrong passphrase, and locked recovery;
- tests cover profile substitution/downgrade, plaintext fallback, stale/replayed
  attestation, wrong store/ceremony binding, exportable or wrong-purpose handle,
  malformed/extreme KDF parameters, repeated salt/entropy failure, partial
  header, receiver close/partial delivery, retained-but-unconfirmed credential,
  confirmation/provider-retrieval failure, crash after delivery or confirmation,
  mismatched reacquisition, unavailable independent platform key, self-derived
  staging protection, staging store/platform-context substitution, undeclared
  cross-machine copy and snapshot rollback, cleanup ambiguity, accidental
  regeneration, bootstrap/realm-key aliasing,
  passphrase purpose-separation and reuse-warning output, ordinary operation
  before cutover, old-key early deletion, stable assurance labels, offline-guess
  verifier fixture, and unsupported-platform refusal.

Verification:

- bounded cross-domain model for all three bootstrap profiles;
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-cli`
- process-kill and provider/key-agent bootstrap-profile integration suite.

Exit criteria:

- Every first authority reservation is either intentionally bounded plaintext or
  encrypted by key material established without depending on that reservation.
- Direct encrypted genesis never silently falls back to plaintext, and minimal
  plaintext genesis cannot admit ordinary work before encrypted cutover.
- Bootstrap key material is purpose-separated, replaced by a normal realm WAL
  key, and retained or retired only under explicit checkpointed recovery policy.
- Passphrase-derived protection makes no claim against offline guessing beyond
  the user's passphrase entropy and the exact admitted KDF cost.
- Generated-credential bootstrap cannot publish its header or genesis until the
  selected receiver proves possession, and interrupted delivery can only resume
  through its exact independently protected storage mode with the same pending
  credential or abort safely; staging cleanup starts only after consumption is
  durable.

### v0.102.0 - Encrypt Project Command

Goal: enable sealed-private encrypted realm storage through `saga`.

Deliverables:

- `saga encrypt project`;
- `saga init --encrypted --bootstrap-profile <profile>` requires an explicit
  admitted profile and never selects a more revealing fallback implicitly;
- passphrase/generated-secret source options select only v0.98.2 interactive,
  descriptor, credential-provider, or generated modes; no command option,
  environment variable, response/config file, or ordinary redirected stdin can
  contain the secret value;
- sealed-private vault initialization transaction;
- execute the v0.101.1 authority-log cutover for an existing plaintext realm;
- new encrypted realms must select and complete one admitted v0.101.2 bootstrap
  profile; only external/passphrase profiles may claim direct encrypted genesis,
  while minimal plaintext must finish its automatic cutover before success;
- encryption-enabled canonical event and compiled fact;
- required migration/repack for an existing open realm;
- irreversible-disclosure warning for previously exposed metadata;
- refusal when sealed-private prerequisites or resource budgets are unmet;
- refusal for already encrypted realms;
- dry-run and interrupted migration tests.
- argv/environment/config/redirection rejection, no-echo input, descriptor/
  credential-provider/generated-secret, confirmation, cancellation, and
  terminal-restoration tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-crypto`
- encrypted-init bootstrap-profile integration suite.

Exit criteria:

- A user can enable sealed-private storage only after private locators,
  immutable semantic commitments, protected metadata, recipients, compartments,
  envelopes, and encrypted indexes exist.
- The command never labels metadata-visible encryption as sealed-private.
- New encrypted initialization succeeds only after the selected bootstrap
  profile reaches `EncryptedGenesisActive`; unsupported direct encryption is a
  refusal, not an undocumented plaintext bootstrap.

### v0.103.0 - Unlock Command

Goal: load admitted keys for a local encrypted realm.

Deliverables:

- `saga unlock`;
- consume only the scoped v0.23.4 `UnlockCapability` and refuse any attempt to
  substitute bootstrap, recovery, or authoritative capability classes;
- use the v0.98.1 committed-header target selection, protected-state typestate,
  no-self-authorization, and bounded anti-oracle response contract;
- acquire passphrase input only through v0.98.2 and expose source-selection
  options without accepting secret values in argv/environment/config or ordinary
  redirected stdin;
- unlock session metadata;
- monotonic time-to-live metadata;
- compartment-aware partial unlock;
- `--no-worktree` verification mode;
- wrong-key, expired-session, compartment-overreach, forbidden secret channel,
  cancellation/terminal-restore, and failed unlock tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-crypto`

Exit criteria:

- Sagnir can verify encrypted storage without always materializing plaintext or
  loading unrelated compartment keys.

### v0.104.0 - Lock Command

Goal: evict local unlock state and optionally remove materialized plaintext.

Deliverables:

- `saga lock`;
- key and session eviction;
- local key-agent revocation;
- `--wipe-worktree`;
- `--keep-worktree`;
- warning text about imperfect plaintext cleanup;
- lock, forked-process, key-agent, and materialized-plaintext tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Sagnir clearly separates encrypted storage from plaintext worktree state and
  does not claim to recall plaintext copied elsewhere.

### v0.105.0 - Vault Status And Leak Scanner

Goal: make encrypted realm state and plaintext or metadata leak surfaces
visible.

Deliverables:

- `saga vault status`;
- `saga vault scan-leaks`;
- sealed-private mode and migration-state reporting;
- ignored-directory, editor cache, build output, old pack, log, and object-ID
  disclosure checks;
- leak warning fixture tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Users get honest warnings about plaintext and metadata risks while unlocked
  and after migration from an open realm.

### v0.106.0 - Rekey And Private-Locator Epoch Migration

Goal: rotate ciphertext keys and private lookup-locator keys without rewriting
or impersonating immutable signed semantic history.

Deliverables:

- TLA+/PlusCal or equivalent key-rotation model completed before mutation code;
- crypto epoch transition;
- `saga vault rekey`;
- recipient and compartment rewrap plan;
- encryption-key rotation independent from private-locator key rotation;
- private-index commitment-key rotation independent from encryption-key and
  private-locator rotation;
- compartment-root, realm-manifest, and neutral-domain key rotation independent
  from unrelated compartment and endpoint-placement projections;
- scheduled and compromise-triggered private-locator key rotation;
- scheduled and compromise-triggered private-index commitment-key rotation with
  exact old/new commitment epochs and no implicit write authority for either
  key holder;
- preservation of the original canonical object bodies, semantic commitments,
  signed graph roots, references, transitions, proofs, and signatures;
- whole-index private-locator and ciphertext-placement rewriting without
  rewriting canonical references;
- old and new private-locator epoch admission rules;
- signed migration transition binding the unchanged semantic roots, old locator
  index root, new locator index root, and resulting ciphertext storage root;
- index-commitment rotation transition binding the unchanged canonical locator
  entry set and semantic-state root, old and new logical roots, old and new
  signed checkpointed manifests, structure version, policy root, membership
  epoch, and governing authorization;
- compartment-root rotation atomically updates only the affected opaque
  compartment reference in the realm manifest and preserves valid
  partial-access proofs for every unaffected compartment;
- neutral-domain rotation binds old/new neutral commitment, locator,
  index-commitment, encryption, wrapping, recipient, recovery, and root epochs,
  plus every compartment edge that may continue referencing the old neutral
  identity;
- authorized historical verification retains or retrieves the old
  index-commitment key only through governed encrypted custody; new peers may
  verify the admitted migration and current root without receiving authority to
  publish either root;
- authenticated quota carry-forward binding the old and new locator epochs,
  per-replica counters, replica-incarnation lineage, aggregate actor/device
  counters, governance quota identity, escrow-right allocation root, spent-right
  commitments, unresolved quota-conflict roots, policy root, and any explicitly
  authorized quota adjustment;
- quota carry-forward is verified before the new locator epoch accepts writes;
  retiring the old locator key, rotating a device key, or creating a new replica
  incarnation cannot reset prior consumption;
- unspent escrow rights move to the new locator epoch only through one signed
  migration allocation that invalidates use under the old epoch; mixed-epoch
  peers cannot spend the same right in both indexes;
- crash-safe dual-commit migration for old and new locator projections;
- encrypted old-to-new locator mapping with no public correlation;
- old signatures remain verifiable against the original semantic commitments;
- old private-locator keys are not required for historical signature
  verification and may exist only in governed, isolated migration or recovery
  custody until admitted old locator indexes and packs are retired;
- historical verification mode that does not require distributing a
  compromised old membership-testing key to new peers;
- scoped authenticated translation evidence allowing new peers to verify that
  old and new projections resolve to the same semantic commitments;
- mixed-epoch sync negotiation and bounded transition support while peers
  migrate;
- retirement of the compromised private-locator key and old externally visible
  packs after admitted peer and retention conditions;
- explicit statement that membership relationships observed before rotation
  cannot be hidden retroactively;
- old-key retention and cryptographic-erasure policy;
- crash-safe staged key rotation;
- rollback and interrupted-rotation recovery;
- invalid epoch, partial compartment, stale recipient, restored snapshot,
  interrupted locator rewrite, semantic-reference mutation, translation
  substitution, public mapping leakage, mixed peer epoch, compromised locator-
  or index-commitment-key reuse, unauthorized logical-root publication,
  historical logical-root verification, old-signature verification, new-peer
  verification, partial-access proof continuity, unrelated-compartment
  non-disclosure, neutral-domain rotation/recovery, stale neutral edge,
  stale-pack, missing quota carry-forward, counter rollback, new-incarnation
  quota evasion, actor/device aggregation, escrow-right double-spend across
  epochs, unresolved conflict carry-forward, and locator-epoch reset tests.

Verification:

- bounded key-rotation model check;
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Key rotation is a signed transition model, not an in-place mutation.
- Ordinary ciphertext rekeying does not force semantic identity or private
  locator changes.
- Private-locator compromise has a complete migration protocol that replaces
  lookup and storage projections without changing any signed semantic object or
  reference.
- Private-index commitment-key rotation produces new signed logical-root
  manifests over the same canonical entry set; it neither rewrites semantic
  history nor grants root-admission authority through key possession.
- Compartment and neutral-domain rotation update authenticated realm
  composition without disclosing or rewriting unrelated compartments.
- The migration transition is new signed history; rewritten locator projections
  are never represented as the originally signed graph.
- Old signatures remain verifiable from retained canonical commitments and
  scoped authenticated translation evidence even after the compromised locator
  key is withheld from new peers.
- Peers on different private-locator epochs either use the admitted migration
  protocol or refuse trust; they do not guess identity equivalence.
- A new private-locator epoch cannot accept duplicate creation until
  authenticated per-replica and actor/device aggregate quota state is carried
  forward from the admitted prior epoch.
- Rotation protects future membership privacy but cannot erase correlations
  already observed under the compromised epoch.

### v0.107.0 - Hybrid Post-Quantum Readiness Scaffold

Goal: prepare recipient wrapping and signatures for reviewed hybrid algorithms.

Deliverables:

- hybrid key-wrap metadata;
- post-quantum algorithm registry placeholders;
- algorithm admission document;
- crypto provider review checklist;
- downgrade and component-stripping tests;
- tests that unknown or unadmitted algorithms fail closed.

Verification:

- `cargo test -p sagnir-crypto`
- `scripts/checks.sh`

Exit criteria:

- Sagnir is ready to admit hybrid classical plus post-quantum providers without
  changing object formats.

### v0.107.1 - Post-Quantum And Hybrid Suite Provider Admission

Goal: admit actual post-quantum and hybrid suites/providers under the
provider-independent v0.21.1 suite identity and transcript rules.

Deliverables:

- suite IDs inherit v0.21.1 and identify only algorithm family, exact parameter
  set, standard/revision, encoding, prehash/pure mode where applicable, and
  hybrid components/combiner;
- provider assurance follows v0.21.2 and separately binds implementation/build/
  backend/platform, issuer/trust root and assurance category, standards
  validation, side-channel profile, hardware/software path, freshness,
  revocation, and vectors; realm admission separately binds registry/policy/
  crypto epoch, errata state, lifecycle, and downgrade rules;
- ML-DSA identifiers distinguish ML-DSA-44, ML-DSA-65, and ML-DSA-87 and pin
  the admitted FIPS 204 revision plus reviewed errata/pending-update policy;
- ML-KEM identifiers distinguish ML-KEM-512, ML-KEM-768, and ML-KEM-1024 and
  pin the admitted FIPS 203 revision plus reviewed errata/pending-update policy;
- signature or ciphertext length is validated after suite selection and can
  never select the suite or parameter set by itself;
- hybrid signature/wrapping suite IDs enumerate both component suites, order,
  combiner, transcript/context binding, downgrade rule, and required all-
  components verification semantics;
- hybrid canonical transcript binds suite ID, each component algorithm and
  parameter set, component count/order/length, message/context digest, realm,
  action, policy/crypto epoch, and result; merely splitting concatenated bytes
  is not verification;
- unknown revisions, unreviewed errata states, generic `PQ`/`MlDsa` ambiguity,
  component stripping/reordering/substitution, length-selected algorithms,
  mixed-epoch components, and classical-only downgrade fail closed;
- algorithm registry can retire one parameter/revision suite or one provider
  assurance/admission combination without making historical statements
  unverifiable, changing suite identity, or silently admitting a sibling
  parameter set;
- NIST standards known-answer and malformed vectors plus provider differential
  vectors for every admitted exact suite;
- operational promotion remains disabled for placeholder-only suites until the
  exact provider, vectors, lifecycle, and policy admission are complete.

Verification:

- `cargo test -p sagnir-crypto`
- exact-suite registry and transcript vector validator;
- NIST known-answer/malformed vector suite;
- hybrid component-binding differential tests.

Exit criteria:

- A parser, signature verifier, recipient wrapper, policy, and audit report all
  agree on one exact v0.21.1 suite without inferring it from byte length or
  embedding provider/admission state in its protocol identity.
- Hybrid acceptance proves the admitted binding of every required component and
  cannot degrade to one surviving component.
- Standards errata and revisions are explicit admission state, not undocumented
  dependency behavior.
- Two conforming admitted providers remain interoperable for the same suite,
  and provider migration does not alter historical transcript meaning.

### v0.108.0 - Selective Disclosure Proofs

Goal: disclose only policy or evidence claims required by a recipient.

Deliverables:

- Merkle multiproof disclosure format;
- signed disclosed claims;
- hidden-witness commitment model;
- scope and audience binding;
- leakage analysis for leaf positions, tree shape, cross-proof correlation,
  freshness, revocation, and repeated disclosure;
- privacy-preserving tree shaping or explicit leakage acknowledgement;
- replay and claim-substitution tests;
- documented admission rule that zero-knowledge proofs are added only for
  predicates that genuinely require hidden witnesses.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-crypto`

Exit criteria:

- A peer can verify selected evidence or policy inputs without receiving the
  unrelated encrypted ledger.
- The disclosure report states what structural metadata the proof reveals and
  whether it remains fresh after revocation or policy-epoch changes.

## Phase 10: Bundles And Sync

### v0.109.0 - Pack File Format

Goal: store multiple immutable objects in a bounded pack.

Deliverables:

- pack header;
- object table;
- object body offsets;
- pack footer;
- pack manifest hash;
- total compressed and expanded byte limits;
- per-object size and reference-count limits;
- one inherited v0.12.1 decode budget covers total records, objects, references,
  offsets, allocation, checksum/hash work, decompression expansion, and delta
  materialization across the whole pack, not only each individually valid
  object;
- compression and delta format admission;
- maximum decompression ratio and delta-chain depth;
- compartment-local delta base rule;
- authenticated random-access page integration for encrypted packs;
- immutable pack generation ID and authenticated predecessor/successor lineage;
- deletion-capability metadata distinguishing independently deletable encrypted
  records from packs that require whole-pack replacement;
- no in-place mutation of a committed pack to remove one record;
- replacement-pack privacy profile with bucketed size, optional padding, opaque
  generation identifiers, and no public survivor-to-old-position map;
- malformed pack, false record-deletion capability, generation substitution,
  lineage fork, and replacement-map leakage tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`

Exit criteria:

- Pack readers verify bounds before trusting offsets or object counts.
- Pack readers reject decompression bombs, deep delta chains, invalid bases, and
  offset arithmetic overflow before expensive materialization.
- Pack formats declare whether one ciphertext record can be deleted
  independently; otherwise later redaction must replace the complete pack.

### v0.110.0 - Non-Durable Bundle Manifest Draft

Goal: draft and test bundle inventory requirements without freezing durable
bytes before v0.110.1 defines blind claims, privacy, and admission order.

Deliverables:

- bundle manifest;
- object refs;
- world refs;
- fact refs;
- policy refs;
- encrypted bundle marker;
- visible versus encrypted metadata policy;
- opaque padded outer manifest contains only framing, protocol/suite versions,
  randomized bundle/storage identity, coarse padded length/count classes,
  ciphertext chunk commitments, and declared transport/preflight limits;
- encrypted authenticated canonical inner manifest contains semantic inventory,
  object/fact/policy roots, world heads, causal frontier, policy/evaluator/crypto
  epochs, signatures, closure evidence, and exact resource requirements;
- resource estimate metadata;
- compressed and expanded byte estimates;
- ancestry depth, reference fanout, concurrent-head, and proof-complexity
  estimates;
- minimum verification mode metadata;
- recommended verification profile metadata;
- manifest validation tests;
- disposable reference schema and prototype bytes with unmistakable experimental
  magic/version;
- prototype cannot be written as durable realm state, accepted by production
  decoders, signed as authority, transferred as a compatibility format, or used
  to materialize a worktree;
- every field remains subject to v0.110.1 visibility, claim-taxonomy, padding,
  cumulative-budget, and two-phase-admission review before format freeze.

Verification:

- `cargo test -p sagnir-sync`

Exit criteria:

- Sagnir can evaluate what a future bundle must describe and estimate without
  creating a durable or interoperable manifest promise.
- No bundle manifest format freezes before the v0.110.1 contract decides which
  claims and fields are visible, encrypted, padded, and authoritative.

### v0.110.1 - Blind-Remote Claims And Bundle Admission Contract

Goal: freeze honest proof names, blind-storage visibility, final outer/inner
manifest bytes, and two-phase bundle admission before verification or live sync
can imply semantic trust.

Deliverables:

- normative claim taxonomy separates outer framing validity, ciphertext chunk
  integrity, Merkle inclusion, storage-ID binding, upload capability/quota,
  replication receipt, and availability evidence from decrypted semantic object,
  graph, signature, policy, and world-transition verification;
- blind `sagad` can issue only claims derivable from opaque ciphertext/framing
  and never labels them object-integrity, graph, policy, promotion, or semantic
  proofs;
- 1.0 semantic acceptance occurs only on an authorized client after decryption
  and complete typed ingest, or through an explicitly trusted witness whose
  statement and trust assumptions are shown; TEE and zero-knowledge alternatives
  remain separate future trust models unless explicitly admitted;
- the v0.110.0 draft is replaced by final opaque outer and encrypted inner
  manifest schemas with exact canonical bytes, production magic/version,
  visibility tables, padding buckets, audience/purpose/epoch binding, replay
  context, migration policy, and malformed vectors;
- blind-visible fields exclude realm ID where policy requires opacity, paths,
  world names, actors, recipients, signatures, semantic roots/commitments,
  private locators, graph edges, policies, facts, and recipient-slot topology;
- two-phase admission state machine: bounded outer preflight, isolated
  quarantine write, recipient authorization/decryption, canonical inner decode,
  body-derived graph verification, signature/revocation/time verification,
  compound policy/obligation admission, WAL transaction, and final trusted
  publication;
- remote transparency candidates inherit v0.99.1's exact
  `UntrustedCandidate -> BoundedQuarantinedCandidate ->
  AdmittedAuthorityEvidence` boundary; no bundle stage can skip a state, convert
  a validly framed/signed candidate into receiver-retained authority, or apply
  the no-drop rule before final authority admission;
- outer preflight reserves aggregate bundle/candidate item, byte, signature,
  page/node, expansion, and verification-work ceilings before integrity-bound
  quarantine capture; unavailable quarantine capacity returns `ResourceLimit`
  without a durable candidate or semantic rejection claim;
- fetching, storing, checksum verification, receipt issuance, or blind-server
  acceptance never grants worktree materialization or trusted-reference
  resolution;
- split-trust summaries disclose only explicitly selected claims and bind
  audience, purpose, realm/opaque scope, policy/crypto epoch, freshness,
  revocation context, nonce, and replay window;
- one cumulative bundle decode/work budget covers outer and inner bytes,
  ciphertext chunks, objects, facts, parents, heads, references, proofs,
  decompression, delta expansion, signatures, causal expansion, allocations,
  unresolved promises, and verification work;
- quota privacy contract: a remote cannot claim both per-identity enforcement
  and identity unlinkability unless clients spend pre-issued unlinkable,
  epoch-scoped quota rights/nullifiers or another admitted construction;
  ordinary stable quota accounts are documented correlation handles;
- malformed outer/inner manifest, padding downgrade, semantic-field exposure,
  false blind proof name, receipt-as-trust confusion, decrypt failure,
  quarantine bypass, partial admission, replay, and quota-correlation tests;
- disposable decoder/reference prototype and early benchmark/fuzz gate inherit
  the v0.92.1 prototype-isolation rule before durable bundle bytes freeze.

Verification:

- independent outer/inner manifest vector validator;
- claim-taxonomy API and CLI golden-output tests;
- blind-store visibility and traffic-shape fixtures;
- two-phase admission state-machine model;
- bundle format fuzz smoke and cumulative-budget tests.

Exit criteria:

- Every bundle or blind-server proof name states exactly what ciphertext-visible
  evidence establishes and cannot be confused with semantic acceptance.
- Bundle manifest compatibility begins only with this release's admitted final
  schema; no v0.110.0 experimental byte is accepted or migrated implicitly.
- No remote byte influences trusted roots, policy, aliases, or materialization
  before the full local typed-ingest and WAL admission pipeline succeeds.
- A remote signature or claimed identity cannot force receiver retention;
  over-capacity input is refused before durable quarantine and remains sender-
  held without creating semantic rejection evidence.
- Privacy documentation does not promise unlinkable per-identity quotas while
  exposing a stable account or selector.

### v0.110.2 - Bundle Privacy, Handoff, And Resume Authority Contract

Goal: define privacy-preserving signatures, retained-byte identity, and durable
single-consumption resume semantics before bundle creation, quarantine, or
import implements them.

Deliverables:

- identity-bearing bundle signatures, signer/key IDs, realm IDs, recipient
  identities/topology, semantic roots, and stable correlation handles exist
  only inside the encrypted authenticated inner manifest for privacy-preserving
  profiles;
- opaque outer hashes and Merkle commitments establish byte/chunk integrity
  only relative to the claimed root and do not establish sender authenticity;
- a recipient-keyed outer MAC establishes authenticity only to holders of that
  MAC key and is not public source identity; an outer signature can establish
  source authenticity but may expose/link a stable signer or key;
- blind storage and CLI/API output report exactly `claimed-root integrity`,
  `recipient-key-holder authenticity`, or `signature source authenticity` as
  applicable and never collapse them into generic `authenticated`;
- any profile exposing an outer signer, key, realm, recipient, stable MAC
  selector, or stable signature is explicitly named privacy-leaking and reports
  that correlation before bundle creation;
- typed non-cloneable in-memory `PreflightedBundle` capability owns or borrows
  the exact retained file/object handle, observed file identity/generation,
  byte length, bundle digest, outer-manifest digest, preflight transcript,
  admitted limits, and intended quarantine transaction;
- preflight-to-quarantine handoff consumes `PreflightedBundle` without reopening
  a display path, but the retained handle is only a source capability and is
  never treated as proof that bytes stayed immutable after preflight;
- v0.110.3 requires copy-and-rehash into a new integrity-bound quarantine object
  or a verified OS-enforced sealed object before any quarantine byte identity is
  admitted; direct transfer of an ordinary retained source handle is forbidden,
  and v0.110.4 governs every later trusted read;
- path validation, close, path reopen, or name-based equality cannot preserve a
  preflight capability across substitution, rename, hard-link, mount, namespace,
  or concurrent replacement races;
- persisted resume state is not described as non-cloneable; store authority
  enforces single consumption within one non-rolled-back store history with
  durable operation ID, checkpoint generation, expected-state CAS, writer lease,
  consumed/superseded marker, and WAL-backed publication;
- resume checkpoint binds immutable original ceilings, deterministic cost-table
  version, authenticated consumed counters, exact remaining allowance, and
  durable operation ID rather than an ephemeral in-memory budget parent;
- the cost table is protocol-fixed or selected by signed canonical realm state
  and evaluator compatibility under v0.12.4, never by local configuration,
  bundle bytes, or resume bytes;
- resumption cannot reset/recalculate ceilings from current configuration,
  return consumed work, duplicate allowance through checkpoint copy, or switch
  to a cheaper cost-table version;
- filesystem snapshot rollback or full store cloning can duplicate persisted
  checkpoint bytes and local leases; detection/containment depends on signed
  checkpoint/frontier continuity, replica incarnation, external witness/anchor
  where policy requires it, and final transaction idempotency, and no stronger
  anti-rollback claim is made without such evidence;
- identical operation inputs and bound contexts deterministically produce the
  same result, while any context change invalidates the resume checkpoint;
- disconnected full-store clones can each acquire their local writer lease and
  publish locally authoritative forks; conflicting results under one operation
  ID cannot later reconcile as one valid history without detectable conflict/
  equivocation when branches, checkpoints, or witnesses are compared;
- external witnesses or non-rollback anchors may prevent or promptly expose
  some forks, but a permanently isolated clone can remain undetected without
  comparison or an admitted external anchor;
- outer-identity leakage, path reopen/substitution, retained-handle mismatch,
  copy/rehash mutation, checkpoint clone, CAS race, snapshot rollback, store
  clone, budget reset, counter rollback, cost-version change, and duplicate-
  completion tests/model.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-store`
- bundle outer/inner privacy vectors;
- retained-handle handoff race fixtures;
- resume single-consumption crash/clone state-machine model.

Exit criteria:

- A privacy-preserving outer envelope exposes no stable signer, key, realm, or
  recipient identity.
- Quarantine receives the exact bytes that passed preflight without trusting a
  path reopen or ordinary retained-handle immutability.
- Persisted resume state is durably single-consumption within one observed
  non-rolled-back history and cannot mint or reset verification budget.
- Offline full clones may fork locally; Sagnir promises deterministic same-
  context results and detectable conflict on reconciliation, not impossible
  publication before disconnected histories are compared.

### v0.110.3 - Quarantine Integrity Capture Contract

Goal: capture mutable preflight source bytes into one durable integrity-bound
quarantine object without claiming portable physical immutability.

Deliverables:

- default capture consumes `PreflightedBundle`, reads only through its retained
  source handle, and streams into a newly created private no-replace temporary
  object under retained quarantine-directory authority;
- capture enforces original byte/work ceilings while computing the admitted
  bundle and outer-manifest digests over the exact destination bytes; destination
  length and digests must equal the preflight transcript or capture fails;
- the destination is synchronized, published under a generation-bound content
  identity with no-replace semantics, and its directory is synchronized before
  returning `IntegrityBoundBundle`;
- `IntegrityBoundBundle` binds destination handle/object identity, generation,
  exact length/digests, capture transcript, source preflight digest, and
  quarantine transaction; later stages never read the original source;
- copy, digest equality, content addressing, no-replace publication, retained
  destination handle, permissions, ACLs, or before/after metadata do not prove
  that another same-UID descriptor, mmap, privileged process, or direct local
  writer cannot later mutate the destination;
- the ordinary copied destination is labelled `integrity-bound`, not physically
  immutable; every consuming read must pass v0.110.4 and cannot verify, rewind,
  then parse mutable file bytes;
- an alternative zero-copy path is admitted only where the OS provides an
  enforceable seal/immutable-object mechanism, Sagnir verifies the seal after
  application, no writable alias can survive, and mutation attempts fail;
  unsupported platforms use copy-and-rehash rather than weakening the rule;
- before/after source metadata, file identity, size, timestamps, and generation
  checks are diagnostic race evidence only and never replace hashing the captured
  bytes or OS-enforced sealing;
- source mutation, truncation, sparse-hole change, overwrite through another
  descriptor, mmap write, hard-link write, rename, mount/namespace replacement,
  short read/write, cancellation, crash, digest mismatch, destination
  substitution, seal failure, and publication failure leave no admitted
  `IntegrityBoundBundle`;
- crash-safe cleanup journal removes unpublished temporary capture objects and
  recovery cannot mistake a partial copy for an integrity-bound quarantine
  object.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`
- concurrent source-mutation and retained-handle race suite;
- platform seal/immutability capability fixtures;
- capture crash/publication fault-injection suite.

Exit criteria:

- A retained source handle alone never establishes byte immutability.
- Every admitted quarantine object is either a fully copied, rehashed,
  synchronized no-replace integrity-bound object or a verified OS-sealed
  immutable object, with the distinction preserved in type/status/reporting.
- Capture alone does not authorize decryption or parsing; consumers require the
  v0.110.4 trusted-read boundary.

### v0.110.4 - Quarantine Trusted-Read Boundary

Goal: guarantee that bytes parsed, decrypted, or verified are the exact bytes
whose commitment/authentication was checked, even when captured storage remains
mutable to the same UID or a privileged local writer.

Deliverables:

- every quarantine consumer uses one admitted boundary: verified OS-enforced
  sealing; a separate least-privilege storage service/account that denies writers
  after capture; one-read immutable owned-memory validation/consumption; or
  authenticated immutable pack pages whose consumed bytes are the validated
  page bytes;
- portable default reads one bounded committed ciphertext chunk/range exactly
  once through retained `IntegrityBoundBundle` authority into immutable owned
  bytes such as `Arc<[u8]>`, validates length/index/digest/Merkle commitment and
  any available authentication over that allocation, and parses/decrypts that
  exact allocation without rewind or a second file read;
- decrypted plaintext crosses v0.23.2/v0.92.2 only after authentication and is
  parsed from the exact owned authenticated plaintext buffer; raw mutable file/
  mmap bytes never become parser input after a separate validation pass;
- owned chunk/range lifetime, total retained bytes, concurrent pages, parser
  references, and cache reuse are bounded by v0.12.1/v0.12.4 budgets; eviction
  invalidates borrowed views before the backing allocation is released;
- service-process mode authenticates requests/responses, binds destination
  object/generation/range/digest, drops ambient repository authority, denies
  same-UID writer access where the platform model permits, and reports residual
  administrator/kernel threats;
- OS-sealed mode verifies seal/immutability state immediately before admission
  and on reopen; any unsupported seal, surviving writable alias, seal removal,
  or platform ambiguity falls back to owned-memory reads or refuses;
- authenticated pack-page mode binds pack/page identity, generation, offsets,
  lengths, page root/tag, suite/key epoch, and immutable backing lifetime before
  exposing a view;
- mutation of unread storage is detected when that range is read and prevents
  further admission; already validated owned bytes remain stable, but no partial
  prefix becomes authoritative before final bundle/chunk completeness and typed
  ingest;
- resume checkpoints bind the exact next trusted-read range/chunk and committed
  destination generation; resumption rereads/validates that next range and never
  serializes a borrowed pointer, mmap view, or mutable file cursor as trust;
- digest-then-rewind mutation, mutation between validation and parse, same-UID
  descriptor/mmap overwrite, service compromise/disconnect, seal ambiguity/
  removal, pack-page substitution/truncation, owned-buffer eviction/reuse,
  resume-range substitution, and concurrent-reader/writer tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`
- same-UID mutation and verify/consume TOCTOU suite;
- sealed/service/owned-memory/pack-page boundary fixtures;
- trusted-read resume and cache-lifetime schedule tests.

Exit criteria:

- Sagnir never validates mutable file bytes and later reparses them after a
  rewind or second read.
- Every parser/decryptor consumes the exact sealed, service-returned,
  authenticated-page, or immutable owned bytes that passed validation.
- Unsealed copied files remain honestly labelled integrity-bound, with ongoing
  trust established per consumed range rather than by a physical-immutability
  claim.

### v0.111.0 - Bundle Creation And Outer Preflight

Goal: create admitted bundles and perform bounded outer/ciphertext preflight
without decrypting untrusted content before quarantine exists.

Deliverables:

- `saga bundle create`;
- `saga bundle verify`;
- `saga bundle create --encrypted`;
- recipient-targeted metadata and identity-bearing context-bound bundle
  signatures inside the encrypted authenticated inner manifest;
- outer signature/identity exposure only through an explicitly selected
  privacy-leaking profile governed by v0.110.2;
- local creation-time missing object detection over already trusted source
  state;
- deduplication before expensive proof or signature work;
- outer framing, size/quota, ciphertext chunk/Merkle, storage-ID, signature-
  framing where the selected profile permits it, and verification-budget
  preflight only for received bundles;
- `saga bundle verify` labels this result `outer/ciphertext preflight` and does
  not claim semantic object, graph, signature, policy, or world validity;
- output distinguishes claimed-root hash/Merkle integrity, recipient-key-holder
  MAC authenticity, and signature source authenticity under v0.110.2, including
  the selected profile's identity/linkability disclosure;
- received bundles cannot be decrypted, semantically decoded, assigned a
  resumable semantic checkpoint, or written outside temporary bounded preflight
  storage before v0.112.0 quarantine exists;
- preflight cancellation discards temporary state and creates no trusted or
  resumable semantic capability;
- successful preflight returns the consuming v0.110.2 `PreflightedBundle`
  capability retaining exact byte-handle identity and digests for quarantine;
- successful preflight also binds one exact bounded quarantine reservation for
  declared/locally capped candidate items, captured bytes, signatures, prepared
  pages/nodes, and subsequent verification work; reservation failure returns
  `ResourceLimit` before quarantine handoff and leaves only disposable temporary
  preflight bytes for deterministic cleanup; the reservation is a provisional
  `QuarantineReservationLease` governed by v0.111.1 and is not a durable
  candidate, authority claim, or renewable session allowance;
- manifest estimates remain untrusted upper-bound hints: understated counts,
  bytes, expansion, signatures, pages, or work fail streaming enforcement and
  cannot extend the original reservation, partially admit a candidate, or mint a
  resumable semantic capability;
- malicious bundle and fork-bomb tests;
- verification-budget preflight tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-cli`

Exit criteria:

- A trusted local realm can create an admitted bundle, and a received bundle can
  receive bounded outer/ciphertext preflight before import.
- Pre-decryption verification reports only v0.110.1 outer/ciphertext claims;
  semantic verification requires successful authorized decryption and complete
  inner typed-ingest verification beginning only after v0.112.0 quarantine.
- Bundle verification reports when local budgets cannot satisfy the bundle's
  minimum verification mode.
- Manifest estimates are treated as untrusted preflight hints; streaming local
  hard limits remain authoritative throughout verification.
- No received bundle creates decrypted or resumable semantic state in this
  release.
- Outer preflight cannot hand off to quarantine without one exact aggregate
  capacity reservation, and understated manifests cannot expand it later.
- Preflight success cannot survive close-and-reopen or path substitution as a
  capability for different bytes.

### v0.111.1 - Quarantine Reservation Lease Lifecycle

Goal: prevent slow, reconnecting, or parallel senders from holding receiver
quarantine capacity indefinitely before a durable candidate exists.

Deliverables:

- versioned deterministic local `QuarantineReservationLease` record binds a
  random reservation ID, exact item/encoded-byte/captured-byte/signature/page/
  node/work counters, store, opaque realm scope where available, peer/endpoint,
  authenticated actor where known, transport session, bundle, transfer,
  destination generation, and originating preflight transcript;
- each lease records creation generation, monotonic progress counter, monotonic
  idle deadline, absolute maximum lifetime, and the original cumulative
  lifetime consumed; wall-clock time, canonical event time, and signed policy
  time cannot extend or decide this live local resource-control lease;
- versioned local resource policy sets non-bypassable maximum concurrent leases
  and aggregate reserved items/bytes/work per peer/endpoint, opaque realm scope,
  authenticated actor where known, and store; identity, session, bundle, or
  transfer fanout never multiplies the store or realm ceiling;
- reconnect, transport replacement, resume-token rotation, bundle splitting,
  peer reauthentication, or session replacement inherits the original creation
  generation, absolute deadline, cumulative lifetime, progress counter, and
  remaining counters; none creates a fresh lease for the same transfer bytes;
- progress can renew only the idle deadline, only after the corresponding bytes
  have been durably captured and the corresponding deterministic work has been
  consumed, and never beyond the original absolute lifetime;
- item, byte, allocation, signature-verification, hashing, decompression, KDF,
  proof-traversal, decode, and page/node work is debited with checked arithmetic
  before that work or allocation begins; exhaustion refuses before execution
  and unused reserved capacity is released only by an atomic state transition;
- expiry or cancellation before complete quarantine publication atomically
  invalidates the handoff capability, removes unpublished partial bytes and
  resume state, releases every reserved counter, and creates no candidate,
  verification result, policy result, or authority evidence;
- successful complete quarantine publication atomically converts the lease into
  the durable `BoundedQuarantinedCandidate` quota charge; after conversion the
  charge follows v0.112.0 expiry/retention and cannot independently expire as a
  pre-capture lease;
- startup recovery reconciles every durable reservation record, cleanup-journal
  entry, partial destination, resume checkpoint, and candidate publication as
  exactly one complete candidate plus matching charge or one incomplete cleanup
  plus fully released charge; an ambiguous or orphan reservation cannot remain
  active indefinitely or be guessed complete from partial bytes;
- scheduling reserves bounded per-peer/endpoint shares or uses an equivalent
  documented fair allocator so one endpoint cannot consume the entire store
  ceiling with valid but stalled transfers; unused shares may be borrowed only
  without defeating hard aggregate limits or starvation bounds;
- lease identifiers, deadlines, counters, progress, and cleanup outcomes are
  local non-authoritative resource metadata; they do not enter canonical realm
  validity, remote policy, signed event time, or shared rejection evidence;
- slowloris, zero-progress, one-byte-progress, reconnect deadline-reset, resume-
  token rotation, bundle-split, parallel-reservation, per-peer starvation,
  work-before-debit, counter-overflow, crash-at-expiry, monotonic-clock failure,
  wall-clock rollback, completion/expiry race, and startup-orphan tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`
- quarantine lease state-machine and deterministic-time tests;
- process-kill capture/publication/recovery integration suite.

Exit criteria:

- No sender can retain receiver capacity indefinitely through inactivity,
  trickle progress, reconnect, resume, session replacement, or transfer split.
- Every expensive operation is charged before execution, and every restart
  resolves each reservation to one durable charged candidate or complete
  cleanup with released capacity.
- Lease expiry and scheduling affect only local resource availability; they
  cannot create, delete, select, or reinterpret authority evidence.

### v0.111.2 - Quarantine Lease Clock And Metadata Closure

Goal: make persisted lease expiry safe across restart and protect temporary
resource metadata from becoming a sealed-private activity ledger.

Deliverables:

- random process-scoped `ClockEpoch` generated from the v0.23.0 OS-CSPRNG at
  daemon/process start and bound to every lease creation, progress, deadline,
  conversion, expiry, and cleanup record; entropy failure refuses new leases;
- monotonic values are comparable only within one exact `ClockEpoch`; a process
  replacement always creates a new epoch even on the same boot, and an OS boot
  identifier is diagnostic context only and cannot substitute for the random
  epoch or authorize deadline comparison;
- an admitted platform clock table names the exact API used on Linux, Windows,
  each supported BSD, macOS, Android, iOS, and later Aesynx, its resolution,
  wrap behavior, whether it advances during suspend, and the tested fallback or
  refusal behavior; no generic `Instant` or "monotonic" claim hides platform
  differences;
- within one epoch, clock read failure, backward movement, wrap, impossible
  discontinuity, or source change conservatively expires/refuses affected
  incomplete leases and cannot extend an idle or absolute deadline;
- after process restart or `ClockEpoch` mismatch, a completely committed
  `BoundedQuarantinedCandidate` and its durable quota charge remain intact,
  while every incomplete pre-candidate lease, partial destination, resume state,
  and reservation charge is atomically cleaned and released before retry;
- 1.0 does not resume a pre-candidate transfer across process epochs; a peer may
  start a new bounded attempt only after old cleanup. Any future cross-restart
  lease resume requires a separately admitted trusted-time protocol and cannot
  reconstruct downtime from unauthenticated wall-clock time;
- candidate publication state is determined by the atomic publication/charge
  transaction, never by a deadline comparison, file presence, progress counter,
  or best-effort cleanup record;
- known encrypted-realm lease records, progress indexes, and cleanup metadata
  use the admitted encrypted metadata/WAL boundary and key epoch; they never
  downgrade to cleartext when realm metadata keys are locked or unavailable;
- before realm identification or authorized decryption, a daemon-local metadata
  protection key encrypts identifiable lease state and indexes while records use
  opaque scoped handles; protected profiles refuse new reservations if that key
  is unavailable, while any profile permitting cleartext operational metadata
  must declare the exact leakage before transfer;
- encrypted lease associated data binds record kind/version, store, daemon
  incarnation, `ClockEpoch`, destination generation, reservation ID, and key
  epoch so records cannot be replayed across stores, processes, generations, or
  reservations;
- actor, realm, peer, endpoint, bundle, and transfer identifiers never appear
  directly in filenames, directory topology, lock names, temporary paths, logs,
  metrics, traces, process titles, or public abuse receipts; filesystem names use
  unlinkable opaque handles with collision-safe creation;
- v0.95.0 privacy profiles define whether lease counters, timing, progress, and
  cleanup are exact, bucketed, padded, delayed, or suppressed; logs, metrics,
  and abuse receipts expose only opaque handles and coarse profile-approved
  values, while detailed identity/activity inspection requires authenticated
  privileged local access and is audited;
- expiry/cancellation cleanup removes lease records, indexes, partial bytes,
  derived telemetry, and temporary key references according to one declared
  retention policy; retained aggregate abuse statistics cannot reconstruct the
  removed identity or transfer timeline;
- blind and split-trust deployment documentation names network endpoint,
  connection timing, traffic volume, and host-observer leakage that encryption,
  opaque handles, bucketing, padding, or cleanup cannot retroactively hide;
- reboot, suspend/resume per admitted platform, process replacement, boot-ID
  reuse/substitution, clock failure/backward movement/wrap, epoch substitution,
  restart with complete versus partial publication, metadata-key unavailable,
  cross-store ciphertext replay, filename/topology leakage, log/metric leakage,
  counter/timing bucketing, and cleanup-retention tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`
- per-platform clock adapter and suspend/restart fixtures;
- encrypted operational-metadata and malicious local-observer leakage suite;
- process-kill complete/partial publication recovery matrix.

Exit criteria:

- A monotonic deadline is never interpreted outside its originating process
  epoch, and restart cannot extend or ambiguously preserve an incomplete lease.
- Complete candidate publication survives restart, while partial work is cleaned
  without consulting unauthenticated wall-clock time.
- Protected or blind profiles do not expose collaboration identities or precise
  lease activity through durable metadata, filenames, logs, metrics, or cleanup
  artifacts beyond their explicit measurable leakage contract.

### v0.112.0 - Quarantine Namespace And Trust Isolation

Goal: ensure untrusted remote data cannot influence trusted state before full
re-admission.

Deliverables:

- physically or logically separate quarantine namespace;
- quarantine capture consumes the v0.110.2 `PreflightedBundle` from v0.111.0
  through the mandatory v0.110.3 copy-and-rehash or verified OS-seal contract;
- quarantine storage accepts only the resulting `IntegrityBoundBundle` with
  exact destination identity/generation/digests and physical seal status, and
  creates no semantic trust capability;
- every decrypt/decode/verify consumer uses a v0.110.4 sealed, isolated-service,
  immutable-owned-memory, or authenticated-pack-page trusted-read boundary;
- separate encrypted staging namespace for later decrypted/decoded intermediate
  state, inaccessible to trusted object/fact/index/alias resolution;
- identifiers that cannot shadow trusted objects;
- quarantine objects excluded from trusted reference resolution;
- quarantine facts excluded from policy obligations and authoritative indexes;
- no worktree materialization without complete re-admission from original
  bytes;
- storage, age, object-count, byte, fanout, and ancestry quotas;
- hard candidate item/signature/prepared-page/verification-work quotas inherit
  v0.99.1 and aggregate across all actors, devices, replicas, peers, bundles,
  connections, and concurrent sessions under store/realm ceilings; identity or
  bundle fanout cannot multiply quarantine capacity;
- quarantine capture atomically consumes the exact live v0.111.1 reservation
  lease under the v0.111.2 clock/privacy contract and converts it into the
  candidate's durable quota charge while
  publishing either one complete `BoundedQuarantinedCandidate`/bundle generation
  or no durable quarantine object; expiry, short writes, crashes, cancellation,
  and `ResourceLimit` clean partial bytes, resume state, and accounting without
  an evaluated/policy-denied/authority result;
- separately bounded abuse digests/receipts remain local transport telemetry and
  cannot satisfy semantic evidence, prove the refused candidate, or replace its
  bytes/signature/transcript;
- deterministic expiry and deletion policy;
- crash-safe quarantine transaction and cleanup journal; recovery resolves every
  lease under v0.111.1/v0.111.2 and cannot move a partially staged bundle into
  trusted storage, infer a completed trust stage, retain an orphan reservation,
  or compare a prior process epoch's monotonic deadline;
- shadowing, index poisoning, trusted-reference substitution, quota, and
  materialization-bypass tests.

Verification:

- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`
- quarantine boundary integration suite.

Exit criteria:

- Quarantined data cannot satisfy any trusted object, proof, fact, policy, or
  materialization dependency.
- Re-admission executes the complete typed ingest pipeline without carrying
  quarantine trust state forward.
- v0.113.0 semantic verification can begin only from this isolated namespace
  and must bind every resume checkpoint to its exact quarantine generation.
- No decryption or resumable semantic state begins while bytes exist only behind
  a mutable source `PreflightedBundle` handle.
- An unsealed `IntegrityBoundBundle` is not parsed directly from file or mmap
  after a separate hash pass.
- Aggregate quarantine remains bounded under identity/bundle/session fanout;
  every durable candidate has complete charged capacity, while refusal and
  partial cleanup leave no candidate or authority claim.
- Slow, reconnecting, or trickle-progress senders cannot hold pre-candidate
  capacity beyond the bounded idle and absolute lease lifetimes.
- Restart retains only atomically complete charged candidates; incomplete leases
  expire and clean up without wall-clock reconstruction or privacy downgrade.

### v0.113.0 - Bundle Import

Goal: import verified bundles safely, including encrypted bundles.

Deliverables:

- `saga bundle import`;
- object deduplication;
- fact deduplication;
- lazy quarantine without worktree materialization;
- per-import byte, object, ancestry, fanout, deterministic work, and logical
  memory budgets plus a separate local monotonic cancellation deadline;
- streaming enforcement independent of manifest estimates;
- cancellation and resumable import;
- canonical resumable verification/import checkpoint binds bundle digest,
  outer-manifest digest, exact v0.110.3 `IntegrityBoundBundle` destination
  identity/seal status and quarantine namespace/generation, exact v0.110.4
  trusted-read/authenticated byte/chunk boundary, inner schema and verifier
  versions, verification mode, recipient/audience and key-session identity,
  realm/policy/crypto/revocation context,
  durable operation ID, immutable original ceilings, deterministic cost-table
  version fixed by protocol or signed canonical realm state under v0.12.4 rather
  than local/bundle input, consumed counters and exact remaining allowance by
  class, completed trust-stage bitmap/list, and transcript root/result for every
  completed stage;
- in-memory resume capability is non-cloneable, while the persisted checkpoint
  follows v0.110.2 durable generation/CAS/lease single-consumption semantics,
  is bounded in size/lifetime, and is admitted only when every binding still
  matches current quarantine bytes, verifier, recipient session, policy,
  revocation state, operation ID, ceilings, counters, and remaining allowance;
- cancellation, panic, process crash, key-session close, policy/crypto epoch
  change, quarantine rewrite, verifier upgrade, or bundle replacement invalidates
  or deterministically rolls back the affected checkpoint; it cannot skip
  decryption, canonical decode, graph, signature, revocation, policy, or WAL
  stages;
- checkpoint expiry uses causal/checkpoint or admitted v0.28.0 authority
  semantics rather than local wall clock when it affects trust;
- a local monotonic deadline can cancel only with `Incomplete`, cannot publish
  a trusted root, and does not alter deterministic consumed/remaining counters;
- decrypt-before-import policy;
- whole-record and chunked decryption inherit v0.23.2 and v0.92.2: no
  unauthenticated plaintext release, exact per-suite nonce/subkey/retry rules,
  resume only after authenticated chunks, no serialized raw AEAD/provider-key
  state, and no authoritative prefix or external side effect before the final
  completeness record;
- explicit v0.110.1 state progression from outer preflight to quarantine,
  decryption, canonical inner decode, body-derived graph closure, signature/
  revocation/time checks, compound policy/obligation admission, WAL commit, and
  trusted publication;
- each incoming transparency candidate remains a
  `BoundedQuarantinedCandidate` through decryption and complete verification and
  becomes `AdmittedAuthorityEvidence` only in the final expected-head authority
  WAL admission; resource/policy/verification refusal cannot be serialized as an
  authoritative rejection transition or used to evict prior admitted evidence;
- resumable import checkpoints retain the original aggregate quarantine and work
  ceilings; reconnect, resubmit, identity changes, parallel imports, or bundle
  splitting cannot reset/stack capacity, and a sender remains responsible for
  every candidate refused before bounded quarantine admission;
- world alias import policy;
- resource-budget comparison before trust;
- refusal when bundle policy requires stronger verification than local config;
- quarantine-on-policy-failure behavior;
- wrong bundle/manifest/chunk, verifier/schema downgrade, verification-mode
  change, recipient/audience substitution, budget reset, completed-stage
  forgery, key-session reuse, policy epoch change, persisted-checkpoint clone,
  CAS double-consume, snapshot/store clone, cancellation, and crash resume tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-store`

Exit criteria:

- Import cannot overwrite local world aliases without explicit policy.
- Import can place data in quarantine for inspection without trusting or
  materializing it.
- Budget refusal leaves no partially trusted alias, index, or worktree state.
- `ResourceLimit` before bounded quarantine leaves no durable candidate and says
  nothing about semantic validity; after bounded quarantine it remains a local
  incomplete/refusal result until a separate complete authority admission.
- Failure or cancellation at any pre-commit stage leaves only bounded
  quarantine state; recovery cannot resume at a later trust stage without
  revalidating the bound transcript and all prior capabilities.
- Resumption can save work but cannot change the bundle, bytes, verifier, key
  session, audience, policy context, budget accounting, or completed trust
  semantics under which that work was performed.
- Persisted checkpoints are single-consumption store records, not magically
  non-copyable files; rollback/clone limits and duplicate-publication defenses
  are exactly those declared by v0.110.2.

### v0.114.0 - Sync Negotiation

Goal: exchange local and remote heads before transfer.

Deliverables:

- remote head request;
- authenticated negotiation transcript binding realm and genesis commitment,
  endpoint identities, protocol version, critical capabilities, crypto suite,
  private-realm mode, and required verification level;
- unknown-critical-feature rejection;
- capability and verification-requirement stripping detection;
- compact causal-context or Merkle-clock exchange;
- missing object response;
- missing fact response;
- protocol version negotiation;
- encrypted realm mode negotiation;
- remote resource estimate exchange;
- authenticated negotiation binds receiver-advertised aggregate preflight/
  quarantine ceilings and current availability class without promising future
  capacity; sender claims cannot raise them, and privacy-preserving output does
  not expose per-actor occupancy or whether a specific signed candidate exists;
- negotiation can return backpressure or `ResourceLimit` before candidate bytes
  transfer, but that transport result is not policy denial, signature-invalid
  evidence, an authority transition, or proof that the candidate was evaluated;
- minimum verification mode negotiation;
- replay rejection metadata;
- private set reconciliation option for private realms;
- equivocation evidence exchange without recursive fork expansion;
- explicit rule that remote requirements may strengthen local verification but
  cannot redefine canonical realm validity.

Verification:

- `cargo test -p sagnir-sync`

Exit criteria:

- Sync can determine the smallest required bundle for a remote.
- Sync can determine whether local verification budgets satisfy remote trust
  requirements before transfer.
- Sync can determine that a receiver currently refuses candidate transfer for
  local capacity without converting that refusal into shared authority history.
- Both peers authenticate the exact negotiated security parameters before
  exchanging trusted bundle state.

### v0.115.0 - Partition And Anti-Entropy Model Gate

Goal: model distributed convergence and hostile ordering before implementing
live sync transfer.

Deliverables:

- TLA+/PlusCal or equivalent partition and reconciliation model;
- reorder, replay, duplication, delay, disconnect, and resume states;
- concurrent world advancement and multi-head preservation;
- concurrent private creation of identical canonical plaintext with one
  deterministic locator and multiple random-blinded semantic commitments;
- duplicate-equivalence transitions and future-reference representative policy
  without rewriting old signed references;
- representative-selection compare-and-swap over the expected equivalence root
  and prior representative, explicit multi-head conflict preservation, and
  authorized multi-parent conflict resolution;
- representative choice independent of attacker-controlled blinding values,
  locators, ciphertext IDs, signatures, and transition hashes;
- canonical authenticated locator B+ tree reconciliation with deterministic
  concurrent union/split, quota carry-forward, and bounded logarithmic proof
  semantics;
- escrowed aggregate quota-right allocation, offline consumption, causal
  transfer, merge-time double-spend detection, overdraw quarantine, governance
  redistribution, and locator-epoch migration states;
- untrusted, bounded-quarantined, and admitted-authority candidate states with
  aggregate receiver capacity, atomic quarantine reservation, pre-admission
  `ResourceLimit`, sender-held retry, reconnect/session/identity fanout, bounded
  abuse receipts, crash cleanup, and admitted-evidence non-eviction;
- pre-candidate reservation-lease states include idle and absolute expiry,
  progress only after charged consumption, non-resettable reconnect/resume/
  transfer lineage, per-peer/store concurrency and aggregate ceilings, fair
  scheduling, work-before-use debit, atomic candidate-charge conversion, and
  startup orphan reconciliation; every persisted deadline carries a process
  `ClockEpoch`, epoch mismatch cleans incomplete state without wall-clock
  reconstruction, and complete candidate publication is retained independently;
- checkpoint, policy epoch, evidence, and key-rotation interactions;
- equivocation and bounded fork handling;
- invariants for no lost heads, no lost duplicate identity, no locator-based
  identity collapse, no last-writer-wins representative selection, no
  randomness-grindable representative priority, no quota reset through replica
  or locator rotation, no aggregate offline overdraw becoming authoritative, no
  quota-right double-spend, no quarantine-capacity multiplication through Sybil/
  session/bundle fanout, no resource refusal as authority evidence, no arrival-
  order authority selection, no stalled or trickle-progress reservation beyond
  its absolute lifetime, no reconnect/resume deadline reset, no work before
  charge, no cross-epoch monotonic comparison, no orphan reservation after
  recovery, no operational-metadata privacy downgrade, no linear locator-proof
  requirement, no stale admission, no partial trust, and eventual convergence
  under documented assumptions;
- bounded model-check command required by the release gate.

Verification:

- bounded partition model check;
- model invariant review.

Exit criteria:

- Sync transfer implementation begins only after no known model counterexample
  can discard an admitted head or duplicate semantic identity, rewrite a signed
  reference through locator equivalence, resolve concurrent representative
  transitions by arrival order, or trust partial remote state.

### v0.115.1 - Sealed-Private Distributed Invariant Closure Gate

Goal: compose every sealed-private format admitted after the original partition
model into the pre-sync distributed model before v0.116.0 transfers live state.

Deliverables:

- extend the v0.115.0 model with exact v0.92.1 multi-instance forward and
  reverse logical index keys and canonical ledger-projection completeness;
- concurrent creation, transfer, merge, redaction, reintroduction, and
  reconciliation of multiple encryption instances for one semantic commitment
  without instance aliasing, omission, overwrite, or quota-class confusion;
- malicious authorized logical-manifest publisher states covering omitted,
  invented, duplicated, or substituted projection entries and full-view
  signer/witness equivocation;
- compartment-root and opaque realm-manifest reconciliation across concurrent
  compartment updates, handle rotation, commitment-key rotation, partial-access
  peers, and stale/forked manifest proofs;
- proof that partial-access peers converge on their authorized compartment
  without learning hidden compartment handles, counts, roots, locators, names,
  or structure and without claiming independently verified global completeness;
- endpoint-local placement divergence and reconciliation across re-encryption,
  repack, receipt renewal, relocation, endpoint loss/rejoin, and concurrent
  endpoint updates;
- invariant that canonical logical/realm roots converge independently of
  endpoint placement and no endpoint projection overwrites another by arrival
  order;
- quota-right expiry states using v0.28.0 authority sequences, checkpoints,
  revocation roots, freshness bounds, quorum/diversity, unavailable/conflicting
  authorities, pre-expiry creation with late delivery, unseen expiry,
  concurrent spend/transfer and expiry, and local clock rollback;
- target-only attestation issue, sync, expiry, revocation, stale/forked staple,
  issuer key rotation/compromise/equivocation, audience replay, and explicit
  authority-assertion claim limits;
- neutral-object creation, recipient authorization, shared cross-compartment
  reference, intentional correlation, rekey, recovery, redaction, erasure, and
  stale-epoch reconciliation;
- canonical `RedactedPlaceholder` propagation, target-only disclosure, audit
  provenance, stale ciphertext return, repair/archive response, concurrent
  reintroduction, and proof/availability/completeness refusal;
- bounded anti-entropy ordering that sends current tombstones, revocations,
  authority checkpoints, logical manifests, compartment proofs, and projection
  evidence before bodies or endpoint repair candidates that depend on them;
- bounded anti-entropy candidate admission models outer preflight and aggregate
  quarantine reservation before durable capture, unlimited validly signed stale
  submissions, many identities under one store/realm ceiling, reconnect/resubmit
  floods, partial capture/restart cleanup, sender-held unadmitted evidence, and
  non-eviction of admitted ambiguous/superseded authority evidence; reservation
  leases retain original deadlines/counters across reconnect, resume-token and
  session replacement, charge progress/work before use, expire atomically, and
  convert exactly once into durable candidate quota; process-epoch mismatch
  retains complete charged candidates but cleans incomplete leases, and
  protected profiles keep lease identity/activity inside encrypted opaque
  operational metadata;
- model invariants for no lost encryption instance, no semantic/index
  completeness gap, no Byzantine manifest omission accepted, no hidden
  compartment disclosure, no endpoint-placement overwrite, no clock-derived
  quota extension, no stale attestation accepted as current, no neutral
  lifecycle bypass, no placeholder upgraded to content, no stale ciphertext
  resurrection, no unbounded receiver quarantine from signed/Sybil/resubmit
  floods, no slow sender retaining pre-candidate capacity past its hard lifetime,
  no reconnect or one-byte progress extending that lifetime, no pre-admission
  refusal interpreted as shared authority evidence, no cross-process monotonic
  deadline reuse, no lease-metadata identity/timing leak beyond the selected
  profile contract, no arrival-order transition selection, and eventual
  convergence under documented authority, availability, and partition
  assumptions;
- counterexample corpus and deterministic bounded model-check command required
  by the v0.116.0 release gate;
- immutable model-run manifest records model source digest, tool and solver
  versions, exact command/configuration, random or search seeds, host resource
  limits, operational wall-clock timeout, state/depth/transition limits, result
  digest, and completion status; a timeout is recorded as incomplete evidence
  and cannot establish an invariant;
- model bounds explicitly record active/retired replicas and incarnations,
  actors/devices, compartments and neutral domains, semantic commitments,
  encryption instances per commitment, endpoints, placement generations,
  authorities and quorum size, witnesses and threshold, worlds/heads, events,
  checkpoints, delta-chain length, messages, partitions, retries, and failures;
- assumptions document reliable/unreliable delivery, eventual delivery,
  fairness for enabled actions and schedulers, crash/recovery, Byzantine versus
  honest roles, authority availability, bounded storage/resources, and whether
  a liveness result depends on each assumption;
- every property is classified as safety, invariant, refinement, bounded
  reachability, or liveness; successful safety exploration is never reported as
  proof of liveness;
- symmetry reduction, partial-order reduction, state hashing, abstraction,
  omitted data domains, and any other state-space reduction are declared with
  justification and a check that the reduction preserves each claimed
  property;
- run artifact records explored/distinct states, generated transitions, maximum
  depth, queue/frontier size, collisions if applicable, pruned states by
  reduction, elapsed time, peak memory, completion versus timeout/exhaustion,
  and uncovered bounds;
- two successful runs are comparable only when their model/configuration,
  assumptions, reductions, bounds, and completion status match; a smaller or
  timed-out run cannot silently replace a stronger release-gate artifact;
- deterministic CI smoke bounds and larger release bounds are separate named
  profiles, both reproducible from versioned manifests; release claims name the
  exact profile and explored coverage;
- counterexamples retain the complete reproducer trace, seed/configuration,
  model/tool versions, and minimized trace where available;
- explicit implementation stop: live sync transfer cannot begin until every
  invariant passes and no unresolved counterexample remains.

Verification:

- composed sealed-private partition model check;
- malicious manifest-publisher model;
- authoritative-time/revocation partition model;
- partial-access and endpoint-placement disclosure review;
- model invariant review;
- reproducibility rerun from the recorded model-run manifest;
- model coverage and assumption audit.

Exit criteria:

- Every distributed invariant introduced by v0.92.1, v0.99.2, and the v0.28.0
  authority substrate is executable before live sync exists.
- Sync cannot accept an incomplete logical projection, alias an encryption
  instance, overwrite endpoint-local placement, extend expired quota rights,
  overclaim a target attestation, bypass neutral policy, or resurrect content
  through a redacted placeholder.
- v0.116.0 begins only after the composed model has no known counterexample
  within admitted bounds.
- "Within admitted bounds" is a reproducible machine-readable claim naming
  exact bounds, assumptions, reductions, property classes, explored-state
  counts, completion status, timeout/resources, and model/tool digests.

### v0.116.0 - Sync Transfer

Goal: transfer proof-carrying bundles to a remote endpoint.

Deliverables:

- `saga sync`;
- `saga clone`;
- `saga clone --no-worktree`;
- transport-independent framed protocol;
- chunk acknowledgement;
- authenticated resume tokens scoped to the session, endpoint identities,
  realm/genesis, bundle, byte or chunk range, and negotiated transcript;
- resume-token expiry, cancellation, single-use or replay window, and key
  rotation behavior;
- per-peer quotas and backpressure;
- receiver-side outer preflight and aggregate quarantine reservation occur before
  a candidate bundle crosses into durable quarantine; insufficient capacity
  returns `ResourceLimit`, transfers no semantic/admitted candidate state, and
  leaves the sender responsible for preservation/resubmission;
- reconnect, resume-token rotation, bundle splitting, parallel sessions, peer/
  actor/device/replica identity changes, and valid-signature floods share the
  store/realm aggregate ceilings; they cannot multiply capacity or evict already
  admitted ambiguous/superseded authority evidence;
- live transfer reservations inherit v0.111.2 process-epoch expiry and metadata-
  protection rules; process restart cleans incomplete transfer leases before a
  new attempt, and protocol/log output exposes only profile-approved opaque or
  coarse operational fields;
- transfer cancellation;
- accepted response;
- denied response;
- quarantined response;
- local-budget-insufficient response;
- blind remote response;
- split-trust remote response;
- local sync result fact;
- protocol tests;
- release-gate dependency on the successful v0.115.1 composed model artifact;
- capability stripping, transcript substitution, token replay, cross-endpoint,
  cross-bundle, expired token, cancelled token, and range-extension tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-cli`

Exit criteria:

- Local work can sync without requiring a hosted product, including encrypted
  blind-storage workflows.
- Clone and sync do not silently downgrade verification; they either satisfy
  remote requirements, quarantine fetched state, or refuse trust/materialization.
- Resumption cannot continue under a different endpoint, bundle, range,
  capability set, or verification requirement.
- Backpressure and resource refusal affect transport scheduling only; arrival
  order and receiver capacity never select the authoritative competing
  transition or become authoritative rejection evidence.

### v0.117.0 - Sparse Materialization And Partial Clone

Goal: let large or compartmentalized realms fetch and materialize only admitted
state.

Deliverables:

- sparse path and compartment selection;
- state-only, receipt-only, and no-worktree clone modes;
- promised-object and missing-body representation;
- proof boundaries for omitted subtrees;
- on-demand object fetch;
- compartment-translation preflight enumerates every promised descendant,
  canonical body, semantic-commitment opening, object-type decoder, source
  reference, and target-policy input required by the v0.100.0 typed
  transformation relation;
- before committing a cross-compartment copy or move, Sagnir fetches,
  authenticates, canonically decodes, and verifies every required promised
  descendant and opening under the source root;
- missing, unavailable, quarantined, corrupt, never-fetched, or opening-less
  required descendants cause explicit translation refusal; policy cannot treat
  an unavailable body as proof of equality or structural isomorphism;
- an opaque boundary may remain unexpanded only when its canonical object type
  is explicitly declared compartment-neutral, its identity contains no source-
  compartment key/domain input, its boundary proof is complete, and target
  policy admits that exact opaque type; ordinary sealed-private trees,
  containers, blobs, or unknown object types are not opaque escape hatches;
- compartment-neutral boundaries must verify the v0.100.0 neutral commitment
  domain, allowlisted type, neutral-only reference closure, and prohibition on
  compartment-bound metadata; a merely absent or stripped compartment field is
  malformed rather than neutral;
- a `RedactedBody` descendant remains a historical redacted reference and
  cannot produce a new equivalent target commitment or content-equality proof;
  copy/move refuses unless a separately specified target transition records the
  canonical v0.99.2 `RedactedPlaceholder` without claiming content translation,
  and protected policy may prohibit that projection;
- `RedactedPlaceholder` is distinct from missing, promised, unavailable,
  quarantined, and `RedactedBody`; it exposes no source commitment to
  target-only recipients and carries only an encrypted audit-provenance
  reference for authorized views;
- a placeholder cannot satisfy body availability, subtree completeness,
  materialization, repair, archive restore, build/test input, or proof
  obligations, and stale ciphertext cannot replace it;
- replacement requires an explicit authorized reintroduction transition with a
  new encryption instance and current policy evaluation;
- sparse translation fetches are transaction-scoped, bounded, cancellable,
  resume-bound to the manifest/CAS inputs, and temporarily GC-pinned until final
  commit or cleanup;
- policy refusal when full materialization is required;
- sparse update, promised-descendant fetch, missing opening, unavailable body,
  redacted descendant, compartment-neutral opaque boundary, prohibited opaque
  container, placeholder/body confusion, placeholder availability or proof
  misuse, stale-ciphertext replacement, authorized reintroduction, cancellation,
  resume, and missing-object tests.

Verification:

- `cargo test -p sagnir-sync`
- `cargo test -p sagnir-worktree`

Exit criteria:

- Omitted state is explicit and cryptographically committed, never confused
  with verified absence.
- Cross-compartment translation never silently retains a source-compartment
  reference because sparse, promised, unavailable, or redacted descendants were
  not locally materialized.
- A redacted placeholder remains an explicit non-content state and cannot be
  upgraded into availability, completeness, repair, or proof success.

### v0.118.0 - Native Transport Adapters

Goal: carry the same Sagnir protocol over practical decentralized transports.

Deliverables:

- removable-file/bundle adapter;
- SSH or stdin/stdout adapter;
- QUIC adapter;
- transport authentication and endpoint identity binding;
- exact sync-negotiation transcript binding independent of transport;
- transport-independent transcript tests;
- replay, truncation, reordering, disconnect, and resume tests.

Verification:

- `cargo test -p sagnir-sync`
- local transport integration tests.

Exit criteria:

- Protocol meaning does not change with transport.
- Local-first exchange remains possible without a hosted service.

### v0.119.0 - Git Import And Export Bridge

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

### v0.120.0 - Private Anti-Entropy And Discovery

Goal: reconcile encrypted peers while limiting head, graph, and access-pattern
leakage.

Deliverables:

- private set reconciliation for heads and fact roots;
- encrypted reconciliation of locator search-tree roots, logarithmic
  inclusion/absence/range proofs, semantic-commitment reverse-index roots, and
  admitted duplicate-equivalence transitions between authorized peers;
- node/range-difference reconciliation with bounded resumable traversal,
  immutable private logical-node reuse, independent randomized envelope
  transfer/placement, deterministic concurrent union/split, and declared
  proof/read/write amplification limits;
- exchange of canonical per-replica and actor/device aggregate quota counters,
  signed escrow-right allocations, spent-right commitments, causal rights-
  transfer transitions, unresolved quota-conflict roots, quota carry-forward
  transitions, and duplicate-amplification evidence so reconnect, new
  incarnation, locator rotation, partition merge, or replay cannot reset or
  double-spend an offline creator's allowance;
- explicit propagation of conflicting equivalence-transition heads without
  selecting a representative by message arrival order;
- no raw locator, semantic commitment, duplicate relation, actor, recipient, or
  transparency-monitor identity disclosed to blind stores;
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
- Offline duplicate identities converge as explicit encrypted state rather than
  being collapsed because their private locators match.
- Aggregate overdraw discovered at merge remains authenticated quarantine and
  does not become admitted duplicate history until an explicit governed
  resolution supplies valid rights or rejects the dependent state.
- Locator-index reconciliation remains bounded per operation with logarithmic
  proofs while preserving every admitted node, candidate, and conflict head.

### v0.121.0 - Minimal Daemon

Goal: provide optional local and remote daemon support.

Deliverables:

- `sagad serve`;
- remote object store;
- remote fact store;
- policy-light opaque storage acceptance limited to canonical outer framing,
  declared quotas, ciphertext chunk/hash/Merkle integrity, storage capability,
  and receipt semantics;
- blind daemon APIs and CLI output use the v0.110.1 claim taxonomy and never
  report semantic object, graph, signature, policy, world, or promotion validity
  without an explicitly configured trusted/decrypting verifier role;
- stable-account quota mode documents identity correlation; unlinkable quota
  mode requires pre-issued epoch-scoped rights/nullifiers and is not simulated
  by hiding an account label;
- graceful shutdown;
- restart tests.

Verification:

- `cargo test -p sagad`
- `cargo run -p sagad --bin sagad`

Exit criteria:

- A minimal Sagnir remote exists for sync testing.
- A blind remote can prove ciphertext integrity and availability within its
  declared assumptions without being mistaken for an authorized semantic
  verifier.

### v0.121.1 - Erasure Evidence Format Admission Stop

Goal: freeze the irreversible erasure state machine, evidence formats, and
provider contracts before v0.122.0 dispatches any destructive operation.

Deliverables:

- reviewed algorithm/format admission document with no destructive production
  implementation or compatibility claim;
- inherit the v0.92.1 disposable prototype boundary: provider simulators,
  reference decoders, fuzz harnesses, models, and benchmarks cannot dispatch a
  real destructive request, write durable realm data, enter production feature
  graphs, emit authoritative evidence, or create a compatibility promise;
- prototype code and experimental bytes are replaced/deleted or explicitly
  promoted only through post-admission production review, independent vectors,
  and release-gate coverage;
- canonical monotonic operation-phase and orthogonal component-result schema;
- complete transition table covering preparation, tombstone commitment,
  destructive dispatch, uncertain outcomes, confirmed destruction, controlled
  copies, storage notices, residual copies, and terminal uncertainty;
- canonical destruction intent, provider request, idempotency token, response,
  outcome query, contradiction, supersession, and terminal-disposition formats;
- canonical authenticated destruction-evidence envelope and selective
  disclosure statement formats with exact visibility rules;
- provider capability contract for idempotency, post-crash query, authentication,
  assurance, revocation, retirement, compromise, contradiction, and permanent
  ambiguity;
- local wrapper/key-slot and wrapping-epoch destruction evidence formats;
- recovery-path enumeration and proof that `KeysDestroyed` cannot be reached
  while any wrapper, share, escrow copy, ancestor derivation path, provider
  request, controlled copy, or required evidence remains unresolved;
- mixed-content pack, endpoint notice, receipt supersession, repair, backup, and
  archive integration interfaces for later milestones;
- canonical redaction tombstone, `RedactedBody`, and v0.99.2
  `RedactedPlaceholder` interaction rules;
- model proving stale ciphertext, repair, restore, placeholder substitution, or
  event reordering cannot resurrect a destroyed encryption instance;
- crash-recovery model covering provider success before local confirmation and
  permanent `DestructionUncertain`/`ResidualUncertainty`;
- privacy review proving evidence IDs, provider/key metadata, timing,
  checkpoints, operation linkage, and assurance do not leak to blind stores,
  ordinary diagnostics, filenames, telemetry, or locked status;
- independent canonical vectors for every operation, evidence, provider, local
  wrapper, tombstone, notice, and terminal-disposition format;
- format-specific reference decoders, seed corpus, and fuzz targets for every
  operation phase, component result, destruction intent, provider request and
  response, outcome query, evidence envelope, selective disclosure, local
  wrapper record, tombstone, notice, contradiction, and terminal disposition;
- early go/no-go benchmark thresholds for redaction preflight, wrapper and
  recovery-path enumeration, DEK unwrap/rewrap planning, evidence verification,
  provider-outcome recovery, tombstone/status lookup, and bounded journal
  replay;
- format admission fails when p95 latency, peak memory, evidence size, provider
  fanout amplification, or journal replay cost exceeds declared thresholds;
- explicit implementation stop: v0.122.0 may not dispatch a destructive request
  or persist final erasure formats until this admission is complete.

Verification:

- independent erasure-format vector validator;
- bounded irreversible-state-machine model;
- provider-contract conformance fixtures;
- evidence-privacy and non-resurrection review;
- format-specific fuzz smoke suite;
- admission benchmark runner and recorded threshold artifact.

Exit criteria:

- Every irreversible transition and ambiguous recovery outcome has one
  canonical meaning before destructive code exists.
- Evidence authentication, privacy, assurance, and provider lifecycle behavior
  are independently reviewable.
- No implementation can report erasure without complete admitted evidence for
  every declared recovery path.
- Erasure formats pass parser-hardening and operational performance rejection
  thresholds before destructive implementation begins.

### v0.122.0 - Redaction And Cryptographic Erasure

Goal: implement the irreversible local redaction and key-destruction state
machine, preserve honest historical evidence, and define normative contracts
for later receipt, repair, and archive integrations.

Deliverables:

- implementation of the formats, state machine, provider contracts, privacy
  rules, and non-resurrection invariants admitted by v0.121.1;
- durable encoding as one monotonic irreversible operation phase plus
  orthogonal per-recovery-path, controlled-copy, storage-notice, and residual
  results, never one mutually exclusive flat enum;
- canonical irreversible operation phases:
  - `Planned`: scope selected, with no authoritative mutation;
  - `Prepared`: authorization, legal hold, ownership, shared-instance handling,
    controlled-backup inventory, replacement-pack plan, provider capability
    preflight, and pre-erasure verification receipt validated while rollback
    remains possible;
  - `TombstoneCommitted`: the private semantic tombstone is authoritative, but
    no destructive request has been dispatched and a signed
    abort-before-destruction outcome remains possible;
  - `DestroyingKeys`: durable destruction intents and idempotency tokens are
    committed before the first local, filesystem, KMS, HSM, escrow, recovery-
    share, or wrapper destruction request is dispatched; the operation can no
    longer abort because an external request may already have succeeded;
  - `KeysDestroyed`: durable admitted evidence confirms that every enumerated
    recovery path is unavailable under its declared destruction assumptions;
- derived `DestructionUncertain` status whenever the operation phase is
  `DestroyingKeys` and at least one destructive request may have succeeded but
  lacks durable confirmation after crash, timeout, provider partition, or
  ambiguous local persistence;
- derived post-destruction status labels including `StorageNoticesPending`,
  `ControlledCopiesCleared`, `Complete`, and `ResidualCopiesKnown`; these labels
  summarize orthogonal monotonic component results and are not operation
  phases;
- per-wrapper and per-provider durable destruction records binding the
  redaction operation, encryption instance, recovery-path kind, provider and
  key identifier, wrapping or share epoch, request precondition, idempotency
  token, request transcript hash, provider result, query result, evidence type,
  and last verified checkpoint;
- canonical destruction-evidence envelope with domain-separated version and
  evidence ID, realm and redaction operation ID, encryption instance, recovery-
  path kind, provider identity and provider-key epoch, key/slot/share/wrapper
  identifier, idempotency token, exact request transcript commitment, result,
  assurance level, authoritative checkpoint or admitted time statement,
  response sequence, and evidence payload commitment;
- destruction-evidence visibility matrix:
  - the full envelope, evidence ID, provider identity, provider-key epoch,
    key/slot/share/wrapper identifiers, timestamps, checkpoints, request
    transcript, assurance level, and payload remain encrypted inside the
    authorized semantic ledger and recipient-scoped recovery/audit packages;
  - blind stores receive no destruction-evidence ID, provider identity, key
    metadata, timing, checkpoint, assurance level, operation linkage, or
    evidence payload;
  - public or selective disclosure emits a separate audience-, purpose-, realm-,
    target-, policy-, and expiry-bound minimal statement containing only
    deliberately disclosed predicates, never the full private envelope;
  - logs, telemetry, crash reports, filenames, directory names, process titles,
    unauthenticated status output, shell completion, and default diagnostics
    cannot expose evidence fields or stable correlators;
- private evidence references in status and operation journals use encrypted
  local handles; locked output reports only coarse policy state such as
  verified, uncertain, or residual without provider or key-slot attribution;
- admitted evidence authentication is a provider signature, hardware
  attestation bound to an admitted trust root, or authenticated local key-agent
  statement whose signing key and process boundary are governed and
  checkpointed; a cached TLS response, unsigned API body, log line, exit code,
  file absence, or operator assertion is not transferable destruction proof;
- evidence assurance levels distinguish provider assertion, authenticated
  software-agent deletion, hardware-backed slot destruction, wrapping-epoch
  destruction with complete surviving-key rewrap, and reviewed puncture proof;
  policy states which levels can satisfy each recovery path;
- evidence verification checks provider-key validity at the evidence
  checkpoint, request/response binding, idempotency token, result semantics,
  revocation and compromise state, provider retirement, replay domain, and
  supersession or contradiction;
- historical evidence remains immutable after provider-key revocation,
  compromise, or retirement, but its current assurance is reevaluated under
  policy; compromised or equivocal evidence cannot silently retain a verified
  erasure claim;
- canonical encoding, signature/attestation, replay, wrong-operation,
  wrong-key, wrong-token, stale checkpoint, revoked provider, compromised key,
  retired provider, assurance downgrade, evidence substitution, blind-store
  leakage, log/telemetry leakage, locked-status leakage, cross-audience replay,
  and overbroad selective-disclosure vectors;
- recovery-path result states for not requested, request durably prepared,
  dispatched, confirmed not destroyed, confirmed destroyed or revoked,
  uncertain, refused, unreachable, and residual-by-policy;
- transition table, durable journal encoding, idempotent resume tokens, allowed
  retry and provider-query actions, and prohibited backward transitions for
  every phase and component result;
- external providers must declare destruction idempotency, status-query, and
  evidence semantics before use; strict erasure policy refuses a provider whose
  post-crash outcome cannot be queried or otherwise durably proven;
- post-crash recovery queries each provider with the original operation and
  idempotency token before retrying; Sagnir never interprets timeout, missing
  response, local key-file absence, or an uncommitted success response as proof
  of destruction;
- a confirmed-not-destroyed path may receive a newly journaled retry intent,
  while an uncertain request is never replaced with a fresh request merely to
  make the journal appear complete;
- a provider response that contradicts earlier destruction evidence creates
  immutable provider-equivocation and compromise evidence, changes the local
  erasure assurance to fail closed or residual, and never rewrites the prior
  transcript or recreates key material;
- `KeysDestroyed` requires durable confirmation for every wrapper, share,
  escrow copy, local key instance, ancestor derivation path, and external
  provider entry enumerated by `Prepared`; one uncertain path keeps local
  cryptographic erasure unverified and fail closed;
- local wrapped-key removal follows the v0.91.0 erasable-wrapping contract:
  ordinary deletion or overwrite is never confirmation, and each local path
  requires evidence for destruction of an independent per-erasure-unit KEK/key
  slot or evidence that the parent wrapping epoch rotated, every surviving DEK
  was transactionally rewrapped, and the old epoch became unavailable;
- filesystem journals, CoW snapshots, volume snapshots, retained block images,
  SSD wear-leveling, and recovered deleted records are modeled as residual
  wrapper copies whenever a surviving parent key could still open them;
- if local media assumptions or surviving parent keys permit wrapper recovery,
  Sagnir reports local erasure as unverified or residual even when the current
  namespace no longer contains the wrapper;
- stable redaction operation identity and `saga vault redaction status` and
  `saga vault redaction resume` commands that expose the durable state,
  component results, next permitted actions, and point-of-no-return status;
- abort requires a signed outcome that preserves the tombstone and records that
  no key destruction occurred, and is permitted only while the phase remains
  `TombstoneCommitted` with no destructive dispatch;
- once `DestroyingKeys` begins, recovery is forward-only for the tombstone and
  every possibly destroyed path: Sagnir may query, confirm, retry a confirmed
  non-destruction, or record residual uncertainty, but never recreate key
  material or return to an abortable phase;
- signed terminal `ResidualUncertainty` disposition may close operational work
  when one or more destruction outcomes can no longer be resolved;
- `ResidualUncertainty` binds the unresolved paths and last evidence, never
  claims cryptographic erasure, retains the tombstone and all destruction
  evidence, remains non-abortable, supports bounded journal compaction and
  explicit alert acknowledgement, and continues to block policy requiring
  verified erasure;
- later valid destruction evidence may advance a `ResidualUncertainty`
  operation to `KeysDestroyed` and subsequent completion, but cannot erase the
  uncertainty interval or rewrite its signed closure;
- after `KeysDestroyed`, recovery is forward-only: keys are never recreated,
  the tombstone is never rolled back, and retries may only clear copies, emit
  notices, collect acknowledgements, repack, or record residuals;
- post-destruction remote acknowledgement and controlled-copy work may complete
  in either order; the top-level state is derived from monotonic component
  results and never moves backward when one component remains pending;
- separate machine-readable result properties:
  - local cryptographic erasure: pending, destroying, uncertain, verified,
    residual uncertainty, confirmed incomplete, or failed closed;
  - controlled-copy clearance: not applicable, pending, verified, or residual;
  - remote deletion acknowledgement: not configured, pending, partial,
    acknowledged, refused, or unreachable;
  - uncontrolled copies: unknown, known residual, or explicitly non-recallable;
- `Complete` means configured obligations are met, never that every
  uncontrolled plaintext or key copy in the world was recalled;
- signed redaction transition identifying scope, authority, reason, and policy
  basis without embedding removed plaintext;
- private encrypted semantic tombstone binding the semantic commitment,
  encryption instance, erasure-unit scope, redaction transition, policy and
  crypto epochs, authority, reason, and effective causal frontier;
- a valid event concurrent with the tombstone may retain a historical reference
  to the erased encryption instance, but current resolution returns
  `RedactedBody` and never restores, requests, repairs, or materializes the old
  ciphertext;
- an event causally after the tombstone cannot make the erased encryption
  instance current again through ordering, merge, replay, alias movement, or a
  stale availability receipt;
- legitimate post-redaction reintroduction requires explicit authorization and
  a new encryption instance, independently random DEK, current selector and
  storage identity, current policy evaluation, and a new semantic commitment
  when the old commitment opening is unavailable; provenance may reference the
  historical tombstone but cannot clear or supersede it;
- stale or concurrent historical references to the erased instance remain
  verifiable signed history but cannot satisfy present availability, repair,
  materialization, promotion, or completeness policy;
- separate endpoint-scoped opaque storage deletion or supersession notice
  binding only the blind store's opaque storage-realm handle, storage-authority
  epoch, ciphertext storage IDs, pack generation and positions, receipt IDs,
  monotonic notice sequence, and replay domain;
- storage deletion notices are authorized by a governance-admitted
  storage-deletion key or scoped capability whose public verification state is
  pinned by that blind store without granting semantic-ledger access;
- blind-storage notices reveal no semantic commitment, path, actor,
  compartment, fact, reason, private locator, or private causal frontier;
- the authorized client resolves the private tombstone to each endpoint's
  current storage selectors through encrypted indexes and receipt lineage
  before issuing a notice;
- pack relocation and repacking update endpoint-local storage selectors without
  exposing a global old-to-new semantic mapping;
- notices are encrypted or transcript-bound to one endpoint and storage epoch,
  supersede only receipts already known to that endpoint, and cannot become a
  reusable cross-epoch or cross-provider correlation oracle;
- normative storage-notice verification and application contract that v0.124.0
  implements after receipt semantics exist; v0.122.0 admits schemas and vectors
  but does not claim live remote deletion acknowledgement;
- canonical `RedactedBody` state distinct from missing, promised, sparse,
  unavailable, quarantined, corrupt, or never-fetched state;
- redaction tombstones retained as authoritative history and propagated during
  anti-entropy before any request for a body or repair source;
- signed private pre-erasure verification receipt recording the semantic
  commitment, exact body and reference verification result, verifier version,
  policy and crypto epochs, causal frontier, verification scope, and the
  encryption instance verified before key destruction;
- the pre-erasure receipt remains historical evidence after erasure but states
  explicitly that commitment-to-plaintext recomputation is unavailable once
  the plaintext and blinding opening are no longer recoverable;
- destruction of independently wrapped erasure-unit data keys or admitted
  punctures, plus recipient revocation where applicable;
- enumeration and destruction of every wrapper, recovery share, escrow copy,
  and admitted key path for the targeted encryption instance;
- validation that no surviving ancestor key can recreate the erased key;
- erasure-unit scope and granularity committed into the redaction transition;
- reference and ownership check proving no other admitted scope still requires
  the targeted encryption instance;
- legal-hold and retention conflict evaluation that blocks redaction explicitly;
- copy-on-write separation before partial redaction of shared logical content;
- compartment, object, metadata, index, cache, and repack handling;
- implemented local and peer-sync rule prohibiting body requests, transfer
  admission, local repack, or cache reconstruction from resurrecting a
  tombstoned encryption instance;
- normative non-resurrection contracts for future availability repair and
  archive restoration, implemented and tested only when those paths arrive in
  v0.125.0 and v0.126.0/v1.6.0;
- pre-redaction ciphertext returned by a stale or offline peer enters
  non-materializable quarantine, is recorded as stale redaction evidence, and
  cannot satisfy a promise, receipt, repair, or availability requirement;
- receipt and archive supersession schemas derived from the private tombstone;
  executable remote receipt handling is deferred to v0.124.0 and archive
  behavior remains a v0.126.0 contract until v1.6.0 implements archival;
- backup, VM-snapshot, recovery-kit, and air-gapped-device restore admission
  begins in restricted non-decrypting, non-materializing mode;
- restore admission compares local checkpoints and crypto epochs with every
  configured later checkpoint, witness, peer, or governance recovery record
  before trusting restored indexes, wrappers, receipts, or ciphertext;
- current private redaction tombstones and endpoint storage notices are
  reconciled before restored ciphertext, indexes, or DEK wrappers can be
  admitted for decryption, materialization, repair, or repack;
- stale offline restores that cannot establish a policy-sufficient current
  redaction frontier warn in permissive profiles and refuse protected
  materialization in strict profiles;
- controlled-backup inventory records which wrapping-key epochs and DEK
  wrappers may remain recoverable;
- when a controlled backup can restore both an old DEK wrapper and a surviving
  wrapping or compartment key, erasure requires sanitizing, replacing, or
  cryptographically superseding every controlled backup copy and then rotating
  the affected wrapping epoch and rewrapping every surviving DEK;
- cryptographic backup supersession is valid only when the backup uses an
  independently destroyable backup-encryption epoch, destruction of that epoch
  makes every superseded backup copy undecryptable, and the destruction evidence
  is included in the redaction result;
- a backup containing plaintext key material or key material recoverable from a
  surviving ancestor cannot be superseded by metadata alone and must be
  sanitized or treated as a surviving copy;
- if a controlled backup containing a recoverable old wrapper and wrapping key
  cannot be sanitized or superseded, Sagnir records the residual copy and
  refuses to report cryptographic erasure even when current storage is rekeyed;
- recovery kits cannot restore destroyed DEKs, superseded wrapping epochs, or
  authority predating the admitted redaction frontier;
- explicit documentation that an isolated restore cannot prove it has observed
  the latest redaction without a later checkpoint, witness, authorized peer, or
  governance recovery record;
- preserved historical commitments and redaction evidence;
- explicit distinction between redaction, logical removal, ciphertext deletion,
  and cryptographic erasure;
- statement that already replicated plaintext, keys, screenshots, logs, or
  exports cannot be recalled;
- interrupted purge, stale recipient, retained key copy, partial repack,
  historical proof, unauthorized redaction, surviving-ancestor reconstruction,
  private-locator-key retention, shared blob, shared tree, overlapping
  compartment, multiple wrapper, legal-hold, partial redaction, redaction-first
  anti-entropy, malicious storage deletion, private/storage projection
  substitution, notice replay, repack relocation, cross-epoch correlation,
  forged or substituted pre-erasure receipt, post-erasure receipt verification,
  stale receipt, stale archive, attempted repair, restored VM snapshot, old
  filesystem backup with retained wrapper and key, backup-encryption epoch
  destruction, unsanitizable backup refusal, stale recovery kit, and offline or
  air-gapped peer partition-return tests.
- state-transition, signed pre-destruction abort, crash-before-dispatch,
  provider-success-before-journal, provider-timeout, provider-query recovery,
  repeated idempotency token, unsigned response refusal, evidence-signature and
  attestation validation, provider revocation/retirement, assurance downgrade,
  unsupported-provider refusal, `DestructionUncertain`,
  `ResidualUncertainty` closure and later evidence advancement, local journal,
  CoW, snapshot and recovered-wrapper cases, all-path evidence,
  irreversible-boundary, forbidden rollback, idempotent resume, partial remote
  acknowledgement, out-of-order component completion, concurrent historical
  reference, post-redaction stale lineage, authorized reintroduction,
  status/resume output, residual-copy classification, and separate-result-
  property tests.

Verification:

- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-store`
- `cargo test -p sagnir-sync`
- independent destruction-evidence envelope and assurance vector validator;
- cryptographic-erasure recovery suite;
- deterministic core state-machine and offline-peer return simulation.

Exit criteria:

- Authorized redaction can make retained ciphertext undecryptable where key
  destruction assumptions hold without erasing the signed fact that a
  redaction occurred.
- Removing one recipient or reference does not erase content still required by
  another admitted scope; incompatible partial redaction first creates a
  separate encryption instance.
- Redaction is denied while a governing retention or legal-hold obligation
  requires the encryption instance.
- Missing, unavailable, and redacted bodies remain distinguishable, and no sync,
  sparse-fetch, local repack, cache, or GC path implemented by this release
  treats a redacted body as recoverable data.
- Storage-notice, repair, and archive integration contracts are canonical, but
  v0.122.0 does not claim executable guarantees for later subsystems.
- Restored state cannot decrypt or materialize protected content until it meets
  the configured current-redaction evidence requirement.
- Cryptographic erasure is not reported successful if the erased data key can
  be derived again from any retained master, compartment, epoch, recipient, or
  private-locator key.
- Sagnir never claims erasure of data or keys already copied beyond its control.
- Historical signatures and pre-erasure verification receipts remain
  checkable, while Sagnir states honestly when erased plaintext can no longer be
  reopened to recompute its semantic commitment.
- Once `KeysDestroyed` commits, no recovery path can return to a state in which
  the targeted DEK is available.
- A crash after an external destructive action cannot produce a false
  `KeysDestroyed` or false failure result: the operation remains
  `DestroyingKeys` with `DestructionUncertain` until every recovery path has
  durable admitted evidence.
- Permanent ambiguity can be closed only as signed `ResidualUncertainty`, which
  is compactable and acknowledgeable but remains non-abortable and never
  satisfies verified-erasure policy.
- An unsigned or transport-authenticated provider response is not destruction
  proof; every verified path has a canonical signature, attestation, or
  authenticated local-agent envelope bound to the exact request and key.
- Full destruction evidence and its provider/key/timing metadata remain private
  to authorized encrypted views; blind storage and ordinary locked diagnostics
  receive no stable evidence correlator.
- Deleting a local wrapper while its parent key survives does not establish
  erasure unless an independently destroyable local KEK/key slot or complete
  wrapping-epoch rotation makes recovered old wrapper bytes unusable.
- Concurrent history remains verifiable but resolves the erased instance as
  `RedactedBody`; only a separately authorized new encryption instance can
  reintroduce content after redaction.
- Status reports local erasure, controlled copies, remote acknowledgements, and
  uncontrolled residuals independently rather than collapsing them into one
  success boolean.

### v0.123.0 - Reachability, Repack, And Safe Garbage Collection

Goal: maintain production stores without deleting state required for integrity,
recovery, partial clones, or policy.

Deliverables:

- canonical reachability roots;
- user, policy, checkpoint, audit, and operation pins;
- quarantine retention separation;
- redaction tombstone roots and quarantined pre-redaction ciphertext handling;
- partial-clone promises and promised-object roots;
- grace periods and recent-object protection;
- repack and compaction transactions;
- local mixed-content-pack redaction replacement:
  - build and fully verify a replacement pack containing every admitted live
    record but excluding redacted encryption instances;
  - preserve the configured size bucket or add padding so local or exported
    metadata does not reveal the exact removed or surviving record count;
  - atomically update encrypted forward/reverse indexes and pack lineage only
    after the replacement pack is durable and verified;
  - retain the old pack until the replacement transaction commits, then make
    the old redacted records non-addressable before physical deletion;
  - preserve live-record availability when replacement construction,
    verification, sync, or commit fails;
- record-level deletion is used only when the v0.109.0 pack capability proves
  independent removal cannot corrupt or reveal adjacent live records;
- proof, checkpoint, governance, and equivocation-evidence retention rules;
- safe deletion criteria;
- interrupted repack/GC recovery journal;
- concurrent writer, stale index, missing promise, mixed live/redacted pack,
  partial replacement write, concurrent repack, index-before-pack, pack-before-
  index, interrupted repack, and over-eager deletion tests.

Verification:

- `cargo test -p sagnir-store`
- GC reachability property tests;
- repack crash-consistency suite.

Exit criteria:

- Garbage collection deletes only objects proven unreachable from every
  admitted root, pin, promise, retention rule, and in-flight transaction.
- Garbage collection preserves redaction evidence while allowing obsolete
  ciphertext to be removed without changing `RedactedBody` into `Missing`.
- Local mixed-content repack never deletes or makes unavailable a live record
  merely because another record in the same old pack was redacted.
- Failure before the replacement transaction commits leaves the old pack
  available for live records while the redacted record remains undecryptable.
- Integrity and local availability are reported as separate properties.

### v0.124.0 - Remote Storage Receipts And Availability Semantics

Goal: distinguish remote protocol acceptance from evidence that bytes are
durably stored and retrievable.

Deliverables:

- signed storage receipt with remote identity, object/pack commitment, retention
  promise, epoch, and expiry semantics;
- explicit acknowledgement-only response distinct from a storage receipt;
- optional retrieval challenge and availability evidence;
- receipt revocation, expiry, and remote-key compromise handling;
- endpoint-scoped opaque storage-notice supersession of receipts and retrieval
  challenges for the affected ciphertext records;
- receipt lineage across pack relocation without exposing semantic identity or
  a public cross-epoch mapping;
- provider deletion-capability negotiation for record-level deletion versus
  whole-pack replacement;
- remote mixed-pack replacement sequence:
  - build and verify the privacy-padded replacement pack locally;
  - upload it without deleting the old pack;
  - obtain every policy-required signed storage receipt for the replacement;
  - atomically commit encrypted index and receipt lineage to the replacement;
  - only then issue endpoint-scoped deletion notices for the old pack or
    independently deletable redacted records;
  - record each provider acknowledgement, refusal, timeout, or unreachable
    result separately;
- crash-safe resume after partial upload, lost replacement receipt, committed
  receipt without local lineage update, notice loss, and provider partition;
- provider-specific opaque lineage and padding that does not disclose which old
  records survived, were removed, or moved to which new position;
- authorized clients refuse to request or accept a receipt for a privately
  tombstoned encryption instance, while blind stores refuse selectors
  superseded by their admitted opaque storage-notice sequence;
- no claim that one receipt proves indefinite availability;
- forged, replayed, expired, partial-storage, unavailable-remote, mixed
  live/redacted pack, partial upload, lost receipt, notice-before-receipt,
  concurrent repack, provider partition, record-deletion downgrade, and
  multi-provider replacement tests.

Verification:

- `cargo test -p sagnir-sync`
- storage receipt vector and integration suite.

Exit criteria:

- Sagnir never reports remote acceptance as durable storage unless the remote
  returns an admitted signed receipt with defined semantics.
- Availability claims state their time, scope, and witness assumptions.
- A receipt predating redaction is historical evidence only and cannot make a
  redacted body available again.
- Remote deletion never begins before required replacement availability is
  proven, unless policy explicitly accepts reduced availability for the
  affected live records.
- This milestone turns the v0.122.0 storage-notice contract into executable
  receipt supersession and remote deletion acknowledgement.

### v0.125.0 - Availability Replication And Repair

Goal: maintain configured availability rather than merely recording storage
receipts.

Deliverables:

- replication policy by object, pack, compartment, archive, and protected world;
- independent provider and administrative-domain diversity requirements;
- periodic authenticated retrieval challenges;
- receipt freshness and challenge scheduling;
- missing or degraded replica detection;
- repair planning and resumable re-replication;
- tombstone-first repair planning that excludes redacted encryption instances,
  stale receipts, quarantined ciphertext, and archives behind the redaction
  frontier;
- mixed-pack replacement and repair coordinate across providers so at least the
  configured number and diversity of replacement receipts exist before old
  packs are deleted;
- repair resumes incomplete v0.124.0 replacements, but never chooses an old
  mixed pack as a source for a tombstoned encryption instance;
- optional evaluated erasure-coding profile with shard commitment and repair
  semantics;
- provider collusion, correlated failure, stale receipt, partial shard,
  unavailable source, repair interruption, offline stale provider, and
  redacted-instance resurrection tests.

Verification:

- `cargo test -p sagnir-sync`
- deterministic availability simulation;
- repair integration suite.

Exit criteria:

- Sagnir can detect when configured availability falls below policy and repair
  it from an admitted surviving source.
- Availability repair never restores an encryption instance superseded by an
  admitted redaction tombstone.
- Multi-provider replacement preserves configured availability for surviving
  live records through partial failure and partition.
- This milestone turns the v0.122.0 repair non-resurrection contract into
  executable repair behavior.
- Provider count alone cannot satisfy diversity policy when providers share one
  administrative failure domain.

## Phase 11: Hardening And Portability

### v0.126.0 - Verifiable Archive Pack Concept

Goal: keep a future path for disk-space relief without making deletion part of
the early trust model.

Deliverables:

- `.saga/archival/` planning document;
- compressed archive pack concept;
- archive manifest concept;
- archive receipt and root commitment concept;
- rehydrate/restore concept;
- restricted restore-admission concept shared by archives, ordinary filesystem
  backups, VM snapshots, recovery kits, and air-gapped replicas;
- checkpoint and witness freshness preflight before decrypting or
  materializing restored protected state;
- redaction-index and tombstone overlay concept preventing archive rehydration
  from restoring superseded encryption instances;
- mixed-content archive replacement and receipt-supersession contract matching
  the v0.109.0 pack capability and v0.124.0 remote replacement ordering;
- stale archive bodies are quarantined and reported, not materialized or used
  as repair sources;
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
- A later archive implementation must apply admitted redaction tombstones before
  body selection, restoration, or receipt validation.
- Restore documentation must state that a purely isolated snapshot cannot prove
  it has the latest redaction frontier.
- This milestone defines the archive integration contract only; it does not
  claim an executable archival deletion or restoration guarantee before
  v1.6.0.

### v0.127.0 - Malicious Corpus

Goal: make hostile input testing part of normal development.

Deliverables:

- canonical codec corpus;
- cumulative decode-budget, atomic-encoder failure, and nested-work corpus;
- object corpus;
- body-derived typed-edge, duplicate-edge, authority-cycle, dependency-SCC, and
  optimized/reference disagreement corpus;
- WAL corpus;
- pack corpus;
- bundle corpus;
- encrypted envelope corpus;
- private locator search node and proof, quota record and carry-forward,
  escrow-right allocation/transfer/spend/conflict, logical-node commitment,
  multi-instance composite key, randomized node envelope, endpoint ciphertext
  placement, creation-operation reservation, projection rebuild/delta replay
  certificate, projection-witness statement,
  compartment-root/realm-manifest/partial-access proof, reverse-index,
  duplicate-equivalence, equivalence-conflict, Merkle-chunked recursive
  compartment-move manifest/typed proof/conflict/resume state, target-only
  attestation, neutral-domain object/key metadata, redacted placeholder,
  destruction
  intent/private evidence/minimal disclosure/terminal disposition, private
  tombstone, opaque storage-notice, and pre-erasure receipt corpus;
- proof and sync-message corpus;
- deterministic fact rule/query-plan, snapshot cursor, immutable-index offset,
  exact cryptographic suite/hybrid transcript, opaque bundle outer/inner
  manifest, and blind-claim corpus;
- decompression and delta-chain bomb corpus;
- fork-bomb and causal-fanout corpus;
- regression tests for every accepted corpus case.

Verification:

- `cargo test --workspace`

Exit criteria:

- Known malicious bytes stay rejected across releases.

### v0.128.0 - Expanded Fuzzing

Goal: expand fuzz and model testing beyond the parser scaffolds added earlier.

Deliverables:

- fuzz target workspace;
- codec fuzz target;
- cumulative decode-budget and atomic-encoder target set;
- every canonical object-body fuzz target;
- body-derived typed graph and SCC target set;
- WAL and recovery-state fuzz targets;
- pack and encrypted-envelope fuzz targets;
- private locator search node/proof, quota record/carry-forward, and
  escrow-right allocation/transfer/spend/conflict, logical/encrypted/storage
  index identity, multi-instance key, creation-operation reservation,
  projection rebuild/delta replay certificate, projection-witness statement,
  endpoint placement,
  compartment-root/realm-manifest/partial-access proof, and reverse-index fuzz
  targets;
- duplicate-equivalence, equivalence-conflict, Merkle-chunked recursive
  compartment-move typed proof/resume/conflict, target-only attestation,
  neutral-domain metadata, redacted placeholder, destruction intent/private
  evidence/minimal disclosure/terminal disposition, private tombstone, opaque
  storage-notice, and pre-erasure receipt fuzz targets;
- bundle parser fuzz target;
- opaque outer/encrypted inner manifest, two-phase admission state, blind-claim,
  and cumulative bundle-work target set;
- untrusted/bounded-quarantine/admitted-authority candidate transition,
  aggregate quarantine-counter, partial-capture cleanup, `ResourceLimit`, and
  bounded abuse-receipt target set;
- `QuarantineReservationLease` framing/state, checked counter debit, idle and
  absolute expiry, progress renewal, reconnect/resume inheritance, atomic
  conversion/release, startup-orphan reconciliation, process `ClockEpoch`,
  complete/partial restart classification, and encrypted opaque metadata target
  set;
- fact rule stratifier, fixpoint/query-plan, pagination cursor, and immutable
  index offset target set;
- exact cryptographic suite and hybrid transcript target set;
- proof and sync-message fuzz targets;
- bounded decompression and delta-chain fuzz targets;
- documentation for running fuzz targets.

Verification:

- parser unit tests;
- v0.12.2 CI smoke, pentest/release, and scheduled campaign profiles.

Exit criteria:

- New parsers inherit executable fuzz coverage at first admission; this
  milestone broadens duration, sanitizer coverage, cross-format composition,
  and corpus scale rather than introducing fuzzing for the first time.

### v0.129.0 - Full-System Formal Model Composition

Goal: compose and re-check the model-first subsystem work against the complete
pre-1.0 design.

Deliverables:

- composed WAL recovery, alias CAS, merge/promotion, private duplicate identity,
  key rotation, redaction projection, restricted restore, checkpoint, GC, and
  partition models;
- composition of cumulative decoder/work budgets, body-derived graph-class
  admission, durability-profile publication, deterministic fact stratification/
  queries, opaque secret sessions, and two-phase bundle quarantine/admission,
  including untrusted/bounded-quarantine/admitted-authority candidate separation,
  aggregate preflight capacity, sender-held refusal, bounded reservation leases,
  non-resettable reconnect/resume lineage, work-before-use accounting, fair
  scheduling, process-epoch-only monotonic comparison, encrypted operational
  metadata, atomic candidate-charge conversion, and atomic partial cleanup;
- composition of the history-independent map algorithm/pages with authority
  active/covered-fence/exception/archive roots, a permanent low-sequence
  exception plus later archival, exact replay refusal, checkpoint rollback
  detection, archive unavailability, exception resolution, and cutover carry;
- composition distinguishes inert v0.23.6 fence transition machinery from
  v0.52.0 production activation and proves checkpoint, retention, and all-active-
  replica stability evidence cannot be omitted or replaced by local authority;
- first-fence-activation composition atomically raises the durable feature and
  minimum authoritative verifier/writer floor across format metadata, WAL state,
  and signed checkpoint state; rollback, stripping, or a pre-v0.52 writer cannot
  reopen or claim full verification of the activated authority root;
- encrypted-genesis composition for minimal-plaintext, externally provisioned,
  and passphrase-derived bootstrap profiles, proving first-log key availability,
  no plaintext fallback, normal-key replacement, and crash recovery without a
  reservation/key provisioning cycle while making no modeled claim that local
  attempt limits constrain offline guesses against copied passphrase ciphertext;
- secret-input composition covers interactive terminal state, owned descriptors,
  credential leases, generated-secret destinations, cancellation/unwind, and the
  prohibition on argv/environment/config/diagnostic secret channels, with exact
  UTF-8/UTF-16 conversion, Enter framing, descriptor framing, generated-secret
  encoding, exact HMAC-SHA3-256 possession transcript, independently protected
  provider-held/volatile/platform-sealed pending state, exact resume or pre-
  authority abort, and non-convertible local-KDF/provider-handle classes;
- transparency composition keeps history-independent current-state map proofs
  separate from append-log consistency proofs and binds both roots in one signed
  checkpoint without cross-structure substitution; every key transition moves
  the map, append log, and dual-root state head through one expected-head WAL CAS
  or leaves the complete prior state authoritative, while v0.23.4 reservations,
  provider ambiguity reconciliation, and superseded signed evidence prevent a
  lost signer response or competing CAS from duplicating signing authority;
  bounded in-flight/quota state, terminal `SupersededAfterSignature`, prepared-
  data leases/GC, and v0.23.6/v0.52 archival prevent contention from creating
  unbounded active evidence without discarding ambiguous or dispute material;
  valid signatures and claimed identities cannot force durable quarantine or
  multiply aggregate receiver capacity before authority admission;
- duplicate-equivalence representative CAS, conflict-head preservation,
  anti-grinding selection, replica/actor/device quota continuity, and persistent
  authenticated index union/split models;
- escrowed bounded-counter model proving disjoint offline quota-right
  consumption, causal transfer, merge-time double-spend/overdraw quarantine,
  signed surrender, acknowledged final spent roots, retirement cutoffs,
  uncertain-right burning, non-retroactive ratification, dependent
  re-evaluation, and migration without counter-only assumptions;
- logarithmic locator inclusion/absence proof, immutable path-copy update,
  history-independent normalization across operation permutations, bounded
  amplification, and locator-rotation quota carry-forward invariants;
- multi-instance index model proving exact forward/reverse lookup for every
  `(semantic commitment, encryption instance)` pair, no aliasing or omission,
  and non-interchangeable duplicate-identity/instance-fanout rights;
- encryption-instance identity model proving exact context binding, collision
  refusal, stability across projection changes, and mandatory replacement when
  erasure identity changes;
- creation-operation reservation model proving durable preallocation,
  monotonic/random identity, one exact idempotent consumption, authenticated
  cancellation, crash recovery, clone/rollback conflict, and permanent
  non-reuse;
- private-index identity model separating deterministic keyed logical roots,
  randomized encrypted node envelopes, and public ciphertext storage IDs, with
  convergence required only for the logical root;
- private-index authority model separating commitment-key possession from
  signed checkpointed manifest admission, plus commitment-key rotation and
  historical-root verification;
- stable-leaf/placement model proving re-encryption, repacking, receipt renewal,
  and relocation cannot change the logical root;
- compartment-root composition model proving scoped inclusion/continuity through
  the opaque count-hiding realm manifest without disclosure of other
  compartments;
- opaque compartment-handle model covering collision, signed rotation,
  encrypted old/new mapping, and unavoidable previously observed correlation;
- semantic-ledger projection model proving complete forward/reverse rebuild and
  delta transitions, malicious publisher omission/invention refusal, witness
  equivocation, and explicit partial-access trust;
- projection replay-certificate model proving canonical verification semantics,
  hard delta-chain/bytes/work limits, mandatory rebuild cadence, transcript
  retention, and refusal of unavailable ledger chunks;
- projection-witness governance model proving full-replay versus
  evidence-validation assurance, principal/domain independence, nominal-Sybil
  refusal, key lifecycle, threshold unavailability, and equivocation handling;
- endpoint-placement model proving canonical logical convergence with divergent
  replica/device/endpoint projections and no arrival-order overwrite;
- authoritative quota-expiry model covering pre-expiry creation with late
  delivery, unobserved expiry, concurrent expiry/spend/transfer, timestamp-
  authority conflict, and local clock rollback or skew;
- authoritative time/revocation substrate model covering monotonic sequences,
  append-only consistency, key rotation/revocation, authority split view,
  quorum/diversity, request privacy, and offline freshness;
- external key-destruction model covering durable intent before dispatch,
  provider success before local journal commit, ambiguous timeout, idempotent
  query, authenticated destruction-evidence admission, provider revocation or
  compromise, confirmed retry, `DestructionUncertain`, terminal
  `ResidualUncertainty`, later evidence advancement, and all-path confirmation;
- local wrapper-erasure model covering recoverable deleted records, independent
  per-erasure-unit key slots, parent wrapping-epoch rotation, complete surviving
  DEK rewrap, old-epoch destruction, and residual media copies;
- recursive cross-compartment graph-translation model with shared descendants,
  source/target/policy compare-and-swap, target-reachability isolation, and
  explicit multi-head move conflicts;
- typed leaf-equality and container-structural-isomorphism model, exact
  reference mapping, transformed-metadata rules, sparse/promised fetch
  prerequisites, redacted/unavailable refusal, separately typed
  compartment-neutral commitment domains, and neutral-only reference closure;
- translation-visibility model proving complete bridges reach only
  cross-authorized/audit actors, target attestations disclose no source
  identity or shape, source-only actors receive no target identity, and blind
  stores receive neither side;
- target-attestation model proving claim limits, issuer/revocation/expiry/replay
  binding, and refusal to infer hidden-source equality without bridge access;
- neutral-object lifecycle model covering dedicated key purposes, recipients,
  retention, erasure, rekey, recovery, and intentional cross-compartment
  linkability;
- redacted-placeholder model proving non-content semantics, audit-only
  provenance, inability to satisfy proof/availability/completeness/repair, and
  stale-body non-replacement;
- Merkle-chunked translation-manifest construction, bounded streaming
  verification, temporary GC pins, cancellation, crash resume, and atomic final
  root commit model;
- concurrent and causally later redaction-reference model proving that event
  ordering cannot resurrect an erased encryption instance;
- compatibility review between subsystem assumptions;
- checked invariants for atomicity, no lost heads, no locator-based identity
  collapse, no silently discarded admitted bucket entry, no last-writer-wins or
  grindable duplicate representative, no offline aggregate quota overdraw
  admitted, no unsafe retired-right redistribution, no retroactive quota
  validity, no history-dependent logical root, no mutable-placement root drift,
  no instance alias/omission or quota-class confusion, no compartment-proof
  disclosure, no creation-reservation reuse, no unbounded projection
  verification/storage amplification, no witness-threshold/Sybil bypass, no
  endpoint-placement overwrite, no clock-derived quota validity,
  no commitment-key authority escalation, no logical/ciphertext index identity
  confusion, no private quota or translation relationship leakage, no target
  attestation overclaim, no neutral lifecycle bypass, no placeholder proof or
  availability satisfaction, no false key-destruction success, no private
  destruction-evidence leakage, no wrapper-deletion erasure claim, no false
  container byte-equality proof, no source-compartment reference in a translated
  target graph, no sparse or redacted descendant bypass, no partial manifest
  authority, no stale move overwrite or newer-source removal, no redacted-body
  resurrection, no stale restore admission, no stale admission, no authority
  from delivered-but-unconfirmed generated credentials, no silent pending-secret
  regeneration, no self/circular pending-secret protection, no possession-proof
  replay, no duplicate signer operation after ambiguity, no signed-transition
  rebase, no unbounded active superseded/prepared state, no ambiguity deletion by
  quota/lease expiry, no unbounded quarantine through signed/Sybil/session/bundle
  floods, no slow/trickle sender retaining a reservation past its hard lifetime,
  no reconnect/resume deadline reset, no work before debit, no cross-epoch
  monotonic comparison, no orphan reservation after recovery, no lease identity/
  activity leakage beyond its selected privacy profile, no partial durable
  candidate after resource refusal, no abuse digest or `ResourceLimit` as
  authority evidence, no arrival-order authority selection, and eventual
  convergence under documented assumptions;
- model execution instructions and CI smoke bounds.
- model-run manifests inherit v0.115.1 exact bounds, assumptions, property
  classes, reductions, coverage counters, resources, seeds/configuration, and
  completion-status requirements.

Verification:

- bounded model-check command;
- model invariant review.

Exit criteria:

- Counterexamples for stale CAS, conflicting duplicate representatives, bucket
  exhaustion, linear-proof amplification, quota-right double-spend, offline
  overdraw, unsafe rights reclamation, retroactive ratification,
  operation-history root divergence, instance alias/omission, quota-class
  substitution, creation-reservation reuse, projection-chain amplification,
  witness Sybil/equivocation/unavailability bypass, compartment-root disclosure,
  endpoint-placement overwrite,
  authoritative-expiry failure, placement-root coupling, unauthorized manifest
  publication, commitment-key rotation, quota/translation metadata leakage,
  target-attestation overclaim, neutral-lifecycle bypass, placeholder
  availability/proof misuse, logical/ciphertext root confusion, partial or
  forged destruction evidence, private evidence leakage, recoverable deleted
  wrappers, permanently uncertain destruction, false typed translation,
  neutral-domain confusion, sparse/redacted descendant bypass, manifest resume
  substitution, partial final commit, recursive move leakage, stale
  compartment-move CAS, stale redaction lineage, unconfirmed generated-secret
  publication, accidental credential regeneration, circular staging-key use,
  possession-proof replay, ambiguous signer duplication, signed-transition
  rebase, stale-head signing flood, superseded/prepared-state amplification,
  unsafe ambiguous archival, quarantine-capacity multiplication, reconnect/
  resubmit retention flood, partial quarantine admission, resource-refusal
  authority confusion, arrival-order selection, lost divergence, and replay are
  represented in executable models rather than prose alone.
- This release is full-system assurance, not the first time foundational
  formats are modeled.

### v0.130.0 - Crash And Concurrency Assurance

Goal: exercise local mutation behavior at every admitted interruption and race
boundary.

Deliverables:

- crash-consistency fault injection at every write, rename, file sync, and
  directory sync;
- v0.32.1 parent-root/nested-parent publication, durability-profile detection,
  short-write/`EINTR`, process-kill, checkpoint, and WAL-segment-retirement
  matrix replay;
- state-machine property tests for recovery;
- loom or equivalent tests for writers, proof caches, and alias updates;
- two-phase bundle quarantine/decrypt/typed-ingest/WAL-publication crash and
  cancellation tests proving no skipped trust stage;
- candidate preflight/quarantine crashes across aggregate quota reservation,
  v0.111.1 lease creation/progress/idle-expiry/absolute-expiry, integrity-bound
  capture, partial write, item/byte/signature/page/work counter debit, atomic
  lease-to-candidate charge conversion, completion/expiry races, restart orphan
  reconciliation, v0.111.2 process-epoch change, complete-versus-partial restart
  classification, encrypted-metadata key unavailability/replay, `ResourceLimit`,
  abuse-receipt rotation, cleanup, re-admission, and final authority publication
  prove all-or-nothing durable quarantine and no resource-refusal authority
  evidence;
- secret-handle/provider-session cancellation, panic, agent-disconnect, cleanup,
  and process-boundary tests proving no partial authorization result;
- generated-credential receiver-close, partial-delivery, possession-confirmation,
  provider retrieval, provider-held/volatile/platform-sealed pending state,
  independent-platform-key availability, exact reacquisition, staging rollback/
  substitution, post-consumption cleanup, crash resume, and pre-authority abort
  boundaries with circular protection and silent regeneration prohibited;
- transparency transition/checkpoint signing crashes before and after durable
  reservation, provider admission, result persistence, response loss,
  reconciliation, competing head CAS, superseded-evidence retention, and exact
  retry, plus prepared-data lease expiry/rebuild/GC and stable archive movement,
  proving no second signer operation, transcript rebase, evidence loss, or active-
  state amplification occurs;
- atomic forward/reverse private-index update and rebuild interruption tests;
- multi-instance forward/reverse update tests proving no instance is lost,
  aliased, or charged to the wrong quota class;
- creation-operation reservation crash tests across allocation, file/database
  durability, event-chain commit, consume, cancel, restart, clone/rollback,
  expiration, and concurrent consume/cancel;
- projection replay crash tests across delta-certificate durability,
  manifest publication, mandatory rebuild trigger, chunk availability, witness
  replay/signature, threshold commit, and equivocation recording;
- crash tests across private logical node commit, randomized envelope write,
  ciphertext storage-ID assignment, placement-root commit, and logical-root
  publication;
- crash tests for signed logical-root manifest publication,
  index-commitment-key rotation, history-independent normalization checkpoints,
  and placement-only re-encryption, repack, receipt renewal, and relocation;
- crash tests for compartment-root/realm-manifest publication, partial-access
  proof generation, and independent endpoint-placement projection updates;
- escrow-right allocation, transfer, spend, double-spend conflict, quota-
  conflict quarantine, locator migration, and governed resolution transaction
  tests;
- crash tests between private tombstone commit, pre-erasure receipt commit,
  durable destruction intent, provider dispatch, provider success, local
  confirmation commit, wrapper destruction, storage-notice emission, receipt
  supersession, and wrapping-epoch rotation;
- KMS, HSM, filesystem, software-wrapper, escrow, and recovery-share fixtures
  for idempotent destruction tokens, post-crash outcome query, confirmed
  non-destruction retry, unavailable query, contradictory provider response,
  and permanently uncertain result;
- canonical destruction-evidence crash fixtures for signed provider responses,
  hardware attestations, authenticated local-agent statements, evidence before
  journal commit, journal before evidence durability, provider-key revocation,
  retirement, compromise, and replay;
- local filesystem journal, CoW snapshot, volume snapshot, retained block image,
  recovered-deleted-wrapper, independent local key-slot destruction, complete
  parent-epoch rewrap, partial rewrap, and old-epoch destruction fixtures;
- terminal `ResidualUncertainty` closure, journal compaction, alert
  acknowledgement, restart, and later-valid-evidence advancement tests;
- crash tests for every durable erasure state transition and for attempts to
  roll back from or recreate keys after `DestroyingKeys` or `KeysDestroyed`;
- crash and concurrency tests for recursive cross-compartment translation
  Merkle-manifest chunk creation, typed descendant proof, promised-body fetch,
  temporary pin creation, durable resume checkpoint, cancellation cleanup,
  target graph commit, encrypted index commit, atomic final manifest/root commit,
  source logical removal, source/target/policy CAS, conflict-head creation,
  resolution, and optional later redaction;
- crash tests across target-only attestation issue/revoke, neutral-domain rekey
  and recovery, redacted-placeholder creation, audit-provenance commit, stale
  body arrival, and authorized reintroduction;
- concurrent duplicate-equivalence compare-and-swap and conflict-resolution
  transaction tests;
- mixed-pack replacement crash tests across pack durability, receipt
  acquisition, encrypted-index commit, notice emission, and old-pack deletion;
- process and thread race tests;
- stale-handle and namespace-replacement fixtures;
- deterministic failure reproduction seeds.

Verification:

- crash fault-injection suite;
- concurrency model test suite.

Exit criteria:

- No injected interruption produces a trusted alias without complete immutable
  bodies or a cache result from the wrong generation.
- No interruption leaves a trusted one-sided private index, reports erasure
  before required evidence and key transitions commit, or loses an admitted
  private semantic tombstone.
- No interruption publishes a logical B+ tree root whose canonical nodes lack
  authenticated encrypted envelopes/placements required by the transaction, or
  treats randomized ciphertext identity as the logical root.
- No interruption makes a commitment-key holder authoritative, publishes an
  unsigned logical root, changes the logical root for a placement-only mutation,
  or leaves old/new commitment epochs ambiguously admitted.
- No interruption loses one policy-separated encryption instance, reveals an
  unrelated compartment through a partial proof, lets one endpoint replace
  another's placement, or upgrades a redacted placeholder into content.
- No interruption reuses a creation reservation, publishes a manifest without
  complete bounded replay evidence, skips a required rebuild, or counts a stale,
  unavailable, compromised, or non-independent witness toward threshold.
- No interruption leaves a durable quarantined candidate without its complete
  aggregate quota charge, leaks a charge without the complete candidate, carries
  partial bytes across restart, or upgrades `ResourceLimit`/abuse telemetry into
  semantic or authority evidence.
- Recovery is forward-only after `DestroyingKeys`; uncertain provider outcomes
  remain explicit and cannot be converted into success or failure by restart.
- Permanently uncertain operations can compact into signed
  `ResidualUncertainty` without claiming erasure and can later advance only
  through valid admitted evidence.
- Cross-compartment move recovery preserves the source until the new target
  graph, every translated descendant, encryption instance, and index is
  durable; stale CAS creates conflict instead of overwriting current state.
- Translation resume cannot substitute source/target/policy inputs, lose
  temporary GC pins, or expose a partial Merkle manifest as authoritative.
- Incomplete remote or controlled-copy work remains resumable without weakening
  local erasure.

### v0.131.0 - Partition And Adversarial Network Tests

Goal: validate convergence and refusal behavior under hostile distributed
ordering.

Deliverables:

- reorder, replay, duplication, delay, truncation, and disconnect tests;
- concurrent partitioned world advancement;
- concurrent equal-plaintext private creation and duplicate-equivalence
  reconciliation;
- conflicting partitioned representative selections, stale expected-root CAS,
  multi-head propagation, and authorized multi-parent resolution;
- concurrent authorized replicas preparing/signing from one transparency head,
  stale-head signing floods, actor/device/realm pending/ambiguous/superseded quota
  exhaustion, offline over-quota reconciliation, permanently ambiguous provider
  operations, prepared-data lease pressure, stable archive propagation, and
  archive recovery without signature/transcript/dispute-evidence loss;
- unlimited validly signed stale untrusted candidates, many claimed identities
  sharing one store/realm quarantine ceiling, peer/bundle/session fanout,
  reconnect/resubmit floods, slowloris and one-byte-progress transfers, resume-
  token/session replacement deadline-reset attempts, parallel reservation
  starvation, process replacement and suspend/resume clock-epoch tests, metadata-
  key unavailability and identifier/timing leakage probes, sender-held refusals,
  bounded abuse-receipt rotation, expiry and restart accounting, partial-
  candidate cleanup, and attempts to reinterpret `ResourceLimit` as signature/
  policy/authority evidence;
- cloned/rolled-back pending-credential staging, provider/platform-key
  unavailability, copied staging outside its declared platform recovery context,
  cross-store/receiver possession-response replay, and concurrent delivery
  attempts prove no partition can substitute protection, confirm the wrong
  receiver, publish authority, or generate a replacement credential;
- authorized-replica duplicate amplification, per-replica quota replay,
  authenticated locator-index reconciliation, actor/device aggregate quota
  evasion, locator-rotation carry-forward, new-incarnation evasion, concurrent
  search-tree union/split, multi-instance fanout and quota-class separation,
  creation-operation reservation replay/cancel/consume races, projection
  delta-chain limit and rebuild enforcement, witness threshold/Sybil/
  equivocation states, compartment-root proof reconciliation, divergent
  endpoint placement, and local operation-budget interruption;
- partitioned escrow-right consumption, causal transfer races, double-spent
  rights, aggregate overdraw quarantine, signed surrender, final spent-root
  acknowledgement, retirement cutoff, unseen offline spend, uncertain-right
  burning, non-retroactive governance ratification, and dependent-transition
  quarantine/re-evaluation;
- quota-expiry partitions covering provably pre-expiry creation with late
  delivery, unseen expiry transition, concurrent spend/transfer and expiry,
  timestamp-authority disagreement, clock rollback, and restored snapshots;
- logarithmic inclusion and absence proof validation under adversarial fanout,
  range, height, operation-history permutations, and amplification declarations;
- concurrent cross-compartment moves with changed source frontier, occupied
  target, changed target policy, overlapping recursive graphs, missing promised
  descendant, unavailable opening, redacted descendant, opaque-boundary
  admission/refusal, neutral-domain confusion, one-sided translation
  disclosure, target-attestation expiry/revocation/replay and overclaim,
  neutral recipient/rekey/recovery partitions, intentional-correlation
  disclosure, redacted-placeholder propagation, stale-body replacement,
  repeated-correlation attempts, chunked-manifest resume, and multi-head
  resolution;
- offline peer and blind store returning with pre-redaction ciphertext,
  receipts, wrappers, or pack positions;
- valid concurrent events referencing a redacted instance, causally later stale
  references, merge-order variation, and explicitly authorized reintroduction
  as a new encryption instance;
- endpoint-specific storage-notice delay, replay, reordering, and substitution;
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
- Partitions preserve every signed duplicate semantic identity and propagate
  redaction before stale body repair or materialization.
- Partitions never choose conflicting duplicate representatives by arrival
  order or attacker-grindable bytes, and quota enforcement cannot silently
  discard already admitted history.
- Replica incarnation, actor/device identity, and locator epoch changes cannot
  reset duplicate-creation quotas.
- Partition merge never converts aggregate quota overdraw or a double-spent
  escrow right into authoritative state; dependent candidates remain explicit
  quarantine until governed resolution.
- Missing-replica retirement cannot recycle possibly spent rights, and later
  ratification creates new admission history instead of rewriting the original
  causal decision.
- Peers deriving the same canonical locator entry set through different
  operation histories converge on one logical root; placement-only differences
  and commitment-key possession do not create alternate authority.
- Multiple encryption instances for one semantic commitment remain distinct,
  endpoint placement remains local, and partial-compartment peers converge
  without learning unrelated compartment state.
- Creation reservations remain single-use across partitions, projection
  certificates remain bounded and rebuildable, and witness threshold cannot be
  satisfied by replay, one administrative operator, or conflicting statements.
- Delivery order, offline observation, or local clock state cannot extend quota
  rights or turn an ambiguous expiry race into authoritative admission.
- Concurrent compartment moves preserve every contender and never leave a
  target graph that reaches source-compartment identities.
- One-sided recipients and blind stores do not learn the complete
  source-to-target relationship, and neutral boundaries cannot smuggle
  compartment-bound references.
- Target-only recipients see an authority assertion rather than a false
  hidden-source proof; neutral lifecycle and redacted-placeholder state remain
  enforceable across partitions.
- Concurrent historical references remain verifiable as `RedactedBody`, while
  stale ordering and repair cannot resurrect the erased encryption instance.
- Hostile signed-candidate, identity, bundle, session, and reconnect floods stay
  within aggregate receiver quarantine/work ceilings; refused input remains the
  sender's responsibility, leases cannot outlive their original absolute
  lifetime or process epoch, protected lease metadata stays within its declared
  leakage profile, fair capacity remains available to other peers, and arrival
  order affects backpressure only.
- Hostile peers cannot force unbounded recursive expansion or partial trust.

### v0.132.0 - Differential Vectors And Performance Budgets

Goal: prove interoperability and set measurable scale expectations before 1.0.

Deliverables:

- independent canonical-codec reference implementation;
- differential canonical bytes and object-ID tests;
- v0.17.1 algorithm-admission proof/model plus v0.17.2 page/root/proof vectors
  and million-entry bounded lookup, update, rebuild, malformed-page, and unique-
  representation benchmarks;
- authenticated subtree range/entry-count/page-count/height summary vectors,
  checked summary-overflow tests, logarithmic point-proof benchmarks, and output-
  sensitive range/page-set proof budgets measured against emitted results;
- cumulative decode-budget and atomic-encoder benchmarks across nested objects,
  packs, bundles, proofs, WAL, and encrypted envelopes;
- minimal authority transaction-substrate model/vectors and crash benchmarks
  covering reservation/result durability, provider/store reconciliation, torn
  records, races, compaction, parent sync, repeated recovery, and later in-place
  record-kind extension;
- independent `sagnir-authority-sha3-256-v1` frame, transaction, logical-state,
  and physical-checkpoint vectors, including genesis/checkpoint anchoring,
  non-circular signing frontiers, physical-compaction logical-root preservation,
  cross-domain substitution, available-byte mock-collision refusal, and
  declared hash-suite collision assumptions;
- authority active-map, terminal-fence, sparse-exception, and epoch-archive
  differential vectors plus million-operation compaction/replay/rebuild budgets
  proving a permanent early exception does not pin later terminal entries and
  active memory/cutover do not scale with all historical operations;
- v0.52.0 first-activation mixed-version fixtures prove the format, WAL,
  checkpoint, and current-root feature floors move atomically; pre-v0.52 tools,
  rolled-back format metadata, stripped activation records, and stale writers
  cannot obtain writable or fully verified current-state status;
- parallel budget-lease stress and schedule benchmarks proving child
  reservations cannot mint capacity and cancellation/panic accounting remains
  deterministic under maximum admitted parallelism;
- cross-host sequential/parallel work-accounting vectors proving CPU speed,
  worker count, scheduling, unused-lease return timing, and local deadlines do
  not change authoritative completion, truncation, counters, or roots;
- protocol-fixed bootstrap versus signed realm-selected work-cost table,
  local-ceiling refusal, operational cancellation, activation/downgrade,
  retirement, historical replay, replacement, unavailable-old-implementation,
  and old/new differential vectors;
- streaming-encoder failure-boundary fixtures proving incomplete sink output
  never becomes hashable, signable, publishable, or authoritative;
- optimized/reference body-derived graph benchmarks covering sorted lookup,
  typed edge schemas, duplicate refusal, authority DAGs, dependency SCCs, and
  adversarial sparse/dense distributions;
- deterministic fact evaluator and snapshot-query benchmarks covering
  stratification, fixpoint work, provenance alternatives, SCC impact, cursor
  continuation, and immutable validated index segments;
- non-circular obligation template/instance identity vectors with preallocated
  issuance-operation IDs plus nested evidence allocation/reuse, independence,
  canonical matching, and adversarial double-count benchmarks;
- cryptographic known-answer and malformed-vector suites;
- CRC-32C hardware/software differential and WAL known-answer vectors plus
  encrypted-WAL silent-clone/snapshot nonce-safety benchmarks;
- provider-independent suite interoperability benchmarks with provider
  assurance and realm admission changed independently;
- provider-assurance issuer/root/category, freshness, revocation, supersession,
  self-claim, build, backend, and platform differential fixtures;
- independently developed reference projection evaluator built from the
  normative projection specification and canonical vectors without importing,
  linking, generating from, or wrapping the production evaluator implementation;
- differential full-rebuild and every admitted delta-transition test across
  production and reference evaluators, including randomized operation histories,
  canonical edge cases, malformed inputs, resource-bound edges, projection
  version changes, and deliberately seeded evaluator defects;
- at least one high-assurance full-replay witness implementation uses the
  independent evaluator and independently acquires committed ledger chunks;
  running another process or administrative domain around the production
  evaluator does not satisfy implementation diversity;
- adversarial evaluator-disagreement suite proving quarantine, immutable
  disagreement evidence, no majority/arrival-order resolution, and refusal by
  protected worlds and partial-access recipients;
- evaluator-bug remediation and projection-version migration fixtures proving
  signed defect scope, original evidence preservation, corrected new-root
  admission, affected-manifest quarantine/release, old/new implementation
  interoperability, and historical verification;
- provider side-channel assurance fixtures covering declared constant-time
  operations, secret-dependent control/data-flow review findings,
  invalid-input response shapes, timing distributions, hardware acceleration
  versus software fallback, secret-copy lifetime, and zeroization limitations;
- whole-record zero-plaintext-on-bad-tag and exact per-suite chunk nonce/subkey,
  exhaustion, retry, authenticated-release/final-completeness vectors, plus
  sealed bootstrap/unlock/authoritative/recovery capability, operation-ID
  reservation, IPC/crash/idempotency/ambiguous-result fixtures;
- bootstrap/genesis cross-domain stage and crash vectors covering orphan keys,
  lost provider results/signatures, pending non-authoritative genesis, provider
  ambiguity, exact-key usability confirmation, and final CLI success;
- encrypted-genesis profile vectors and crash benchmarks covering bounded
  plaintext cutover, external handle/attestation admission, passphrase header/
  KDF derivation, direct-encryption refusal, normal WAL-key replacement, and
  bootstrap-key retirement;
- passphrase-bootstrap KDF cost/parameter benchmarks, offline-guess verifier
  fixture, permanent-header/ciphertext disclosure statement, assurance-label
  snapshots, reuse warnings, and high-assurance policy refusal tests;
- v0.98.2 PTY/console, owned-descriptor, credential-provider, generated-secret,
  forbidden-channel, cancellation/unwind, terminal-restore, and sanitization
  vectors with stable redacted output snapshots, exact Unix UTF-8/Windows UTF-16
  conversion, LF/CRLF framing, descriptor length/EOF behavior, canonical
  generated-secret encoding, cross-source byte equivalence, generated delivery/
  exact HMAC-SHA3-256 possession transcript/response vectors, provider-held/
  volatile/platform-sealed staging vectors, circular-key and staging-substitution
  refusal, exact pending-state resume/cleanup, regeneration refusal, and source/
  profile incompatibility before secret acquisition;
- transparency current-map and append-event-log independent vectors, atomic
  old/new map-log-head transition and concurrent-CAS/crash vectors, monitor
  replay, dual-root checkpoint, map/log proof-substitution, split-view, and
  jointly governed algorithm/version-transition tests, plus v0.23.4 signer
  reservation/result vectors for lost responses, provider reconciliation,
  competing-CAS supersession, stale signed transitions, exact retry, stale-head
  pre-sign rejection, bounded concurrency/quota accounting, prepared-data lease/
  rebuild/GC, v0.23.6/v0.52 superseded archive/recovery, and three-state incoming
  candidate/quarantine/admission vectors with aggregate item/byte/signature/page/
  work ceilings and non-authoritative `ResourceLimit` results;
- hostile bundle/sync preflight benchmarks cover valid-signature stale floods,
  many-identity/session/bundle aggregation, reconnect/resubmit, atomic quarantine
  reservation/capture/restart cleanup, bounded abuse receipts, sender-held
  refusal, and authority results independent of transport arrival order;
- quarantine-reservation lease vectors and benchmarks cover exact counter and
  scope binding, monotonic idle/absolute expiry, charged-progress renewal,
  reconnect/resume/transfer-split lineage, per-peer/store concurrency ceilings,
  fair scheduling under stalled valid transfers, work-before-use debit, atomic
  candidate-charge conversion, expiry/completion races, and startup orphan
  cleanup without relying on wall-clock rollback behavior;
- lease clock/privacy vectors cover random process `ClockEpoch` binding,
  per-platform source and suspend semantics, clock failure/backward movement/
  wrap, process/reboot epoch mismatch, complete-versus-partial restart recovery,
  no cross-restart resume, encrypted metadata context binding, opaque filesystem
  names, profile counter/timing buckets, privileged inspection, telemetry
  cleanup, and malicious local-observer leakage thresholds;
- benchmarks for cold/warm status and one-file changes in million-file realms;
- encrypted random-read and proof-cache reuse benchmarks;
- plaintext-to-encrypted authority-log cutover model/vectors and crash benchmarks
  covering writer exclusion, terminal tail sealing, bounded authenticated page/
  manifest carry with identical logical root, encrypted successor publication,
  locked recovery, rollback, and retention;
- unlock committed-target/range/slot/result-class validation, protected-state
  typestate, chosen-ciphertext/AD rejection, response-shape, rate/resource, and
  staging-cleanup benchmarks;
- million-object semantic-commitment reverse-index lookup and authenticated
  rebuild benchmarks;
- multi-instance forward/reverse lookup, compartment-root realm-manifest proof,
  and endpoint-local placement reconciliation benchmarks;
- concurrent offline duplicate creation, locator candidate-set reconciliation,
  duplicate-equivalence admission, and future-reference selection benchmarks;
- authenticated logarithmic inclusion/absence proofs, immutable path-copy
  updates, history-independent normalization across insertion/deletion/union/
  split/merge/bulk-build permutations, logical-root computation, signed manifest
  admission, creation-operation reserve/consume/cancel, full-rebuild replay,
  bounded delta-chain verification, mandatory rebuild, witness full replay and
  evidence validation, commitment-key rotation, randomized node encryption and
  placement,
  placement-only re-encryption/repack/receipt/relocation, escrow-right
  allocation/transfer/spend/surrender/burn, quota-conflict merge and
  authoritative-expiry evaluation, non-retroactive ratification,
  duplicate-identity versus instance-fanout accounting,
  duplicate-amplification detection,
  conflict-head reconciliation, and multi-parent representative-resolution
  benchmarks;
- wrapped-DEK creation, recipient rewrap, multi-wrapper enumeration, and
  compartment-scale key-rotation benchmarks;
- cross-compartment copy/move target creation, index commit, source removal, and
  target-policy evaluation benchmarks across deep trees, shared subgraphs,
  leaf-equality proofs, container-isomorphism proofs, sparse promised fetches,
  target-only attestation verification, neutral-domain lifecycle, redacted
  placeholder handling, redacted refusal, and large Merkle-chunked resumable
  translation manifests;
- redaction preflight, wrapper destruction, tombstone propagation, stale-body
  quarantine, repack, and partial-redaction copy-on-write benchmarks;
- KMS/HSM destruction dispatch, idempotent outcome query, ambiguous recovery,
  canonical evidence verification, terminal uncertainty closure, local
  wrapping-epoch rewrap, private-evidence encryption, minimal selective
  disclosure, and all-recovery-path evidence aggregation benchmarks;
- full-world verification and hostile-bundle rejection benchmarks;
- opaque outer/inner bundle preflight, quarantine, decrypt, typed-ingest,
  cumulative-work-budget, blind-claim, and stable-versus-unlinkable quota-mode
  benchmarks;
- resumable bundle-import substitution benchmarks across bundle, manifest,
  quarantine generation, byte/chunk position, verifier/schema, key session,
  policy/revocation context, completed transcript, and consumed budget;
- bundle retained-handle/path-substitution, copy-and-rehash, durable resume CAS,
  checkpoint copy, snapshot/store clone, duplicate-completion, and original-
  ceiling preservation benchmarks;
- concurrent source-descriptor/mmap mutation, integrity-bound quarantine capture,
  same-byte owned-memory validation/parse, isolated service, authenticated pack
  pages, OS-seal refusal/fallback, outer claim-taxonomy, and disconnected-fork/
  reconciliation-equivocation fixtures;
- p50/p95/p99 latency, memory, I/O amplification, and ciphertext-expansion
  budgets;
- per-privacy-profile measured no-op/edit/read/write/unlock/repack/sync traces
  covering size, timing, frequency, access-pattern and repeated-operation
  linkability, filesystem timestamp/directory churn, batching latency, padding
  overhead, and cover-traffic bandwidth/storage cost;
- privacy-profile runtime state-machine traces for healthy operation,
  degradation detection, fail-closed protected traffic, unavailable
  capabilities, restart/partition recovery, recovery observation windows,
  permanent weaker labelling of affected intervals, and non-oracular status;
- malicious local storage-provider benchmark/simulation that retains old
  ciphertext and observes filenames, metadata, offsets, journals/CoW effects,
  and I/O timing/frequency under every supported privacy profile;
- explicit locator-index maximum read, write, proof, node, path-copy, split,
  normalization, rebalance, and concurrent-union amplification thresholds at
  small, million-entry, and adversarial distributions;
- regression comparison against the v0.92.1, v0.99.2, and v0.121.1 admission
  threshold artifacts, with any changed fixture, metric, environment, or
  accepted regression explicitly reviewed rather than silently rebased;
- CI regression thresholds where stable.

Verification:

- differential test suite;
- benchmark runner and recorded baseline artifact.

Exit criteria:

- Canonical interoperability does not depend on one implementation.
- Projection correctness for high-assurance profiles is exercised by an
  independently developed evaluator and full-replay witness, with fail-closed
  disagreement and evidence-preserving migration behavior.
- Sagnir publishes explicit resource budgets and detects material regressions
  in its critical local and hostile-input paths.
- Encryption and redaction claims include measured costs for realistic wrapper,
  compartment, shared-object, and offline-peer scales.
- Private identity performance budgets include bounded reverse lookup and
  logarithmic authenticated lookup and duplicate reconciliation rather than
  assuming one locator maps to one object or accepting linear page scans.
- Projection verification and witness assurance have explicit bounded replay,
  delta-chain, rebuild, transcript-storage, and threshold costs.
- Privacy claims are tied to measured profile leakage and overhead; failed or
  absent cover traffic does not inherit stronger profile claims.
- Provider side-channel claims and runtime privacy health are bounded,
  reproducible assurance artifacts with explicit residual risks rather than
  unqualified whole-system guarantees.

### v0.133.0 - Cross-Platform Build Gate

Goal: complete the comprehensive 1.0 portability audit after continuous
per-version portability checks.

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
- Canonical vectors, policy evaluation, and portable source-state results match
  the continuous platform gates inherited by earlier milestones.

### v0.134.0 - Rootless Podman Gate

Goal: make `saga` usable from a rootless container.

Deliverables:

- rootless Podman build;
- rootless Podman run;
- release base image digest pinning;
- deterministic OCI image manifest and multi-platform index generation where
  supported;
- recorded image config, layer, manifest, and index digests;
- CLI smoke test;
- non-root user in image;
- container documentation.

Verification:

- `scripts/podman_smoke.sh`

Exit criteria:

- A user can run the CLI in rootless Podman.
- Release images do not use mutable base image tags.
- OCI manifests and indexes are stable release artifacts ready for signing and
  provenance binding.

### v0.135.0 - Release Evidence

Goal: make release outputs auditable.

Deliverables:

- SBOM generation;
- signed release checksums and detached release-artifact signatures;
- signatures over every published OCI image manifest and multi-platform image
  index, not only tarballs or generic checksum files;
- provenance attestation binding source commit and signed tag, toolchain,
  dependency lock, build command, target, artifact digests, SBOM digest,
  release notes, release-gate result, OCI image config and layer digests, image
  manifest digest, and multi-platform index digest;
- offline verification instructions and machine-readable verification command;
- release-signing key custody, rotation, revocation, compromise, and historical
  verification policy;
- refusal to publish when an artifact, checksum, signature, provenance
  statement, SBOM, tag, source commit, OCI manifest, image index, or referenced
  image layer does not match;
- reproducible local release build check;
- release notes validator;
- signed tag checklist;
- release runbook.

Verification:

- `scripts/generate-sbom.sh`
- release metadata validator;
- release signature and provenance verification fixture.

Exit criteria:

- A release candidate produces independently verifiable signed artifacts,
  checksums, OCI manifests and indexes, SBOMs, and provenance tied to the exact
  admitted source and gate result.

### v0.136.0 - 1.0 Release Candidate Gate

Goal: freeze the 1.0 feature set and reject incomplete production behavior.

Deliverables:

- 1.0 release gate script;
- all required commands covered by tests;
- documentation consistency validator passes;
- no supporting architecture document weakens the normative roadmap;
- v0.92.1, v0.99.2, and v0.121.1 admission artifacts, independent vectors,
  disclosure reviews, and model results are complete and match implementation;
- no admission prototype or experimental magic is reachable from production
  feature graphs, release binaries, durable decoders, migration paths, or
  authoritative signing/manifest APIs unless explicitly promoted and reviewed;
- shared store platform boundary is the only authoritative filesystem path used
  by CLI, daemon, migration, and recovery code, with no private frontend fork;
- v0.17.1 history-independent algorithm admission and v0.17.2 bounded immutable
  page format, unique representation, authenticated subtree summaries,
  logarithmic point proofs/updates, output-sensitive range/page proofs, streaming
  rebuild, cumulative budgets, and malformed/mutable/non-canonical page refusal
  pass independent vectors and release tests;
- v0.23.3 minimal authority transaction model, base format, durability profile,
  torn-record/race/compaction/recovery suites, provider/store reconciliation,
  and in-place later record-kind extension pass; no legacy/new log authority
  selection or dual writer exists;
- exact SHA3-256 authority frame/transaction/state/checkpoint transcripts,
  independent vectors, genesis and signed-checkpoint anchors, non-circular
  signing frontiers, algorithm-transition refusal/migration, and logical-root-
  preserving physical-compaction pass;
- v0.23.6 active/covered-fence/exception/archive model, exact interval coverage
  and disjointness, permanent low-sequence exceptions, exception resolution,
  replay rejection, reachable historical evidence, copy-on-write archival,
  archive-unavailability behavior, old-checkpoint restoration, and explicit
  production-inactive refusal tests pass;
- v0.52.0 is the only production terminal-fence activation point; typed v0.27.0
  checkpoint, v0.35.0 retention, all-active-replica stability/retirement, and
  expected-root evidence gates pass with no local/quorum-only bypass; first
  activation atomically raises the format/WAL/checkpoint minimum authoritative
  verifier/writer floor and pre-v0.52 tools refuse writes and full current-root
  verification despite metadata rollback or feature stripping;
- cumulative decode budgets, atomic encoders, body-derived typed graph admission,
  graph-class DAG/SCC semantics, and optimized/reference differential results
  pass their first-admission and release profiles;
- parallel decode/verification workers use non-cloneable child budget leases,
  checked aggregate accounting, and deterministic cancellation/panic handling;
  streaming encoder failures cannot publish partial authoritative bytes;
- v0.12.3 sequential/parallel and cross-host vectors prove authoritative work
  accounting, truncation, and roots are independent of speed, scheduling,
  worker count, deadline cancellation, and unused-lease return timing;
- v0.12.4 policy/evaluator table selection, unknown/downgrade refusal,
  protocol-fixed bootstrap and signed activation frontier, canonical/local/
  operational layer separation, underpriced-table retirement, historical
  semantics, migration, and old/new differential vectors pass;
- every parser/format has corpus-backed fuzz smoke from first admission, every
  applicable concurrent state machine has schedule exploration, and every
  durability/distributed model has reproducible release-profile evidence;
- v0.115.1 sealed-private distributed invariant model artifact passes with no
  unresolved counterexample, records complete bounds/assumptions/reductions/
  coverage/resources/completion metadata, and is reproduced by the release
  gate;
- projection replay certificates, delta-chain bounds/rebuild cadence,
  creation-reservation non-reuse, and projection-witness governance pass
  independent vectors, crash, partition, and adversarial tests;
- the independent projection evaluator and independently implemented
  high-assurance full-replay witness pass differential rebuild/delta tests;
  evaluator disagreement and signed bug-remediation/migration fixtures preserve
  historical evidence and fail closed;
- canonical and cryptographic vectors pass independently;
- production provider side-channel profiles, constant-time assurance boundaries,
  invalid-input response-shape tests, acceleration/fallback equivalence,
  timing-regression artifacts, secret-copy inventory, zeroization review, and
  excluded-adversary statements are complete;
- formal models complete within admitted bounds;
- crash, concurrency, partition, and hostile-network suites pass;
- filesystem durability profiles, parent-directory sync, crash injection, and
  deterministic recovery pass on the admitted platform matrix;
- exact CRC-32C format/vectors, WAL incarnation, sequence exhaustion, clone-safe
  encrypted-profile activation, nonce uniqueness under undetected cloned/
  restored stores, locked recovery, old-key retention, and checkpoint-gated key
  retirement suites pass under every admitted durability profile;
- opaque provider-secret and operation-capability compile-fail/API tests,
  sealed bootstrap/unlock/authoritative/recovery non-conversion and exact
  operation-ID reservation, whole-record zero-release-on-bad-tag, per-suite
  chunk nonce/subkey/exhaustion/retry/final completeness, key-agent IPC replay/
  crash/idempotency/ambiguity, plaintext-declassification consumer/fault tests,
  and secret-copy lifetime audits pass without claiming released plaintext is
  non-copyable;
- v0.23.5 bootstrap/genesis cross-domain model and crash suite prove pending
  genesis remains non-authoritative until exact provider key usability and store
  publication are durably reconciled, with orphan/ambiguous states explicit;
- v0.98.1 unlock target/slot/range/result-class binding, protected-state
  typestate, no-self-authorization, chosen-ciphertext/AD refusal, anti-oracle
  response shapes, and cleanup tests pass;
- v0.98.2 secret-source API/compile-fail tests, forbidden argv/environment/
  config/diagnostic channels, PTY/console restore, descriptor/provider/generated
  input, strict UTF-8/UTF-16 conversion, LF/CRLF and descriptor framing,
  canonical generated-secret encoding, cross-source equivalence, generated-
  credential HMAC-SHA3-256 possession confirmation, independently protected
  provider-held/volatile/platform-sealed pending modes, circular/self-derived
  staging-key and rollback/substitution refusal, exact crash recovery/cleanup or
  pre-authority abort, regeneration refusal, source/profile type refusal before
  acquisition, CSPRNG failure, cancellation/unwind, and sanitization tests pass;
- v0.99.0 transparency current-state map and append-only event log use separate
  admitted structures/proofs and one signed dual-root checkpoint; monitor replay,
  split view, and map/log substitution tests pass;
- v0.99.1 key transitions update the current map, append log, and dual-root state
  head in one expected-head WAL transaction; concurrent, signer-failure,
  duplicate, stale-head, and every crash-boundary fixture exposes the complete
  old state or complete new state and never a split authoritative pair; signing
  uses durable v0.23.4 reservation/result state, ambiguous-response provider
  reconciliation, and non-rebased superseded evidence for transitions and
  periodic checkpoints; stale pre-sign rejection, bounded in-flight and active-
  evidence quotas, terminal supersession, prepared-data leases/GC, and stable
  v0.23.6/v0.52 archival pass tests for floods, ambiguity, retention, and
  recovery; untrusted/bounded-quarantine/admitted-evidence state separation,
  aggregate preflight quotas, sender-held refusal, atomic restart cleanup, and
  non-authoritative `ResourceLimit` behavior pass signed/Sybil/resubmit floods;
- v0.111.1 reservation leases bind exact counters and transfer scope, debit work
  before execution, retain original idle/absolute lifetime and remaining budget
  across reconnect/resume/session replacement, provide bounded fair peer/store
  capacity, convert atomically into durable candidate quota, and recover every
  crash state as one charged candidate or complete cleanup; slowloris, trickle-
  progress, deadline-reset, parallel-starvation, expiry/publication-race, clock-
  rollback, and orphan-recovery suites pass;
- v0.111.2 lease records use random process `ClockEpoch` binding and admitted
  per-platform clock/suspend semantics; restart retains complete charged
  candidates, cleans every incomplete lease without wall-clock reconstruction,
  and does not resume pre-candidate work across epochs; encrypted-realm and pre-
  realm operational metadata, opaque names, profile bucketing/padding,
  privileged inspection, telemetry retention, and inherent blind-network
  leakage pass privacy-at-rest and local-observer suites;
- v0.101.1 plaintext-to-encrypted authority-log cutover model, signed frontier
  anchor, terminal tail seal, encrypted predecessor, bounded page/manifest carry
  preserving the logical root, single-writer activation, locked recovery, prior-
  leakage disclosure, and crash/rollback/retention suites pass;
- v0.101.2 all three encrypted-genesis bootstrap-profile models, vectors,
  platform refusal matrix, no-fallback tests, first-log key proof, normal-key
  replacement, passphrase offline-guess disclosure/labels/policy refusal, and
  v0.98.2-only secret ingestion plus generated delivery/possession confirmation,
  independently protected pending storage, exact resume or pre-authority abort,
  post-consumption cleanup, and crash/recovery/bootstrap-key-retirement suites
  pass;
- deterministic fact-language stratification, typed parameterized obligation
  template/instance identity preimages, preallocated issuance-operation cycle
  break, self-inclusion refusal, evidence-consumption/independence and discharge
  vectors, aggregate obligation-state proof, nested double-count refusal,
  incomplete-fixpoint refusal, schedule-independent bounded top-k explanations,
  admitted-time pagination, SCC, and mutable-storage refusal suites pass;
- exact provider-independent cryptographic suite/parameter/revision IDs, hybrid
  transcript binding, provider-assurance issuer/root/category/freshness trust,
  self-claim separation, realm-admission separation, provider interoperability,
  standards vectors, and errata admission state pass;
- opaque bundle outer/inner visibility, blind claim taxonomy, cumulative bundle
  budget, retained-handle preflight handoff, integrity-bound copy/rehash,
  sealed/service/owned-memory/authenticated-page trusted reads, concurrent source
  and captured-object mutation, two-phase quarantine/admission, durable resume
  generation/CAS single consumption, honest disconnected-clone fork/
  reconciliation limits, and context-complete authenticated resume-checkpoint
  substitution suites pass; v0.99.1 candidate states, aggregate item/byte/
  signature/page/work ceilings, pre-quarantine `ResourceLimit`, bounded abuse
  receipts, sender-held evidence, partial cleanup, and arrival-order-independent
  authority selection pass bundle, clone, sync, and reconnect fixtures;
- v0.111.1 pre-candidate reservation leases pass exact-scope/counter vectors,
  monotonic idle and hard-lifetime expiry, charged-progress renewal, reconnect/
  resume inheritance, fair scheduling, work-before-use debit, atomic conversion,
  expiry/publication races, and deterministic startup reconciliation fixtures;
- v0.111.2 process/reboot/suspend clock-epoch, complete-versus-partial restart,
  clock failure/wrap, encrypted metadata replay, key-unavailable refusal, opaque
  path/log/metric, bucketing/padding, cleanup-retention, and blind-observer
  leakage fixtures pass on every admitted platform/privacy profile;
- documented p50/p95/p99 resource budgets meet release thresholds;
- privacy-profile leakage traces, malicious local storage-provider simulations,
  padding/batching/cover-traffic overhead bounds, and profile downgrade/refusal
  behavior meet declared release thresholds;
- privacy-profile `Healthy`/`Degraded`/`Unavailable`/`Recovering` transitions,
  protected fail-closed behavior, encrypted degraded-interval evidence,
  restart/partition recovery, non-retroactive claims, and non-oracular status
  reporting pass release fixtures;
- known limitations document;
- security controls updated;
- threat model updated;
- boundary-by-boundary threat-model and security-control map audit complete;
- release artifact signatures and provenance attestations verify independently;
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
- one shared audited native store platform boundary used by every frontend;
- durable repository-format compatibility, golden fixtures, and transactional
  migration;
- realm genesis-bound identity, first-contact trust bootstrap, governance,
  full invitation lifecycle, membership, and trust roots;
- normative canonical formats and independent vectors;
- cumulative nested decode budgets, non-cloneable parallel budget leases,
  schedule/clock-independent authoritative work accounting, protocol-fixed
  bootstrap and signed realm-selected cost-table semantics separated from local
  ceilings/operational cancellation, failure-atomic streaming encoders, and
  continuous corpus-backed parser fuzz evidence from first format admission;
- computed object hashes and body-derived references;
- typed body-derived graph admission with indexed lookup, duplicate-edge
  refusal, authority DAGs, dependency/impact SCCs, and independent differential
  verification;
- bounded immutable uniquely represented authenticated-map pages with verified
  subtree summaries, logarithmic point proofs/updates, output-sensitive bounded
  range/page proofs, streaming rebuild, append-only commitments, and complete
  checkpoints;
- one formally modeled authority transaction substrate used from bootstrap
  capability reservations through the general WAL, with durable reservations
  before capability minting, durable reconciled results before success,
  provider-side journal separation, deterministic crash recovery, and no
  temporary authorization log, migration, dual writer, or authority-source
  selection;
- bounded composite authority state with active-operation pages, exact terminal
  covered sequence fences, sparse exact permanent exceptions, predecessor-linked
  epoch archives, atomic exception resolution, and no probabilistic replay
  decision or whole-lifetime map record;
- terminal-fence formats remain inert until typed checkpoint, retention, and
  all-active-replica causal-stability/retirement evidence activates production
  advancement at v0.52.0, whose first activation atomically raises a durable
  minimum authoritative verifier/writer floor that older tools cannot bypass by
  rolling back format metadata or stripping a feature marker;
- exact domain-separated SHA3-256 frame, transaction, logical authority-state,
  and physical log-checkpoint commitments anchored by genesis, signed realm
  checkpoints, or retained witnesses, with non-circular signing frontiers and
  logical-root-preserving compaction;
- a staged cross-domain bootstrap/genesis ceremony that keeps pending genesis
  non-authoritative until the exact provider key, signature result, and store
  publication are durably reconciled, including explicit orphan, lost-response,
  ambiguous, abort, and recovery states;
- explicit minimal-plaintext, externally provisioned, and passphrase-derived
  encrypted-genesis bootstrap profiles that break the first-reservation/key
  cycle, never silently fall back, and replace purpose-scoped bootstrap keys
  with a normal realm WAL key; passphrase protection is labeled separately,
  bounded by passphrase entropy/KDF cost, permanently offline-guessable from
  retained header/ciphertext copies, and refusable by high-assurance policy;
- no-echo terminal, ownership-checked descriptor, admitted credential-provider,
  and generated-secret input boundary with no secret values in argv, environment,
  ordinary config/stdin, logs, diagnostics, or shell history; OS-CSPRNG failure
  applies to salts/generated secrets and human password entropy is not claimed
  measurable; strict cross-platform Unicode-to-UTF-8, Enter and descriptor
  framing, canonical generated-secret bytes, source equivalence, and typed local-
  KDF/provider-handle compatibility are normative; generated opaque credentials
  require a normative HMAC-SHA3-256 possession proof and provider-held,
  volatile-locked, or independently platform-sealed pending storage; interrupted
  delivery resumes with the exact credential or aborts before authority without
  circular protection or regeneration, and cleans staging only after durable
  consumption;
- checkpoint-anchored chained WAL with exact CRC-32C, explicit log incarnations,
  exhaustion, encrypted-profile activation only after clone-safe nonce evidence,
  locked recovery, old-key retention, signed event DAG, and rollback/
  equivocation evidence;
- named filesystem durability profiles, complete parent-directory publication,
  failure injection, and deterministic idempotent recovery;
- world and change workflow;
- convergent multi-head worlds and deterministic multi-parent merges;
- all-active-replica causal stability with explicit retirement cutoffs;
- seal and amend;
- status and diff;
- byte-preserving cross-platform paths and root-bound materialization;
- stable worktree snapshots, incremental indexes, and recoverable
  materialization;
- context-bound signatures, key lifecycle, and anti-replay;
- opaque non-cloneable provider-secret handles; sealed non-convertible bootstrap,
  unlock, authoritative, and recovery capabilities with preallocated operation
  IDs and authenticated crash-idempotent key-agent IPC; authenticate-before-
  release whole/chunked AEAD with exact nonce/subkey/retry limits; explicit
  auditable plaintext declassification; and bounded memory-lifetime/zeroization
  claims without claiming released plaintext is non-copyable;
- threshold-governed end-to-end emergency recovery ceremony;
- explicit causal/checkpoint time semantics;
- canonical authoritative time and revocation statements with monotonic
  sequencing, trust-root/key lifecycle, consistency/equivocation evidence,
  optional quorum/diversity, request privacy, and offline freshness limits;
- canonical realm/world policy separated from local acceptance policy;
- deterministic policy resource limits and historical evaluator migration;
- target-bound proof artifacts and compound policy admission;
- canonical typed parameterized obligations, scoped discharge evidence, and a
  derived non-authoritative fixed-size summary for built-in classes;
- non-circular domain-separated obligation template and governed-instance
  identity using a preallocated issuance-operation ID, explicit evidence-reuse/
  consumption semantics, and nested principal/witness/proof independence;
- bounded proof parsing and verification with private proof defaults;
- non-destructive protected promotion;
- operation undo;
- local facts;
- event log and deterministic fact compiler;
- stratified terminating fact evaluation, deterministic SCC-aware causal
  indexes, and snapshot/plan/evidence-bound query transcripts and pagination;
- auditable explanations;
- why, explain, trace, and impact;
- bounded context packs;
- `saga ask` scaffold over deterministic facts;
- encrypted local realms;
- lock and unlock;
- retained-store/header/slot/range/result-bound unlock capabilities, untrusted
  protected-state staging until complete validation, no decrypted-state self-
  authorization, and bounded failure behavior that does not expose a general
  decryption, key-validity, recipient-membership, slot, or plaintext oracle;
- vault status and leak scanning;
- recipient metadata and rekeying;
- exact provider-independent cryptographic suite, parameter-set, standards-
  revision, errata, and hybrid-component transcript identity, with provider
  assurance issuer/trust/category/freshness evidence and realm admission
  committed separately;
- encrypted indexes, authenticated pages, private locators, immutable semantic
  commitments, and metadata protection;
- random-blinded confidential semantic commitments with explicit visibility
  rules and encrypted authenticated locator translation;
- canonical composite-key private-locator index, logarithmic proofs,
  history-independent uniquely represented keyed logical roots with stable
  multi-instance leaves and signed checkpointed compartment/realm manifests,
  count-hiding partial-access proofs, endpoint-local placement separated from
  randomized encrypted node envelopes and public storage IDs, exact semantic-
  instance reverse indexes, and immutable offline duplicate identities;
- context-bound random-nonce encryption-instance IDs, keyed nonce-bound opaque
  compartment handles with signed rotation, and deterministic semantic-ledger
  projection completeness proofs with explicit partial-access trust;
- durable single-use creation-operation reservations, bounded authenticated
  projection replay/delta certificates with mandatory rebuild cadence, and
  independent governed projection witnesses;
- signed escrowed replica quota rights, actor/device aggregate limits,
  safe surrender/fencing/cutoff/burning on retirement, merge-time
  overdraw/double-spend quarantine, authoritative causal/checkpoint expiry,
  non-retroactive ratification, separate instance-fanout capacity,
  locator-rotation carry-forward, encrypted quota metadata,
  duplicate-amplification detection, and conflict-safe representatives;
- signed Merkle-chunked recursive cross-compartment graph copy/move transitions
  with typed leaf-equality/container-isomorphism proofs, sparse/redacted
  descendant handling, honest target-only authority attestations, complete
  neutral-object lifecycle and intentional-linkability disclosure, canonical
  redacted placeholders, least-privilege bridge disclosure, resumable bounded
  manifests, source/target/policy CAS, new target identities and encryption
  instances, target-policy evaluation, and explicit distributed conflicts;
- pre-implementation algorithm/format admission stops for private indexing,
  compartment translation, and irreversible erasure evidence;
- pre-sync sealed-private distributed invariant closure gate;
- reproducible model assurance with exact bounds, assumptions, property classes,
  reductions, explored-state coverage, resources, seeds/configuration, and
  completion status;
- measurable sealed-private privacy profiles covering size/timing/frequency/
  access-pattern and filesystem leakage, malicious local storage observation,
  padding/batching overhead, and cover-traffic requirements;
- separate authenticated key-transparency current-state map and append-only event
  log semantics/proofs, signed dual-root checkpoints, monitor replay, gossip, and
  split-view evidence, with every key mutation publishing map, event log, and
  state head atomically through one expected-head WAL transition and every
  transition/checkpoint signature using durable one-use reservation, ambiguous-
  response reconciliation, retained non-rebased superseded evidence, stale pre-
  sign rejection, bounded active quotas/prepared-data leases, terminal
  supersession, authenticated stable archival without dispute-evidence loss,
  and sealed untrusted/bounded-quarantine/admitted-evidence states whose
  aggregate preflight limits refuse excess without evicting admitted evidence;
- no operational encryption mode before sealed-private prerequisites;
- sealed-private migration and honest prior-leakage accounting;
- forward-only plaintext-to-encrypted authority-log cutover with exclusive
  writer fencing, signed closed-frontier anchor, terminal plaintext tail seal,
  bounded authenticated page/manifest carry preserving the logical state root,
  durably published encrypted successor, locked recovery, and no claim that
  previously observed metadata can be recalled;
- crash-safe nonce and live-key session handling;
- independently wrapped erasure-unit data keys separated from private-locator
  keys and immutable semantic commitments;
- independently destroyable local erasure-unit KEKs/key slots or complete
  parent wrapping-epoch rotation with surviving-DEK rewrap;
- selective disclosure;
- bundles;
- encrypted bundles;
- opaque padded outer/encrypted inner manifests, cumulative bundle work budgets,
  two-phase quarantine admission, and honest blind ciphertext-integrity/
  availability claim names;
- incoming bundle/sync candidates reserve aggregate item, byte, signature,
  prepared-page, and verification-work capacity before durable quarantine;
  pre-admission `ResourceLimit` leaves sender-held evidence and no partial or
  authoritative rejection state, while identity/session/bundle fanout cannot
  multiply store/realm capacity or influence authority selection by arrival;
- pre-candidate quarantine reservations are bounded local leases with exact
  scope/counters, monotonic idle and absolute lifetimes, charged-progress-only
  renewal, non-resettable reconnect/resume lineage, per-peer/store concurrency
  ceilings and fair scheduling, work-before-use debit, atomic conversion into
  durable candidate quota, and deterministic expiry/crash/orphan cleanup;
- every lease deadline is process-`ClockEpoch` scoped under an admitted platform
  clock; restart keeps atomically complete candidates but cleans incomplete
  leases without wall-clock reconstruction or cross-epoch resume, while known-
  realm and pre-realm identifiable lease metadata is encrypted, context-bound,
  stored under opaque names, profile-bucketed where required, access-controlled,
  and removed with derived telemetry under declared retention;
- privacy-preserving inner signatures, exact outer integrity/authenticity claim
  taxonomy, retained-handle preflight followed by integrity-bound capture and
  sealed/service/owned-memory/authenticated-page trusted reads, and durable
  operation/generation/CAS resume authority with honest disconnected snapshot/
  clone fork limitations;
- authenticated context-complete resumable verification/import checkpoints that
  cannot cross bundle, quarantine, verifier, key-session, policy, revocation,
  transcript, or consumed-budget boundaries;
- bounded quarantine, sparse materialization, and partial clone;
- isolated quarantine, safe repack/GC, promises, and retention roots;
- resumable transport-independent sync;
- removable-file, SSH/stdin, and QUIC transports;
- blind or split-trust encrypted sync mode;
- authenticated negotiation transcripts and scoped replay-resistant resume
  tokens;
- explicit storage receipts and availability semantics;
- replication diversity, retrieval challenges, and availability repair;
- redaction and cryptographic erasure with signed tombstones, non-resurrection
  across implemented sync, repair, backup and snapshot restore paths, an
  explicit archival integration contract, opaque blind-storage deletion
  notices, mixed-pack replacement, pre-erasure verification receipts, an
  irreversible forward-only state machine, uncertain provider-destruction
  recovery, canonical authenticated destruction evidence, signed terminal
  residual uncertainty, encrypted evidence metadata with minimal scoped
  disclosure, concurrent-reference semantics, and explicit non-recall limits;
- Git import/export interoperability without using Git as native storage;
- optional daemon;
- formal, crash, concurrency, partition, fuzz, and performance assurance;
- complete release notes;
- signed release artifacts, checksums, OCI manifests and indexes, SBOM, and
  provenance attestation;
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
- independent proof-system vectors and maintainer pentest coverage of the
  admitted construction, setup assumptions, transcript binding, and resource
  limits.

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
- tombstone-first restore planning and `RedactedBody` preservation;
- quarantine of archived ciphertext superseded by an admitted redaction
  tombstone;
- receipt supersession and refusal to use redacted bodies for availability
  repair;
- implementation of the v0.126.0 mixed-content archive replacement contract,
  including replacement upload, receipt acquisition, atomic lineage update,
  and old-pack deletion only after surviving archive availability is proven;
- receipt-only downstream mode;
- availability proof or explicit unavailable state;
- regulated-policy retention controls;
- missing, substituted, truncated, unavailable, stale-redacted,
  offline-archive, and resurrection-attempt tests.

Verification:

- `cargo test -p sagnir-store`
- archival recovery integration suite.

Exit criteria:

- Cold history can be compacted and restored while missing archive bodies are
  detectable and never represented as locally available proof.
- Archive restoration and availability repair cannot resurrect a redacted
  encryption instance.
- This release converts the pre-1.0 archival redaction contract into executable
  archive replacement and restoration behavior.

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
