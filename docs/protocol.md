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
One locator can name authenticated bounded pages containing multiple
independently blinded semantic commitments created while peers were offline.
Sync preserves every admitted commitment and reference without requiring one
unbounded bucket allocation. Canonical per-replica creation quotas and
duplicate-amplification evidence limit an authorized replica; over-quota
candidates are refused before admission or kept in authenticated quarantine,
never silently removed after admission.

Optional encrypted duplicate-equivalence evidence may guide future references,
but it cannot rewrite historical signatures or collapse different plaintext
that happens to share a keyed locator. Representative transitions compare and
swap the expected equivalence root and prior representative. Concurrent choices
remain explicit conflict heads until an authorized multi-parent resolution;
arrival order and attacker-controlled blinded values never choose the winner.

Authorized peers reconcile authenticated forward locator buckets and reverse
semantic-commitment indexes. Blind remotes receive neither locators, semantic
commitments, duplicate relationships, nor private transparency-monitor
metadata.

## Cross-Compartment Movement

Compartment identity is committed into sealed-private semantic identity.
Copying or moving content to another compartment therefore creates a signed
transition with a new target semantic commitment, private locator, encryption
instance, DEK, selector, and target-policy evaluation. It is not an ordinary
rename.

The target becomes durable and verifiable before a move records logical removal
from the source. Historical source references remain unchanged. Removing or
cryptographically erasing the source ciphertext is a separate redaction
operation with its own authority, retention, backup, and residual-copy checks.

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

Cryptographic erasure stores a monotonic operation phase plus orthogonal
per-provider and cleanup results. Planning and preparation may abort with signed
evidence before destructive dispatch. Sagnir commits `DestroyingKeys` intent
and provider idempotency tokens before issuing a filesystem, KMS, HSM, escrow,
wrapper, or recovery-share destruction request.

A crash after provider success but before local confirmation produces
`DestructionUncertain`, not success or failure. Recovery queries the provider
with the original token and fails closed while any path is unresolved.
`KeysDestroyed` commits only after durable admitted evidence covers every
recovery path. From `DestroyingKeys` onward, Sagnir cannot roll back the
tombstone or recreate possibly destroyed key material.

Events concurrent with redaction remain verifiable historical references but
resolve the old instance as `RedactedBody`. Later ordering, merge, replay,
receipt, or repair cannot resurrect it. Authorized reintroduction creates a new
encryption instance, DEK, selectors, and current policy decision.

For a mixed-content pack, an independently deletable record may be removed only
when the pack and provider both advertise that capability. Otherwise the client
uploads a verified privacy-padded replacement pack, obtains required receipts,
commits encrypted index and receipt lineage, and only then asks the provider to
delete the old pack. Failed upload or receipt acquisition preserves the old pack
for live-record availability while the redacted record remains undecryptable.
