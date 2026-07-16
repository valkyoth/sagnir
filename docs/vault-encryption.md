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

Blind stores receive neither semantic commitments nor private locators. For the
1.0 design, locator translation uses an encrypted authenticated mapping and
authenticated-index evidence, not a public zero-knowledge proof. This prevents
outsiders from checking whether known plaintext appears in an encrypted realm
by comparing public hashes or commitments.

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
