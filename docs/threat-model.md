# Sagnir Threat Model

Status: baseline

Sagnir assumes hostile networks, malicious bundles, copied repositories,
compromised developer devices, stolen signing keys, poisoned dependencies,
corrupt object packs, replayed sync messages, stale remotes, malicious
automation, local plaintext leakage while unlocked, and partial local disk
exposure.

## Protected Assets

- source objects;
- world history;
- sealed revisions;
- local facts;
- policy epochs;
- crypto epochs;
- signatures;
- evidence references;
- local operation log;
- local event log;
- causal graph indexes;
- explanation objects;
- context packs;
- sync bundles;
- key metadata;
- encrypted realm keys;
- immutable private semantic commitments and blinding values;
- private lookup locators and translation mappings;
- locator candidate buckets, semantic reverse indexes, and private duplicate
  equivalence evidence;
- recipient slots;
- compartment metadata;
- encrypted bundle manifests.
- invitation and enrollment lifecycle records;
- key-transparency roots and consistency evidence;
- recovery shares, ceremony evidence, and replacement authority;
- redaction tombstones and `RedactedBody` state;
- opaque blind-storage deletion notices and pre-erasure verification receipts;
- restored backup, VM-snapshot, recovery-kit, archive, and air-gapped state;
- release-signing keys, checksums, artifacts, SBOMs, and provenance statements.

## Initial Threats

- forged object type or digest confusion;
- malicious canonical encoding;
- WAL corruption;
- alias rollback;
- path traversal during worktree materialization;
- symlink traversal outside the worktree or into `.saga/` control data;
- concurrent namespace replacement during privileged local-store operations;
- `.saga/` control data accidentally tracked as source;
- forged facts or reviews;
- forged or replayed command events;
- event-to-fact compiler confusion;
- explanation output that hides missing evidence;
- context packs that leak unrelated source, facts, keys, or protected metadata;
- AI summaries being mistaken for authoritative evidence;
- oversized signature or fact payloads;
- unauthorized promotion;
- poisoned automation output;
- bundle replay;
- local key disclosure through logs or debug output.
- plaintext left in editor caches, build outputs, language-server indexes,
  shell history, backups, or OS search indexes;
- public object IDs, semantic commitments, logs, proofs, receipts, or metadata
  leaking known plaintext membership in encrypted realms;
- dictionary attacks and cross-compartment correlation against low-entropy
  private content;
- locator equality collapsing distinct independently signed semantic
  commitments or keyed-locator collisions aliasing different plaintext;
- an authorized replica exhausting a locator bucket through repeated blinded
  duplicates, replaying quota state, or forcing unbounded reconciliation;
- concurrent equivalence transitions choosing different future
  representatives, with arrival order or grindable blinded values deciding the
  result;
- corrupt forward/reverse indexes forcing full scans, substitution, or wrong
  ciphertext resolution;
- cross-compartment content being treated as a rename and retaining source
  identity, keys, recipients, or policy in the target compartment;
- recipient removal being mistaken for retroactive access revocation.
- replayed, duplicated, expired, revoked, superseded, or over-scoped
  invitations;
- key-transparency split views, false absence, rollback, or hidden revocation;
- recovery ceremonies that expose shares, retain stale authority, or create an
  unauthenticated second administration path;
- stale peers, archives, receipts, or availability repair resurrecting
  ciphertext after redaction;
- malicious or substituted storage notices deleting arbitrary blind-store
  ciphertext or correlating private content across epochs and providers;
- restored backups or recovery kits reviving old DEK wrappers and superseded
  wrapping keys before current redaction state is observed;
- current-storage rekeying being misreported as erasure while a controlled
  backup still holds a recoverable old wrapper and wrapping key;
- rollback or key recreation after the erasure state machine dispatches its
  first destructive request in `DestroyingKeys`;
- a KMS, HSM, filesystem, escrow, wrapper, or recovery-share destruction
  succeeding immediately before a crash and Sagnir falsely reporting either
  success or failure without durable evidence;
- concurrent or causally later events, merge order, replay, receipts, or repair
  resurrecting a redacted encryption instance;
- deletion of a mixed pack before live-record replacement durability and
  required remote receipts are established;
- replacement pack size, lineage, or position mapping revealing which records
  were redacted or survived;
- substituted release artifacts, signatures, checksums, SBOMs, tags, or
  provenance attestations.

## Design Responses

- domain-separated object IDs;
- strict canonical decoding;
- append-only operation and fact logs;
- separation between command events and authoritative facts;
- deterministic fact compiler rules;
- auditable explanation objects;
- bounded context packs with redaction notices;
- AI output cannot create authority, override policy, or promote worlds;
- immutable objects;
- rebuildable indexes;
- deterministic promotion preflight;
- case-folded `.saga` control-path detection;
- symlink resolution required before filesystem I/O accepts tracked candidates;
- canonical restricted-root checks and handle-relative, no-follow Unix store
  initialization prevent `.saga/` namespace replacement from redirecting an
  active initialization transaction;
- effective-user ownership checks, retained root/store identities, attachment
  checks, and temporary/committed file identity checks prevent detached or
  substituted Unix initialization from reporting success;
- non-Unix stateful initialization fails closed until a native
  handle-relative backend has hosted security tests;
- future filesystem APIs must use a verified worktree-path type so symlink
  boundary checks are enforced before source-state I/O;
- bounded signature envelopes;
- crypto-agile metadata;
- encrypted realm storage;
- random-blinded immutable semantic commitments kept inside the encrypted
  ledger;
- compartment-keyed private locators and encrypted authenticated translation
  mappings separated from ciphertext storage IDs;
- bounded authenticated locator pages, canonical per-replica admission quotas,
  duplicate-amplification detection, authenticated semantic reverse indexes,
  and duplicate-equivalence transitions that preserve old signed identities;
- expected-root representative compare-and-swap, explicit conflict heads,
  multi-parent resolution, and prohibition on randomness-derived winner
  selection;
- explicit prohibition on semantic commitments in blind-store metadata, logs,
  public proofs, and public storage receipts;
- lock/unlock materialization;
- recipient key wrapping;
- governed invitation issuance, acceptance, consumption, expiry, revocation,
  and supersession;
- authenticated key-transparency maps with inclusion, absence, consistency,
  checkpoint, monitor, and split-view evidence;
- key epochs and rekey operations;
- signed cross-compartment copy/move transitions with new target commitments,
  locators, encryption instances, DEKs, selectors, and target-policy checks;
- threshold-governed recovery ceremony with stale-authority invalidation;
- signed redaction tombstones, distinct `RedactedBody` state, tombstone-first
  anti-entropy, and quarantine of stale ciphertext;
- endpoint-scoped opaque storage notices authorized by separate deletion keys
  without semantic-ledger disclosure;
- restricted restore admission and wrapping-epoch rotation when controlled
  backups could recover old DEK wrappers;
- durable forward-only erasure phases with pre-dispatch destruction intent,
  per-provider idempotency and query evidence, explicit
  `DestructionUncertain`, and separate local, controlled-copy,
  remote-acknowledgement, and uncontrolled-residual results;
- concurrent historical redaction references resolve to `RedactedBody`, while
  authorized reintroduction requires a new encryption instance;
- privacy-padded mixed-pack replacement before old-pack deletion;
- redaction-aware storage receipts, availability repair, and archive restore;
- signed release artifacts, checksums, SBOMs, and provenance bound to the exact
  source, tag, toolchain, dependencies, target, and release gate;
- plaintext leak scanner;
- local proof verification;
- causal impact traversal;
- quarantine state;
- no unsafe code in trusted crates.
