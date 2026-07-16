# Sagnir Vault Encryption

Status: planning document

Sagnir should have native encrypted realms.

The core UX is:

```text
saga encrypt project
saga unlock
saga lock
saga vault status
```

`saga decrypt project` is not the normal mental model. Daily use should be
lock/unlock because encrypted realm storage and plaintext worktree
materialization are different states.

## Design Goals

- Encrypt Sagnir storage as a first-class object, fact, world, proof, and sync
  feature.
- Keep daily editing normal while unlocked.
- Keep `.saga/` protected while locked.
- Support recipient-based key wrapping.
- Support crypto epochs and rekeying.
- Prepare for hybrid classical plus post-quantum recipient wrapping.
- Support encrypted bundles for air-gapped and blind-remote workflows.
- Be honest about plaintext leakage while unlocked.

## Encryption Levels

Level 1: encrypted Sagnir store.

- `.saga/objects` encrypted;
- `.saga/facts` encrypted;
- `.saga/worlds` encrypted;
- `.saga/indexes` encrypted or redacted;
- working tree plaintext while unlocked.

Level 2: encrypted worktree lock.

- keys evicted on lock;
- optional plaintext worktree removal;
- plaintext is materialized again on unlock.

Level 3: compartment encryption.

- path, world, and projection data can belong to compartments;
- users can unlock only admitted compartments;
- protected metadata can stay encrypted from unauthorized clients.

## Metadata Policy

Public metadata may include:

- format version;
- crypto algorithm identifiers;
- encrypted realm marker;
- rounded ciphertext size buckets;
- crypto epoch;
- opaque ciphertext pack commitments and documented availability metadata.

Public or blind-store metadata must not include semantic commitments, private
locators, translation mappings, canonical object hashes, paths, actors, worlds,
facts, or graph-edge identities.

Protected metadata includes:

- file paths;
- file names;
- world names when sensitive;
- change titles and messages;
- actor identity when policy requires privacy;
- fact bodies;
- symbol names;
- dependency graph;
- AI context packs.

Operational metadata should be minimized and documented.

## Key Hierarchy

Sagnir should not encrypt every object directly with a user password.

Planned hierarchy:

```text
user passphrase / device key / hardware key / recipient key
  -> key encryption key
  -> realm master key
  -> world or compartment key
  -> segment or object data key
  -> encrypted object
```

The realm master key is random. A passphrase unlocks or wraps it; the passphrase
is not the realm master key.

Key material types must use Sagnir's admitted sanitization boundary and reviewed
secret-lifetime policy before they are implemented. Until that boundary covers
the required key containers and provider behavior, vault work may model key
metadata but must not store real secret bytes in long-lived Rust values.

## Object Identity In Encrypted Realms

Open mode:

```text
object_id = public hash of canonical plaintext object
```

Sealed private mode:

```text
semantic_commitment = hash(
  domain || realm || compartment || schema || type || canonical_plaintext ||
  random_256_bit_blinding
)
private_locator = keyed lookup over canonical plaintext in one compartment
storage_id = public hash over randomized ciphertext envelope
```

The random blinding value and semantic commitment remain inside the encrypted
semantic ledger. Canonical references and signatures bind the immutable
semantic commitment. The private locator is a rotatable lookup projection, and
the storage ID identifies ciphertext placement.

One private locator can map to multiple semantic commitments. Disconnected peers
may independently create equal plaintext with different random blinding values,
and both signed identities remain valid. The encrypted forward index therefore
uses one canonical persistent authenticated B+ tree keyed by
`(locator_epoch, private_locator, semantic_commitment,
encryption_instance_id)`. Each policy-separated encryption instance is a
separate leaf key with logarithmic inclusion and absence proofs.

The B+ tree has three identity layers. Authorized peers converge on a
deterministic keyed logical node/root commitment. Nodes are stored in randomized
encrypted envelopes that authenticate the logical commitment. Blind stores see
only public ciphertext storage IDs for those envelopes. Re-encryption changes
ciphertext identity without changing the private logical root.

Logical leaves contain only stable candidate information: the composite key,
logical object kind, and erasure-policy class. Ciphertext IDs, pack generations,
receipts, and positions exist only in encrypted placement and reverse indexes.
Re-encryption, repacking, receipt renewal, and relocation may change placement
roots but never the logical root.

One semantic commitment may have multiple encryption instances when retention,
recipient, legal-hold, redaction, or erasure policy differs. Creating another
instance consumes separately governed fanout capacity, not a duplicate-semantic-
identity right. Exact forward and reverse keys include the instance ID, so
lookup never assumes one instance per commitment.

The instance ID is a domain-separated hash over the realm, opaque compartment
or neutral handle, semantic commitment, erasure unit, preallocated creation
operation, and an independent 256-bit random nonce. Its signed creation
transition binds every field. It survives rewrap, re-encryption, repack, receipt
renewal, relocation, and endpoint projection changes; a new independently
erasable or policy-incompatible instance, redaction reintroduction, or
erased-instance replacement receives a new ID.

The creation operation is itself a durable replica-incarnation-bound random
reservation. It is consumed idempotently by one exact transition or cancelled
through authenticated state. Crash, abandonment, expiry, cancellation, or
uncertain outcome never makes the reservation reusable.

The logical structure must be history-independent: every insertion, deletion,
union, split, merge, and bulk-build ordering for one canonical entry set
produces one root. A dedicated index-commitment key is domain- and epoch-bound,
but possession of it does not authorize a root. A signed checkpointed manifest
binds the root to semantic state, policy, membership, and structure version. If
the B+ tree cannot meet unique-representation and bounded-amplification
requirements, Sagnir will select a uniquely represented structure before the
format freezes.

Each compartment has one logical root and commitment-key epoch. An authenticated
count-hiding realm manifest contains opaque compartment-root references and
supports scoped inclusion/continuity proofs without revealing other
compartment identities, entry counts, names, locators, or tree shape.

Signed compartment logical-root manifests are canonical state. Placement and
reverse-resolution manifests belong to one replica/device/storage endpoint.
They may differ across authorized replicas and are reconciled separately from
logical sync; no endpoint's projection wins by arrival order.

Every logical manifest carries deterministic full-rebuild or delta proof
evidence from the admitted semantic ledger to all forward/reverse entries.
Full-view verifiers replay that projection. Partial-access recipients can verify
their own compartment and the required signer/witness policy, but cannot
independently establish completeness of hidden compartments; Sagnir reports
that limitation rather than presenting signature authority as mathematical
completeness.

The 1.0 rebuild evidence is a Merkle-chunked authenticated replay certificate,
not a succinct proof. Verifiers acquire every committed ledger chunk and run
the frozen projection evaluator. Delta replay chains are bounded by count,
bytes, changed entries, work, memory, and checkpoint gap, with mandatory full
rebuilds.

Witness statements identify whether the witness independently replayed the
ledger or only validated supplied evidence. Threshold policy requires
independent principals and administrative domains, rejects nominal Sybil
witnesses, and fails closed on unavailable, revoked, compromised, stale, or
equivocal witness state.

## Provider Side-Channel Boundary

Every production cryptographic provider/backend declares which secret-bearing
operations have an admitted constant-time implementation guarantee and the
platform, compiler, CPU-feature, acceleration, and fallback scope of that
claim. Secret-dependent branches, lookups, failures, diagnostics, temporary
copies, and zeroization behavior require review. Timing distributions and
invalid-input tests detect regressions but do not prove protection from cache,
page-fault, speculative-execution, physical, firmware, or privileged local
adversaries unless a provider profile explicitly includes them.

## Measurable Privacy Profiles

Each sealed-private profile declares observable ciphertext/pack sizes, epochs,
key and endpoint fields, request timing/frequency, access order, repeated
read/write linkability, filesystem timestamps and directory churn, padding and
batching overhead, cover-traffic requirements, and residual leakage.

The malicious local storage model may retain all old ciphertext and observe
names, sizes, offsets, timestamps, journal/CoW behavior, and read/write timing.
Profile tests use reproducible traces and statistical thresholds. Sagnir does
not claim timing or access-pattern protection when cover traffic is disabled,
unavailable, over budget, or distinguishable.

Runtime profile health is authenticated as `Healthy`, `Degraded`,
`Unavailable`, or `Recovering`. Protected profiles emit no new protected
traffic outside `Healthy` except through a separately modelled recovery
channel. Encrypted health records preserve the best-known degraded interval and
its weaker observed assurance; recovery cannot retroactively upgrade traffic
already emitted. Authorized status can report detail, while locked, blind-store,
remote, and public views remain fixed or coarsened so health monitoring does
not expose an activity or outage oracle.

Opaque compartment handles are keyed, random-nonce-bound, and epoch-versioned.
Collision is a security failure, not an alias. Rotation uses a signed
realm-manifest transition and encrypted mapping. It protects future discovery
but cannot hide correlations already observed.

Aggregate actor/device quotas are escrowed into signed rights allocations for
replica incarnations. Offline replicas consume only held rights, and causal
transfers cannot double-spend. Merge-time overdraw or double-spend remains
authenticated quota-conflict quarantine. Duplicate-amplification detection and
quota carry-forward prevent new replicas or locator rotation from resetting
limits. Rights from a missing replica are surrendered, fenced by an acknowledged
final spent-right root, invalidated through an explicit retirement cutoff, or
burned. Governance cannot guess that offline rights were unused. A later quota
increase admits a new signed ratification transition and does not rewrite the
original quarantined event.

Expiry is authoritative only through causal/checkpoint order or an admitted
timestamp authority. Late delivery does not invalidate a provably pre-expiry
spend, while an offline spend that cannot establish its pre-expiry creation
frontier remains quarantined. Local clock behavior cannot extend capacity.

Timestamp and revocation authorities publish monotonic signed statements,
append-only checkpoints, and consistency proofs under governed rotating keys.
Profiles may require authority quorum and administrative diversity. Missing,
conflicting, stale, revoked, or equivocal evidence fails closed, and offline
verification has an explicit freshness ceiling. Timestamp requests use opaque
handles and privacy-preserving batching/padding where configured.

In sealed-private realms, allocations, spent-right references, conflict roots,
replica topology, and actor/device activity remain encrypted from blind stores,
logs, telemetry, filenames, public receipts, and locked status.

Optional private duplicate-equivalence evidence can guide future references but
cannot rewrite historical signatures or references. Representative changes bind
the expected equivalence root and prior representative. Concurrent selections
remain conflict heads until an authorized multi-parent resolution, and no
blinding value, commitment, locator, ciphertext ID, or transition hash supplies
an attacker-grindable priority.

Blind stores receive neither semantic commitments nor private locators. For the
1.0 design, locator translation uses an encrypted authenticated mapping and
authenticated-index evidence, not a public zero-knowledge proof. This prevents
outsiders from checking whether known plaintext appears in an encrypted realm
by comparing public hashes or commitments.

## Cross-Compartment Movement

Because the compartment is an input to every semantic commitment and locator
domain, a cross-compartment directory copy or move recursively translates every
compartment-bound reachable descendant. A Merkle-chunked signed manifest binds
source and target commitments, references, locators, encryption instances,
DEKs, selectors and shared-subgraph mappings, and proves the target reaches no
source-compartment identity.

Typed proofs establish authenticated content equality for leaves and structural
isomorphism for trees/containers under the manifest reference mapping.
Transformed metadata is explicit; rewritten container bytes are not claimed
equal. Construction and verification are streaming, bounded, resumable,
cancellable, temporarily GC-pinned, and committed atomically at the final root.

The complete bridge manifest is disclosed only to actors authorized for both
compartments or a governed audit role. Target-only recipients receive a
target-scoped attestation without source identifiers or graph relationships.
Source-only recipients receive no target identifiers, and blind stores receive
neither side. Audience- and transition-bound disclosures prevent repeated
translations from becoming a public correlation oracle.

The target-only attestation is a revocable and replay-bound assertion by an
admitted issuer that the target was accepted under a named policy. It does not
independently prove hidden source equality or graph isomorphism. A policy that
requires independent equivalence must require bridge/opening access or refuse
until a post-1.0 hidden-witness proof system is admitted.

Its current validity requires a stapled issuer checkpoint, revocation root, and
consistency proof within policy freshness. Offline verification reports the
staple's covered state and becomes unknown when freshness is exceeded.

The transition compares and swaps the expected source frontier/root, expected
target absence or replacement, and target policy root. Concurrency produces an
explicit conflict rather than arrival-order overwrite. A copy preserves the
source. A move records source logical removal only after the complete target
graph and encrypted indexes are durable. Source reviews, proofs, and approvals
do not automatically authorize target identities. Destroying source DEKs
requires a separate redaction operation.

Sparse or partial clones fetch and verify every required promised descendant
and commitment opening before translation. Missing, unavailable, or redacted
descendants cause refusal unless a specifically typed compartment-neutral
opaque boundary or redacted placeholder is admitted without claiming content
equivalence. A neutral boundary uses a separate typed commitment domain, an
allowlisted object kind, neutral-only reference closure, dedicated locator,
commitment, encryption, wrapping, recipient, recovery and epoch lifecycles, and
no compartment-bound metadata; omitting a compartment field from an ordinary
commitment never makes it neutral. Reusing one neutral identity intentionally
creates cross-compartment linkability for actors able to observe it.

The only admitted redacted target projection is the canonical
`RedactedPlaceholder`. It contains no target-visible source commitment, makes no
content-equivalence claim, cannot satisfy body, availability, completeness,
repair, build/test input, or proof requirements, and cannot be replaced by
stale ciphertext. Authorized audit views may resolve its encrypted provenance;
reintroduction requires a new transition and encryption instance.

## Redaction And Restore Projections

Authorized peers retain a private encrypted semantic tombstone. Blind stores
receive only an endpoint-scoped opaque deletion or receipt-supersession notice
for ciphertext IDs and pack positions they already host. The notice is signed by
an admitted storage-deletion authority and reveals no semantic commitment,
path, actor, compartment, or private causal history.

Filesystem backups, VM snapshots, recovery kits, archives, and air-gapped
devices start restore in restricted mode. They must reconcile a
policy-sufficient current redaction frontier before restored keys, wrappers,
ciphertext, or indexes may be decrypted or materialized. A purely isolated
snapshot cannot prove it has observed the latest redaction without a later
checkpoint, witness, authorized peer, or governance recovery record.

If a controlled backup retains both an old DEK wrapper and the wrapping key
that can open it, rotating current storage is not cryptographic erasure. The
backup must be sanitized, replaced, or cryptographically superseded, and the
affected wrapping epoch must rotate with surviving DEKs rewrapped. When that
cannot be established, Sagnir records the residual copy and refuses to claim
erasure. Cryptographic supersession is valid only when backups use an
independently destroyable backup-encryption epoch whose destruction makes every
superseded backup copy undecryptable; metadata marking alone is insufficient.

The same rule applies to current local storage. Unlinking, overwriting, or
truncating a wrapper is not cryptographic erasure because journals, snapshots,
CoW blocks, SSD wear-leveling, and forensic images may retain it. Each erasure
unit therefore uses an independently destroyable local KEK/key slot, or erasure
rotates the parent wrapping epoch, rewraps every surviving DEK, and destroys the
old epoch. Otherwise Sagnir records residual or unverified erasure.

## Erasure State

The durable encoding is a monotonic operation phase plus orthogonal recovery-
path and cleanup results. The phases are `Planned`, `Prepared`,
`TombstoneCommitted`, `DestroyingKeys`, and `KeysDestroyed`.
`StorageNoticesPending`, `ControlledCopiesCleared`, `Complete`, and
`ResidualCopiesKnown` are derived summaries, not exclusive phases.

Before destructive dispatch, Sagnir durably records every wrapper and provider
path, request precondition, and idempotency token. Once `DestroyingKeys` begins,
abort is no longer safe. A KMS, HSM, filesystem, escrow, wrapper, or recovery-
share request that may have succeeded without durable confirmation produces
`DestructionUncertain`. Recovery queries the original provider operation and
fails closed; it never guesses from timeout or local absence. `KeysDestroyed`
requires durable admitted evidence for every enumerated path.

Admitted evidence uses a canonical envelope binding provider identity and key
epoch, operation and key/slot/share identifiers, idempotency token, request
transcript, result, assurance level, and checkpoint. It is authenticated by a
provider signature, hardware attestation, or governed local key agent. Cached
TLS responses, unsigned API results, logs, exit codes, and file absence are not
transferable proof. Revocation, compromise, provider retirement, replay, and
contradiction reevaluate assurance without rewriting historical evidence.

Full destruction evidence remains encrypted inside the authorized semantic
ledger. Blind stores, filenames, logs, telemetry, crash reports, and locked
status reveal no evidence ID, provider/key metadata, timestamp/checkpoint, or
assurance level. Public or selective disclosure creates a separate audience-,
purpose-, target-, policy-, and expiry-bound minimal statement.

If an outcome can never be resolved, a signed `ResidualUncertainty` disposition
closes operational work without claiming erasure. It retains the tombstone and
evidence, remains non-abortable, allows bounded journal compaction and alert
acknowledgement, and may later advance to `KeysDestroyed` only when valid
evidence appears.

Remote acknowledgements and controlled-copy clearance may finish in either
order. The top-level status is derived from their monotonic component results
rather than treating them as one reversible linear sequence.

Status must report local cryptographic erasure, controlled-backup clearance,
remote deletion acknowledgements, and uncontrolled residual copies separately.
`Complete` means configured obligations are satisfied; it never means Sagnir
recalled plaintext or keys copied beyond its control.

Concurrent signed events may retain historical references to an erased
encryption instance, but current resolution is `RedactedBody`. Event order,
merge, replay, receipt, and repair cannot restore that instance. Legitimate
reintroduction requires explicit authorization, a new encryption instance and
DEK, new selectors, current policy evaluation, and a new semantic commitment
when the old opening is unavailable.

Mixed-content packs require either independently authenticated record deletion
or replacement. Replacement is built and verified without redacted instances,
padded according to privacy policy, uploaded and receipted before encrypted
indexes move, and only then may the old pack be deleted. Archive execution is
post-1.0; pre-1.0 milestones define the same non-resurrection contract without
claiming an archive implementation exists.

## Encryption Honesty

Sagnir encryption helps protect against:

- stolen devices while locked;
- leaked `.saga/` directories;
- untrusted backups;
- blind remote storage;
- copied encrypted bundles;
- accidental repository exposure.

It does not automatically protect against:

- malware while unlocked;
- compromised editors or language servers;
- plaintext build artifacts;
- terminal scrollback;
- OS swap;
- screenshots;
- copy/paste history;
- debug logs;
- intentional plaintext export.

`saga vault scan-leaks` should detect common plaintext leak surfaces, but it
must not claim perfect cleanup.

## Commands

Planned commands:

```text
saga encrypt project
saga unlock
saga lock
saga lock --wipe-worktree
saga vault status
saga vault scan-leaks
saga vault recipient list
saga vault recipient add
saga vault recipient remove
saga vault rekey
saga vault rotate
saga vault export-recovery
saga vault import-recovery
saga vault compartment create
saga vault protect
saga vault unprotect
saga decrypt export
saga vault disable --export-plaintext
saga bundle create --encrypted
```

Dangerous plaintext export and encryption disable flows require explicit
wording, warnings, and tests.
