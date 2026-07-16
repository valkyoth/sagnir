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
uses a persistent content-addressed authenticated B-tree, trie, or equivalent
search structure with committed key ranges and logarithmic inclusion and
absence proofs. An authenticated encrypted reverse index maps each semantic
commitment directly to its locator epoch and ciphertext record.

Per-replica and aggregate actor/device canonical quotas plus
duplicate-amplification detection prevent an authorized principal from
exhausting a locator through new replicas or locator rotation. Authenticated
quota state carries across replica incarnations and locator epochs. Local
resource limits may quarantine unadmitted candidates but cannot discard
admitted search nodes.

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
compartment-bound reachable descendant. A signed manifest binds source and
target commitments, references, locators, encryption instances, DEKs, selectors
and shared-subgraph mappings, and proves the target reaches no source-
compartment identity.

The transition compares and swaps the expected source frontier/root, expected
target absence or replacement, and target policy root. Concurrency produces an
explicit conflict rather than arrival-order overwrite. A copy preserves the
source. A move records source logical removal only after the complete target
graph and encrypted indexes are durable. Source reviews, proofs, and approvals
do not automatically authorize target identities. Destroying source DEKs
requires a separate redaction operation.

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
