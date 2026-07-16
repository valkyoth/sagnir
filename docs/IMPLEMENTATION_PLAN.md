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

## UX And Policy Position

Sagnir must be strict by default without forcing unnecessary ceremony for daily
local work. The storage model, object checks, append-only history, path safety,
and policy engine stay native Sagnir concepts from the first usable workflows.
User-facing commands may still provide high-level workflows that compose those
concepts safely.

Default initialization should create a strict integrity posture:

- canonical object and graph checks are required;
- unsafe worktree paths are rejected;
- immutable history is not rewritten;
- world updates are transaction-backed;
- configured policy is never bypassed by convenience commands.

Normal-user ergonomics should come from workflow commands, not from weakening
the engine. `saga save "message"` is planned as a secure convenience command
that creates or reuses local intent, builds the source-state transition, seals
an immutable revision, records the operation, and updates the current local
world only when policy allows it.

Relaxed project behavior must be explicit. Profiles may tune requirements for
solo, open-source, team, or regulated use, but they must not silently downgrade
security:

- `standard`: default strict integrity with simple local workflow commands;
- `solo`: opt-in fewer evidence/review requirements for private local work;
- `team`: signatures, reviews, and protected worlds are available locally;
- `regulated`: strict signatures, evidence, audit, and promotion policy.

The CLI may be simple; the model must remain Sagnir. High-level commands are
aliases over native objects, proofs, policy decisions, and world transitions,
not a Git compatibility layer.

## Verification Scale Position

Sagnir must keep hostile-input admission bounded while still scaling to large
repositories. Fixed-capacity graph verification is an admission primitive for a
single chunk, bundle, transaction, or parser boundary. It is not the maximum
size of a Sagnir realm.

Repository-scale verification uses explicit modes:

- `bounded-batch`: verify one bounded object/reference batch before ingest;
- `lazy-cone`: verify the touched source-state cone for normal local work;
- `full-world`: verify a whole world when policy or operator choice requires
  high-assurance coverage.

Large worlds must be handled through chunked cryptographic verification,
rebuildable indexes, changed-cone traversal, and proof caching. A seal that
touches one file should be able to reuse unchanged tree, object, and world
proofs instead of re-verifying every object in the realm.

Full-world verification is allowed, but it is never unbounded. It must be
resource-budgeted and configured before allocation:

```toml
[verification]
mode = "full-world"
memory_budget = "96GiB"
parallelism = 32
```

If only a memory budget is configured, Sagnir should derive safe internal
entry/reference chunk sizes from that budget. If only parallelism is configured,
Sagnir should schedule work within default memory limits. Explicit entry and
reference caps may still be used by strict operators who want exact ceilings.

Strict profiles or protected worlds may require full-world proofs for promotion
or release. Normal saves should prefer lazy-cone verification plus cached proofs
unless policy asks for more.

Clone, bundle import, and sync must perform a safety preflight before trusting
or materializing remote state. Sagnir should inspect lightweight metadata first:
format version, object/ref count estimates, chunk manifest, required policy
profile, minimum verification mode, and estimated memory/time. It then compares
that metadata with local verification settings.

The default behavior is:

- proceed when the remote requirements fit local budgets;
- warn and use `lazy-cone` or bounded chunk verification only when policy allows
  fallback;
- refuse trust or worktree materialization when upstream policy requires
  stronger verification than local resources allow;
- allow quarantine or `--no-worktree` fetch for inspection without trust.

Fetching bytes is not the same as trusting bytes. Importing a bundle is not the
same as materializing a world.

## Non-Negotiable Engineering Rules

- Rust stable `1.97.1`, edition 2024, workspace resolver `3`.
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
- `sagnir-event`: typed command and state-transition event envelopes.
- `sagnir-fact`: local fact envelopes, evidence references, confidence, causes.
- `sagnir-memory`: append-only local event and fact memory.
- `sagnir-causality`: causal edges, graph traversal, and impact planning.
- `sagnir-query`: deterministic query planning and execution.
- `sagnir-explain`: auditable explanation objects and builders.
- `sagnir-context`: bounded context packs for diagnostics and optional AI use.
- `sagnir-audit`: proof-backed local audit exports.
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
- native handle-relative local-store backends with fail-closed behavior on
  platforms whose backend is not yet admitted;
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
- `saga save` as a high-level secure local workflow;
- `saga change amend`;
- `saga log`.

Design rule:

- a change is logical intent;
- a sealed revision is immutable evidence for one exact change version;
- a world is policy-bound state, not just a mutable branch pointer.
- `saga save` must never bypass configured policy; it only reduces manual CLI
  steps when the same transition would be allowed through primitive commands.

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

## Phase 5: Facts, Evidence, And Causal Memory

Build local fact support that can answer provenance and explanation questions
without external infrastructure.

Required work:

- structured event envelope;
- event log;
- local fact envelope;
- deterministic fact compiler;
- fact log;
- evidence references;
- confidence score;
- causal links;
- rebuildable causal indexes;
- explanation object;
- context pack object;
- `saga test record`;
- `saga review approve`;
- `saga why`;
- `saga explain`;
- `saga trace`;
- `saga context build`;
- `saga ask` scaffold;
- `saga impact`;
- taint and quarantine fact types.

Memory rule:

- objects, committed WAL frames, admitted events, and canonical facts are
  truth;
- path, symbol, policy, operation, causal graph, explanation, and context
  indexes are rebuildable projections;
- explanations cite facts and objects, show missing evidence, and can be
  audited later.

AI boundary:

- optional AI support may summarize facts, draft query plans, and explain
  already-selected evidence;
- AI output must not create authoritative facts, approve changes, override
  policy, hide evidence, or promote worlds.

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
- sealed private mode uses random-blinded immutable semantic commitments inside
  the encrypted ledger, compartment-keyed private lookup locators, and
  randomized ciphertext storage IDs;
- private locators target one canonical persistent authenticated B+ tree keyed
  by `(locator_epoch, private_locator, semantic_commitment)` so each candidate
  has logarithmic inclusion and absence proofs; it freezes only after the
  unique-representation gate passes;
- logical leaves contain only stable candidate identity and object kind;
  ciphertext IDs, packs, receipts, and positions live solely in mutable
  encrypted placement/reverse indexes, so re-encryption and relocation do not
  change the logical root;
- private index identity has three layers: deterministic keyed logical node/root
  commitments for authorized peers, randomized encrypted node envelopes, and
  public ciphertext storage IDs for blind stores; convergence applies only to
  the logical root;
- a dedicated governed index-commitment key and signed checkpointed manifest
  bind each logical root to admitted semantic state; key possession alone is
  not root-admission authority;
- a history-independent normalization proof must show that insert, delete,
  union, split, merge, and bulk-build permutations produce one root for one
  entry set; otherwise a uniquely represented trie or key-derived tree replaces
  the B+ tree before compatibility freezes;
- deterministic path-copy updates and declared read, write, proof,
  normalization, and rebalance amplification budgets avoid linear page chains;
- aggregate actor/device quotas use signed escrow rights preallocated to replica
  incarnations; offline admission consumes only held rights, causal transfers
  cannot double-spend, and merge-time overdraw remains explicit quarantine;
- retirement reclaims rights only through signed surrender, an acknowledged
  final spent-right root, or an explicit cutoff invalidating unseen spends;
  uncertain rights are burned, and later quota increases ratify through new
  signed history rather than rewriting the original quarantined event;
- duplicate-amplification detection and quota carry-forward prevent identity or
  locator rotation from exhausting candidate sets, while local limits may
  quarantine but never silently discard admitted history;
- duplicate identities remain valid signed history; optional encrypted
  equivalence evidence may guide future references but cannot rewrite old ones;
- representative changes use expected-root compare-and-swap; concurrent choices
  remain explicit conflict heads until an authorized multi-parent resolution,
  and attacker-controlled blinded values never determine representative
  priority;
- authenticated encrypted reverse indexes map semantic commitments directly to
  locator epochs and ciphertext records for bounded graph traversal;
- blind-store metadata, logs, public proofs, and public storage receipts expose
  none of the semantic commitment, blinding value, private locator, or
  translation mapping;
- locator translation uses encrypted authenticated mappings for the 1.0 design,
  not an unadmitted public zero-knowledge proof;
- path names, world names, change titles, author identity, facts, symbol names,
  and AI context packs are protected metadata in serious encrypted mode;
- sync-visible operational metadata is minimized and documented.

Redaction rule:

- a signed private semantic tombstone and distinct `RedactedBody` state survive
  key destruction;
- erasure uses a monotonic irreversible phase plus orthogonal component
  results, not one flat enum;
- reversible planning/preparation and tombstone commit precede
  `DestroyingKeys`; durable per-provider intent and idempotency tokens commit
  before destructive dispatch;
- ambiguous KMS, HSM, filesystem, escrow, wrapper, or recovery-share outcomes
  remain `DestructionUncertain` until post-crash query or other admitted durable
  evidence resolves every path; only then may `KeysDestroyed` commit;
- destruction evidence is a canonical signed, attested, or authenticated
  local-agent envelope bound to provider/key epoch, operation, key, idempotency
  token, request transcript, result, assurance level, and checkpoint; transport
  authentication or an unsigned API response is insufficient;
- full destruction evidence remains encrypted in authorized semantic views;
  blind stores, logs, telemetry, filenames, and locked status expose no evidence
  ID, provider/key metadata, timing, or assurance level, while selective
  disclosure uses separate audience- and purpose-bound minimal statements;
- local wrapper unlink or overwrite is not erasure: local storage uses an
  independently destroyable erasure-unit KEK/key slot or rotates the parent
  wrapping epoch, rewraps every surviving DEK, and destroys the old epoch;
- permanently unresolved operations close only as signed
  `ResidualUncertainty`, remain non-abortable, compact their journal without
  claiming erasure, and may advance later if valid evidence appears;
- recovery is forward-only from `DestroyingKeys`, including when a provider may
  have succeeded before Sagnir journaled confirmation;
- remote acknowledgement and controlled-copy cleanup advance independently
  after key destruction, with a derived top-level state and no backward
  transition;
- stable operation IDs plus status and resume commands expose interruption
  recovery and the remaining permitted actions;
- local erasure, controlled copies, remote acknowledgements, and uncontrolled
  residuals are reported as separate properties;
- private semantic tombstones project to endpoint-scoped opaque storage deletion
  notices that reveal no semantic commitment, path, actor, or compartment;
- anti-entropy propagates admitted tombstones before body requests;
- concurrent historical references remain valid but resolve to
  `RedactedBody`; stale ordering cannot restore availability, and authorized
  reintroduction creates a new encryption instance and DEK;
- sync, repair, receipts, repack, partial clone, and archive restoration cannot
  resurrect a redacted encryption instance;
- stale ciphertext returned after redaction is quarantined and cannot satisfy
  availability or repair policy.
- backup, VM-snapshot, recovery-kit, and air-gapped restore begins restricted
  and reconciles a policy-sufficient redaction frontier before decryption or
  materialization;
- erasure rotates affected wrapping epochs and rewraps surviving DEKs when
  current local storage or controlled backups could otherwise recover an old
  wrapper and its wrapping key, but Sagnir cannot claim erasure until every
  controlled recoverable copy is sanitized or cryptographically superseded.
- mixed-content packs use independently authenticated record deletion only when
  declared by the pack/provider capability; otherwise Sagnir verifies and
  receipts a privacy-padded replacement before deleting the old pack.
- v0.122 defines the core operation and later integration contracts; repack,
  remote receipts, repair, and archival implement those contracts in their own
  milestones.

Compartment movement rule:

- compartment identity is part of sealed-private semantic identity, so a
  cross-compartment move is a signed copy/move transition rather than a rename;
- the target receives a new random-blinded semantic commitment, private
  locator, encryption instance, DEK, selector, and target-policy evaluation for
  every compartment-bound reachable descendant;
- typed translation proves leaf content equality and container structural
  isomorphism under exact source-to-target reference mapping; transformed
  metadata is explicit and containers are never claimed byte-identical;
- a Merkle-chunked recursive authenticated translation manifest handles shared
  subgraphs, bounded streaming verification, durable resume, cancellation,
  temporary GC pins, and atomic final commit while proving no
  source-compartment commitment remains reachable from the target;
- the complete bridge is encrypted only to cross-authorized or governed audit
  actors; target-only recipients receive a minimal target attestation,
  source-only recipients receive no target identifiers, and blind stores receive
  no mapping;
- promised descendants and openings must be fetched and verified; unavailable
  or redacted descendants refuse translation unless an explicitly
  compartment-neutral opaque boundary or typed redacted placeholder is admitted
  without claiming content equality;
- compartment-neutral identity uses a separate typed commitment domain with an
  allowlisted object kind, neutral-only reference closure, and no
  compartment-bound metadata;
- source frontier, exact root, target absence/replacement, and target policy use
  compare-and-swap; concurrent changes become explicit conflict heads;
- source-bound reviews, proofs, and approvals do not automatically authorize
  translated target identities;
- source history remains bound to the original identity, and source removal
  happens only after target durability;
- cryptographic erasure of the source is a separate redaction operation.

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

## Future Archival And Retention Idea

Sagnir should not rush into pruning immutable history, but it should leave room
for verifiable cold storage if `.saga/` growth becomes a real problem. The
preferred direction is compressed archive packs plus immutable archive receipts,
not silent deletion.

Potential model:

- granular history, events, facts, explanations, and proof caches can be moved
  into compressed `.saga/archival/` packs;
- each archive pack has a manifest and cryptographic commitment;
- an archive receipt remains hot and proves which objects, facts, operations,
  and summaries were archived;
- archive packs can be verified and rehydrated when full detail is required;
- downstream clones may choose current state only, receipt-only history, or
  full archive packs;
- strict policies can require full archive retention and disable local pruning.

Archival must never let a project pretend history did not exist. If a receipt
claims an archive exists, Sagnir must detect missing, corrupted, or substituted
archive bodies.

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
- explain local policy decisions and operations;
- trace local causal chains;
- build bounded context packs;
- answer bounded natural-language questions over deterministic facts;
- trace local blast radius;
- enable and use encrypted local realms;
- lock and unlock encrypted realm materialization;
- create and verify encrypted bundles;
- undo through the operation ledger;
- create, verify, and import bundles;
- sync with a minimal Sagnir remote;
- pass security, modularity, release, dependency, and documentation gates.
