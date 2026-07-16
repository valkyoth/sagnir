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
uses bounded candidate buckets, while an authenticated encrypted reverse index
maps each semantic commitment directly to its locator epoch and ciphertext
record. Optional private duplicate-equivalence evidence can guide future
references but cannot rewrite historical signatures or references.

Blind stores receive neither semantic commitments nor private locators. For the
1.0 design, locator translation uses an encrypted authenticated mapping and
authenticated-index evidence, not a public zero-knowledge proof. This prevents
outsiders from checking whether known plaintext appears in an encrypted realm
by comparing public hashes or commitments.

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

## Erasure State

The durable operation uses `Planned`, `Prepared`, `TombstoneCommitted`,
`KeysDestroyed`, `StorageNoticesPending`, `ControlledCopiesCleared`,
`Complete`, and `ResidualCopiesKnown`. `KeysDestroyed` is the irreversible
point. Before it, a signed abort can record that destruction did not occur.
After it, recovery is forward-only. Remote acknowledgements and controlled-copy
clearance may finish in either order; the top-level state is derived from their
monotonic component results rather than treating them as one reversible linear
sequence.

Status must report local cryptographic erasure, controlled-backup clearance,
remote deletion acknowledgements, and uncontrolled residual copies separately.
`Complete` means configured obligations are satisfied; it never means Sagnir
recalled plaintext or keys copied beyond its control.

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
