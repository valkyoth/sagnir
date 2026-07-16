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
tags. Independent pentest scope should be proportional to the change and should
expand at phase boundaries, cryptographic trust boundaries, distributed-state
boundaries, and release candidates.

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

### v0.11.0 - Architecture Documentation Authority

Goal: make the expanded roadmap and supporting architecture documents agree
before new trust formats are implemented.

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
- declare which document is authoritative when future drafts conflict.

Verification:

- `scripts/check_doc_links.sh`
- documentation consistency validator;
- `scripts/checks.sh`

Exit criteria:

- No supporting architecture document describes a weaker trust boundary than
  this version plan.
- CI rejects reintroduction of the known contradictory patterns.

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

### v0.17.0 - Authenticated Commitment Structures

Goal: choose canonical authenticated structures for membership, absence, and
append-only consistency.

Deliverables:

- canonical authenticated map for object, alias, fact, policy, and key roots;
- append-only event commitment such as an admitted MMR-style structure;
- canonical node, empty-root, and proof encodings;
- inclusion, absence, append, and consistency algorithms;
- independent known-answer vectors;
- malformed proof, ambiguous key, and extension-confusion tests.

Verification:

- `cargo test -p sagnir-proof`
- commitment vector validator.

Exit criteria:

- Every root and proof operation has one normative structure and byte encoding.
- Membership and append-only consistency are not overloaded onto one structure
  when their required security properties differ.

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
- out-of-band fingerprint verification workflow;
- explicitly warned trust-on-first-use mode;
- persisted first-contact trust record and later mismatch detection;
- no silent trust inheritance from DNS, URL, transport certificate, remote
  endpoint identity, or realm name;
- fingerprint substitution, invitation replay, endpoint impersonation,
  same-name realm, TOFU mismatch, and downgrade tests;
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
- algorithm and suite allow-list;
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

### v0.22.0 - Canonical Signed Statement Transcripts

Goal: bind authoritative signatures to complete realm and transition context.

Deliverables:

- versioned transcript format per statement and action type;
- realm, target, parent/frontier, base root, and result root commitments;
- source and target world commitments where applicable;
- policy root and epoch, crypto suite and epoch, signer key and epoch;
- per-key sequence number, verification scope, and algorithm-transition state;
- domain separation and cross-statement replay tests;
- transcript known-answer vectors.

Verification:

- `cargo test -p sagnir-crypto`
- transcript vector validator.

Exit criteria:

- A valid signature for one realm, action, epoch, world, scope, frontier, or
  algorithm state cannot authorize another.

### v0.23.0 - Signing And Verification Providers

Goal: admit actual cryptographic signing and verification before signed state is
used by storage or worlds.

Deliverables:

- reviewed provider abstraction;
- one mandatory production signature suite;
- key generation/import boundary;
- signing and verification over canonical transcripts only;
- known-answer and established adversarial vectors;
- secret redaction and zeroization through the admitted sanitization crate;
- provider failure, unsupported-suite, and key-substitution tests.

Verification:

- `cargo test -p sagnir-crypto`
- provider known-answer vector suite;
- `cargo deny check`

Exit criteria:

- Sagnir creates and verifies real context-bound signatures without relying on
  envelope shape as evidence of authenticity.

### v0.24.0 - Key Lifecycle And Anti-Replay

Goal: define governed key authority before keys authorize durable events.

Deliverables:

- governed key epoch transitions;
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

- Old, revoked, rotated, replayed, or algorithm-retired keys cannot silently
  regain authority.

### v0.25.0 - Live Governance Enforcement

Goal: activate signed realm governance only after signature providers, canonical
transcripts, identity schemas, and key lifecycle rules exist.

Deliverables:

- cryptographically verified genesis administrator authority;
- signed actor and device enrollment;
- signed membership and role transitions;
- threshold administrative actions over independent principals;
- signed ownership transfer and emergency recovery;
- governance root updates committed into checkpoints;
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
- Every live governance mutation verifies against the admitted genesis and
  current governance root.

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
  policy, key-registry, and crypto-epoch roots;
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
- optional trusted timestamp attestation for regulated profiles;
- clock rollback, suspend/resume, restored-snapshot, and conflicting timestamp
  tests.

Verification:

- `cargo test -p sagnir-core`
- time-policy state-machine tests.

Exit criteria:

- Canonical validity never depends silently on an unauthenticated local wall
  clock.

### v0.29.0 - Typed Ingest Contract

Goal: establish the type-state boundary before durable ingest APIs are written.

Deliverables:

- distinct untrusted, canonical, hash-verified, reference-derived,
  causally-closed, signature-verified, policy-admitted, and committed states;
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

### v0.30.0 - WAL Recovery Model Gate

Goal: model transactional durability before fixing the WAL byte format.

Deliverables:

- TLA+/PlusCal or equivalent WAL recovery model;
- object, fact, world, alias, and checkpoint atomicity invariants;
- write, rename, file-sync, and directory-sync failure points;
- stale checkpoint and missing-body states;
- counterexamples for partial commit and alias-before-body behavior;
- bounded model-check command required by the release gate.

Verification:

- bounded WAL model check;
- model invariant review.

Exit criteria:

- The WAL format and writer milestones begin only after the model has no known
  counterexample within admitted bounds.

### v0.31.0 - Chained WAL Commitment Format

Goal: define bounded WAL frames and chained transaction commitments before
writing transactions.

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
- Reordering, removal, insertion, or replay changes the transaction commitment.
- Adversarial tampering is claimed detectable only relative to an admitted
  signed checkpoint or trusted witness; a store controller can recompute
  unauthenticated local chains.

### v0.32.0 - WAL Writer And Recovery

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
- symlink and reparse-point policy;
- resolver output bound to the exact root, path, and file identity;
- no reusable zero-sized symlink proof capability;
- special-file rejection;
- tracked symlink-as-data semantics without silently following the target;
- sparse-file and hard-link classification policy;
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
- canonical signer set and governance-authorized quorum rule;
- explicit Byzantine, indefinitely offline, retired, and later-returning
  replica behavior;
- retirement grace period and pre-retirement-state admission rule;
- invalidation when governance equivocation, membership rollback, or a
  conflicting stability certificate is detected;
- bounded frontier and concurrent-head representation;
- tombstone and retirement-evidence retention rules;
- safe dotted-context or Merkle-clock compaction;
- replica-creation quotas and governance-backed Sybil controls;
- tests proving compaction cannot erase an unseen concurrent head;
- retirement, rejoin, stale replica, and malicious replica-fanout tests.
- unseen concurrent head, Byzantine quorum, offline quorum, rollback,
  conflicting certificate, and late pre-retirement state tests.

Verification:

- `cargo test -p sagnir-world`
- causal-compaction state-machine model.

Exit criteria:

- Device churn does not make frontiers grow without bound.
- Compaction requires sufficient stability evidence and preserves every
  potentially concurrent admitted head.
- "Sufficient" means the canonical quorum for the certificate's committed
  membership epoch, not the currently reachable peer set.

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

### v0.63.0 - Provider Isolation And Live Key Handling

Goal: contain provider failure and minimize exposure of live secret keys.

Deliverables:

- reviewed provider abstraction;
- process and privilege boundary options for key operations;
- locked-memory policy where available;
- core-dump, fork, swap, and crash behavior;
- optional privilege-separated local key agent interface;
- key confirmation and wrong-key diagnostics;
- secret redaction and zeroization through the admitted sanitization crate;
- provider crash, agent disconnect, fork, core-dump configuration, wrong-key,
  and unsupported-platform tests.

Verification:

- `cargo test -p sagnir-crypto`
- provider known-answer vector suite;
- `cargo deny check`

Exit criteria:

- Provider or key-agent failure cannot leave an operation partially authorized
  or secrets intentionally exposed through logs, debug output, or crash dumps.

### v0.64.0 - Governed Rotation And Emergency Recovery

Goal: implement governance-backed rotation and recovery without creating a
second unauthenticated authority path.

Deliverables:

- ordinary and emergency key rotation transactions;
- threshold recovery authorization;
- ownership transfer workflow;
- stale administrator and split-governance refusal;
- recovery event and checkpoint commitments;
- post-recovery key and policy epoch advancement;
- documentation that recipient removal cannot revoke already acquired keys.

Verification:

- `cargo test -p sagnir-crypto`
- governance and recovery state-machine tests.

Exit criteria:

- Emergency recovery is explicit, threshold-governed, auditable, and cannot be
  replayed as an ordinary administrative action.

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
- General relationship cycles remain representable while dependency cycles are
  rejected.

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
- stale, substituted, partially written, and epoch-change cache tests;
- cache deletion and deterministic rebuild behavior.

Verification:

- `cargo test -p sagnir-proof`
- `cargo test -p sagnir-store`

Exit criteria:

- Cached results can accelerate one-file changes in large worlds without
  surviving any relevant verifier, policy, crypto, or frontier change.

### v0.70.0 - Compound Policy Admission

Goal: combine canonical realm validity and local acceptance into one
non-contradictory admission result.

Deliverables:

- canonical realm/world policy evaluation;
- local acceptance policy evaluation as a separate stricter layer;
- validated compound admission result combining integrity, signatures, causal
  closure, policy decision, and discharged obligations;
- impossible-state prevention for `allow` with unsatisfied obligations;
- evaluator-version and policy-root binding;
- explicit denial source and missing-obligation diagnostics;
- invalid policy tests.

Verification:

- `cargo test -p sagnir-policy`

Exit criteria:

- Draft, review, staging, and production policies can differ locally.
- Relaxed behavior is explicit through profile selection; strict environments
  can require signatures, evidence, review, and promotion checks.
- Promotion code consumes one validated admission result rather than checking
  independent enums that can contradict each other.

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

### v0.81.0 - Why Query

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

### v0.82.0 - Local Impact Traversal

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

### v0.92.0 - AEAD Nonce And Live Key Session Safety

Goal: prevent nonce reuse, wrong-key acceptance, and unsafe live-key handling
before encrypted pages or objects are written.

Deliverables:

- admitted misuse-resistant or nonce-robust AEAD evaluation;
- crash-safe nonce uniqueness or per-record key/nonce derivation;
- concurrent-writer allocation rules;
- counter rollback and exhaustion behavior;
- restored-snapshot nonce safety;
- key confirmation and deterministic wrong-key refusal;
- process memory, core-dump, fork, swap, and suspend policy;
- optional privilege-separated local key agent integration;
- nonce-reuse, crash, snapshot restore, concurrent writer, exhaustion, and
  wrong-key tests.

Verification:

- `cargo test -p sagnir-crypto`
- AEAD known-answer and misuse vector suite;
- nonce-allocation state-machine model.

Exit criteria:

- A crash, concurrent writer, restored filesystem snapshot, or counter rollback
  cannot cause two records under one key to reuse a forbidden nonce.
- Wrong keys fail authentication without exposing parsed plaintext.

### v0.93.0 - Sealed Private Object IDs

Goal: avoid known-plaintext membership leaks before any encrypted realm can be
created.

Deliverables:

- private keyed object ID format;
- ciphertext storage ID format;
- deduplication identity policy scoped to one key domain;
- randomized encryption requirement;
- non-revealing private-ID formatting;
- no public plaintext object hash in sealed-private metadata;
- known-plaintext, cross-realm correlation, formatting, and identity-confusion
  tests.

Verification:

- `cargo test -p sagnir-object`
- `cargo test -p sagnir-crypto`

Exit criteria:

- Sealed-private formats can hide whether known plaintext content is present.
- No user-facing encryption command exists before this identity format.

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

### v0.96.0 - Encrypted Object Envelope

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

### v0.97.0 - Authenticated Encrypted Pages And Index

Goal: support bounded random access without decrypting an entire realm.

Deliverables:

- authenticated encrypted page or segment format;
- encrypted manifest and index;
- signed pack/root commitment;
- isolated compression contexts that never combine attacker-controlled input
  with secrets;
- deduplication and delta bases scoped to one compartment and key domain;
- authentication before decompression or plaintext parsing;
- expanded-size and decompression-ratio limits;
- ciphertext storage hash computed during encryption;
- compression-oracle, cross-compartment base, random-read, substitution,
  truncation, and decompression-bomb tests.

Verification:

- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-store`

Exit criteria:

- Status and index lookup can read bounded authenticated regions.
- Secret and attacker-controlled bytes never share a compression context.
- Unauthenticated ciphertext never reaches decompression or canonical parsing.

### v0.98.0 - Passphrase Unlock Baseline

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

### v0.99.0 - Device Recipients And Recipient Slots

Goal: define device access and recovery without one shared user secret.

Deliverables:

- per-device recipient keys;
- recipient ID, kind, wrapping algorithm, and wrapped key metadata;
- signed recipient authorization;
- recipient and key-transparency records;
- OS keychain, TPM, secure enclave, and hardware-token backend interfaces;
- threshold and offline recovery metadata;
- anonymous recipient slots where feasible;
- key compromise and recovery evidence;
- acknowledgement that removal cannot revoke already acquired keys;
- backend-unavailable, duplicate slot, unauthorized recipient, and recovery
  threshold tests.

Verification:

- `cargo test -p sagnir-crypto`
- backend contract tests with software fixtures.

Exit criteria:

- The format supports device-specific access, revocation evidence, and offline
  recovery without embedding a mandatory platform backend.

### v0.100.0 - Compartment Encryption Boundaries

Goal: establish path, world, and projection access boundaries before encrypted
realm creation.

Deliverables:

- compartment ID and key metadata;
- recipient wrapping of only authorized compartment keys;
- prohibition on giving partial-access recipients a realm master key that can
  derive every compartment;
- `saga vault compartment create`;
- `saga vault protect`;
- `saga vault unprotect`;
- partial unlock metadata;
- compartment, cross-compartment deduplication, and unauthorized derivation
  tests.

Verification:

- `cargo test -p sagnir-policy`
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- A recipient authorized for one compartment cannot derive keys, deduplication
  identities, or delta bases for another compartment.

### v0.101.0 - Sealed-Private Migration And Leakage Accounting

Goal: define safe conversion of an existing open realm before exposing the
encryption command.

Deliverables:

- full object-ID, metadata, index, pack, and compartment rewrite plan;
- crash-safe staged repack and rollback;
- old plaintext identifier and metadata cleanup policy;
- explicit leakage statement for information previously published, replicated,
  logged, cached, or observed;
- prohibition on claiming that migration recalls prior disclosures;
- old-to-new commitment map protected inside the encrypted ledger;
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

### v0.102.0 - Encrypt Project Command

Goal: enable sealed-private encrypted realm storage through `saga`.

Deliverables:

- `saga encrypt project`;
- sealed-private vault initialization transaction;
- encryption-enabled canonical event and compiled fact;
- required migration/repack for an existing open realm;
- irreversible-disclosure warning for previously exposed metadata;
- refusal when sealed-private prerequisites or resource budgets are unmet;
- refusal for already encrypted realms;
- dry-run and interrupted migration tests.

Verification:

- `cargo test -p sagnir-cli`
- `cargo test -p sagnir-store`

Exit criteria:

- A user can enable sealed-private storage only after private IDs, protected
  metadata, recipients, compartments, envelopes, and encrypted indexes exist.
- The command never labels metadata-visible encryption as sealed-private.

### v0.103.0 - Unlock Command

Goal: load admitted keys for a local encrypted realm.

Deliverables:

- `saga unlock`;
- unlock session metadata;
- monotonic time-to-live metadata;
- compartment-aware partial unlock;
- `--no-worktree` verification mode;
- wrong-key, expired-session, compartment-overreach, and failed unlock tests.

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

### v0.106.0 - Rekey And Crypto Epochs

Goal: rotate encrypted realm keys without mutating old history in place.

Deliverables:

- TLA+/PlusCal or equivalent key-rotation model completed before mutation code;
- crypto epoch transition;
- `saga vault rekey`;
- recipient and compartment rewrap plan;
- old-key retention and cryptographic-erasure policy;
- crash-safe staged key rotation;
- rollback and interrupted-rotation recovery;
- invalid epoch, partial compartment, stale recipient, and restored-snapshot
  tests.

Verification:

- bounded key-rotation model check;
- `cargo test -p sagnir-crypto`
- `cargo test -p sagnir-cli`

Exit criteria:

- Key rotation is a signed transition model, not an in-place mutation.

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

### v0.110.0 - Bundle Manifest

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

### v0.111.0 - Bundle Create And Verify

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
- Manifest estimates are treated as untrusted preflight hints; streaming local
  hard limits remain authoritative throughout verification.

### v0.112.0 - Quarantine Namespace And Trust Isolation

Goal: ensure untrusted remote data cannot influence trusted state before full
re-admission.

Deliverables:

- physically or logically separate quarantine namespace;
- identifiers that cannot shadow trusted objects;
- quarantine objects excluded from trusted reference resolution;
- quarantine facts excluded from policy obligations and authoritative indexes;
- no worktree materialization without complete re-admission from original
  bytes;
- storage, age, object-count, byte, fanout, and ancestry quotas;
- deterministic expiry and deletion policy;
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

### v0.113.0 - Bundle Import

Goal: import verified bundles safely, including encrypted bundles.

Deliverables:

- `saga bundle import`;
- object deduplication;
- fact deduplication;
- lazy quarantine without worktree materialization;
- per-import byte, object, ancestry, fanout, and time budgets;
- streaming enforcement independent of manifest estimates;
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
- Both peers authenticate the exact negotiated security parameters before
  exchanging trusted bundle state.

### v0.115.0 - Partition And Anti-Entropy Model Gate

Goal: model distributed convergence and hostile ordering before implementing
live sync transfer.

Deliverables:

- TLA+/PlusCal or equivalent partition and reconciliation model;
- reorder, replay, duplication, delay, disconnect, and resume states;
- concurrent world advancement and multi-head preservation;
- checkpoint, policy epoch, evidence, and key-rotation interactions;
- equivocation and bounded fork handling;
- invariants for no lost heads, no stale admission, no partial trust, and
  eventual convergence under documented assumptions;
- bounded model-check command required by the release gate.

Verification:

- bounded partition model check;
- model invariant review.

Exit criteria:

- Sync transfer implementation begins only after no known model counterexample
  can discard an admitted head or trust partial remote state.

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
- transfer cancellation;
- accepted response;
- denied response;
- quarantined response;
- local-budget-insufficient response;
- blind remote response;
- split-trust remote response;
- local sync result fact;
- protocol tests;
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

### v0.117.0 - Sparse Materialization And Partial Clone

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

### v0.121.0 - Minimal Daemon

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

### v0.122.0 - Redaction And Cryptographic Erasure

Goal: revoke future access to selected encrypted material while preserving
honest signed historical evidence.

Deliverables:

- signed redaction transition identifying scope, authority, reason, and policy
  basis without embedding removed plaintext;
- key-destruction and recipient-revocation plan for cryptographic erasure;
- compartment, object, metadata, index, cache, and repack handling;
- preserved historical commitments and redaction evidence;
- explicit distinction between redaction, logical removal, ciphertext deletion,
  and cryptographic erasure;
- statement that already replicated plaintext, keys, screenshots, logs, or
  exports cannot be recalled;
- interrupted purge, stale recipient, retained key copy, partial repack,
  historical proof, and unauthorized redaction tests.

Verification:

- `cargo test -p sagnir-vault`
- `cargo test -p sagnir-store`
- cryptographic-erasure recovery suite.

Exit criteria:

- Authorized redaction can make retained ciphertext undecryptable where key
  destruction assumptions hold without erasing the signed fact that a
  redaction occurred.
- Sagnir never claims erasure of data or keys already copied beyond its control.

### v0.123.0 - Reachability, Repack, And Safe Garbage Collection

Goal: maintain production stores without deleting state required for integrity,
recovery, partial clones, or policy.

Deliverables:

- canonical reachability roots;
- user, policy, checkpoint, audit, and operation pins;
- quarantine retention separation;
- partial-clone promises and promised-object roots;
- grace periods and recent-object protection;
- repack and compaction transactions;
- proof, checkpoint, governance, and equivocation-evidence retention rules;
- safe deletion criteria;
- interrupted repack/GC recovery journal;
- concurrent writer, stale index, missing promise, interrupted repack, and
  over-eager deletion tests.

Verification:

- `cargo test -p sagnir-store`
- GC reachability property tests;
- repack crash-consistency suite.

Exit criteria:

- Garbage collection deletes only objects proven unreachable from every
  admitted root, pin, promise, retention rule, and in-flight transaction.
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
- no claim that one receipt proves indefinite availability;
- forged, replayed, expired, partial-storage, and unavailable-remote tests.

Verification:

- `cargo test -p sagnir-sync`
- storage receipt vector and integration suite.

Exit criteria:

- Sagnir never reports remote acceptance as durable storage unless the remote
  returns an admitted signed receipt with defined semantics.
- Availability claims state their time, scope, and witness assumptions.

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
- optional evaluated erasure-coding profile with shard commitment and repair
  semantics;
- provider collusion, correlated failure, stale receipt, partial shard,
  unavailable source, and repair interruption tests.

Verification:

- `cargo test -p sagnir-sync`
- deterministic availability simulation;
- repair integration suite.

Exit criteria:

- Sagnir can detect when configured availability falls below policy and repair
  it from an admitted surviving source.
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

### v0.127.0 - Malicious Corpus

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

### v0.128.0 - Expanded Fuzzing

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

### v0.129.0 - Full-System Formal Model Composition

Goal: compose and re-check the model-first subsystem work against the complete
pre-1.0 design.

Deliverables:

- composed WAL recovery, alias CAS, merge/promotion, key rotation, checkpoint,
  GC, and partition models;
- compatibility review between subsystem assumptions;
- checked invariants for atomicity, no lost heads, no stale admission, and
  eventual convergence under documented assumptions;
- model execution instructions and CI smoke bounds.

Verification:

- bounded model-check command;
- model invariant review.

Exit criteria:

- Counterexamples for stale CAS, partial commit, lost divergence, and replay are
  represented in executable models rather than prose alone.
- This release is full-system assurance, not the first time foundational
  formats are modeled.

### v0.130.0 - Crash And Concurrency Assurance

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

### v0.131.0 - Partition And Adversarial Network Tests

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

### v0.132.0 - Differential Vectors And Performance Budgets

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
- CLI smoke test;
- non-root user in image;
- container documentation.

Verification:

- `scripts/podman_smoke.sh`

Exit criteria:

- A user can run the CLI in rootless Podman.
- Release images do not use mutable base image tags.

### v0.135.0 - Release Evidence

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

### v0.136.0 - 1.0 Release Candidate Gate

Goal: freeze the 1.0 feature set and reject incomplete production behavior.

Deliverables:

- 1.0 release gate script;
- all required commands covered by tests;
- documentation consistency validator passes;
- no supporting architecture document weakens the normative roadmap;
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
- realm genesis-bound identity, first-contact trust bootstrap, governance,
  membership, and trust roots;
- normative canonical formats and independent vectors;
- computed object hashes and body-derived references;
- authenticated maps, append-only commitments, and complete checkpoints;
- checkpoint-anchored chained WAL, signed event DAG, and rollback/equivocation
  evidence;
- world and change workflow;
- convergent multi-head worlds and deterministic multi-parent merges;
- seal and amend;
- status and diff;
- byte-preserving cross-platform paths and root-bound materialization;
- stable worktree snapshots, incremental indexes, and recoverable
  materialization;
- context-bound signatures, key lifecycle, and anti-replay;
- explicit causal/checkpoint time semantics;
- canonical realm/world policy separated from local acceptance policy;
- deterministic policy resource limits and historical evaluator migration;
- target-bound proof artifacts and compound policy admission;
- bounded proof parsing and verification with private proof defaults;
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
- no operational encryption mode before sealed-private prerequisites;
- sealed-private migration and honest prior-leakage accounting;
- crash-safe nonce and live-key session handling;
- selective disclosure;
- bundles;
- encrypted bundles;
- bounded quarantine, sparse materialization, and partial clone;
- isolated quarantine, safe repack/GC, promises, and retention roots;
- resumable transport-independent sync;
- removable-file, SSH/stdin, and QUIC transports;
- blind or split-trust encrypted sync mode;
- authenticated negotiation transcripts and scoped replay-resistant resume
  tokens;
- explicit storage receipts and availability semantics;
- replication diversity, retrieval challenges, and availability repair;
- redaction and cryptographic erasure with explicit non-recall limits;
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
