# Sagnir Protocol

Status: planning document

The Sagnir protocol moves proof-carrying bundles.

Sync flow:

1. Client announces local heads, worlds, and fact roots.
2. Remote answers with missing objects and facts.
3. Client sends a bundle.
4. Remote verifies hashes, object types, signatures, and facts.
5. Remote evaluates policy.
6. Remote accepts, denies, quarantines, or asks for more evidence.
7. Client records the remote decision locally.

This is the logical trusted/open flow. Private blind-storage mode replaces raw
head and fact-root announcement with encrypted or opaque set reconciliation and
does not disclose semantic commitments, private locators, duplicate
relationships, or private transparency metadata.

Local work must never require network access.

## Encrypted Sync

Encrypted realms support three planned sync modes:

- trusted remote: remote can decrypt only through admitted key policy;
- blind remote: remote stores encrypted packs and facts without source access;
- split-trust remote: remote sees approved proof summaries and redacted
  metadata while protected source remains encrypted.

Encrypted bundles must be verified before decrypt or import.

## Private Identity Reconciliation

Private locators are deterministic lookup values, not signed object identity.
One locator can name a bounded encrypted candidate bucket containing multiple
independently blinded semantic commitments created while peers were offline.
Sync preserves every signed commitment and reference. Optional encrypted
duplicate-equivalence evidence may guide future references, but it cannot
rewrite historical signatures or collapse different plaintext that happens to
share a keyed locator.

Authorized peers reconcile authenticated forward locator buckets and reverse
semantic-commitment indexes. Blind remotes receive neither locators, semantic
commitments, duplicate relationships, nor private transparency-monitor
metadata.

## Redaction Projection

Authorized peers exchange the private encrypted semantic tombstone before
requesting bodies. A blind remote instead receives an endpoint-scoped opaque
storage deletion or receipt-supersession notice for ciphertext IDs, pack
positions, and receipt IDs it already knows. The notice is authenticated by a
governance-admitted storage authority and reveals no semantic commitment, path,
actor, compartment, reason, or private causal history.

Backup, VM-snapshot, recovery-kit, archive, and air-gapped restore starts in
restricted mode. Protected decryption or materialization waits until the
restored state establishes the redaction frontier required by local policy.

Cryptographic erasure is a durable forward-only operation. Planning and
preparation may abort with signed evidence before key destruction. Once
`KeysDestroyed` commits, Sagnir cannot roll back or recreate the targeted key;
it can only continue storage notices, controlled-copy cleanup, replacement
packs, acknowledgement collection, or residual-copy reporting.

For a mixed-content pack, an independently deletable record may be removed only
when the pack and provider both advertise that capability. Otherwise the client
uploads a verified privacy-padded replacement pack, obtains required receipts,
commits encrypted index and receipt lineage, and only then asks the provider to
delete the old pack. Failed upload or receipt acquisition preserves the old pack
for live-record availability while the redacted record remains undecryptable.
