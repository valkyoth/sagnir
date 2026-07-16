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
The 1.0 candidate is one persistent authenticated B+ tree keyed by
`(locator_epoch, private_locator, semantic_commitment,
encryption_instance_id)`. Every policy-separated instance is its own leaf key,
so inclusion and absence remain logarithmic. It freezes only after the
unique-representation gate passes; canonical ordering, node encoding,
split/merge rules, and logical-root derivation then become protocol formats
rather than implementation choices.

The deterministic keyed logical root is visible only to authorized peers.
Randomized encrypted node envelopes authenticate that logical identity, and
blind stores address those envelopes by public ciphertext storage IDs. Equal
logical entry sets converge to one private logical root without requiring equal
ciphertext.

Logical leaves contain stable candidate identity and object kind only.
Ciphertext IDs, pack generations, receipts, and positions live in encrypted
placement/reverse indexes. Re-encryption, repacking, receipt renewal, and
relocation therefore change placement roots without changing the logical root.
The logical tree is admitted only if its normalization is history-independent:
all operation permutations for one canonical entry set produce one root within
declared amplification bounds. Otherwise the format must use a reviewed uniquely
represented structure before compatibility freezes.

A dedicated domain- and epoch-bound index-commitment key computes logical node
commitments. Root authority comes from a signed checkpointed manifest binding
the logical root to semantic state, policy, membership, and structure version,
not from possession of that key.

The semantic ledger has one versioned canonical projection to complete forward
and reverse logical entries. Manifests bind full-rebuild or deterministic delta
proofs, and full-view verifiers replay them before acceptance. A malicious
authorized signer cannot make an omission or invented entry valid by signature
alone. Compartment-only recipients verify inclusion, continuity, and required
full-view signer/witness policy, while explicitly relying on those actors for
hidden-compartment completeness.

For 1.0, a full-rebuild proof is an authenticated deterministic replay
certificate, not a succinct proof. It commits the ledger range/chunks,
projection evaluator, resource bounds, independently reproduced roots/counts,
and transcript. Verification requires canonical ledger availability and
execution. Delta certificates are replayable transitions with hard
chain/byte/work limits and mandatory rebuild cadence.

Projection witnesses sign exact semantic/index roots, checkpoint, evaluator,
replay transcript, policy, and declared replay mode. Full-replay and
evidence-validation witnesses are distinct assurance levels. Threshold policy
requires independent principals and administrative domains; key compromise,
revocation, equivocation, Sybil enrollment, or unavailable threshold fails
closed.

An encryption-instance ID is a domain-separated hash over its realm, opaque
compartment or neutral handle, semantic commitment, erasure unit, preallocated
creation-operation ID, and an independent 256-bit random nonce. The signed
creation transition binds all inputs. Rewrap, re-encryption, repack, receipt
renewal, and relocation preserve the ID; a new erasure unit, incompatible
policy copy, redaction reintroduction, or erased-instance replacement requires a
new ID.

The creation-operation ID is a domain-separated hash over realm, actor/device,
replica/incarnation, monotonic reservation sequence, independent random nonce,
operation kind, and intended domain. Its authenticated reservation is durable
before use. It becomes consumed by one exact idempotent transition or
authenticated-cancelled; abandoned, expired, crashed, cancelled, or uncertain
reservations are never reused.

Each compartment has one logical root and commitment-key epoch. A fixed-depth or
equivalently count-hiding authenticated realm manifest composes opaque
compartment-root references. Scoped proofs let a compartment-only recipient
verify inclusion and continuity without learning other compartment identities,
counts, names, or tree shape.

Opaque compartment handles are keyed, nonce-bound, and epoch-versioned.
Collisions never alias compartments. Rotation is a signed realm-manifest
transition with encrypted old/new mapping; it limits future correlation but
cannot recall relationships already observed.

Canonical logical manifests are shared realm state. Placement and
reverse-resolution manifests are scoped to one replica incarnation, device, or
storage endpoint. Repacking, relocation, receipt renewal, and re-encryption
modify only that projection. Sync compares logical roots and reconciles
placement separately; one endpoint projection never overwrites another by
arrival order.

Aggregate actor/device quotas use signed escrow rights assigned to replica
incarnations. Offline replicas consume only locally held rights; rights transfer
through causal compare-and-swap transitions. Merge detects double-spend or
aggregate overdraw and keeps the candidate and dependent transitions in
authenticated quota-conflict quarantine.

Duplicate-semantic-identity rights are separate from bounded
encryption-instance-fanout rights. Adding a policy-separated instance to an
existing commitment consumes no duplicate-identity right, but offline creation
must possess separately governed instance capacity.

Retirement reclaims remaining rights only from signed surrender, a stability
acknowledgement committing the final spent-right root, or an explicit cutoff
that rejects every unseen spend after the grace period. Otherwise the rights are
burned. Governance cannot attest that a missing replica did not spend offline.
A later quota increase creates a new ratification/admission transition and
re-evaluates dependents; the original event remains evidence of its initial
quarantine. Private quota topology and activity metadata remain encrypted from
blind stores and ordinary diagnostics.

Quota expiry is bound to causal/checkpoint frontiers or an admitted timestamp
authority. A provably pre-expiry spend may arrive later; an offline candidate
that cannot prove pre-expiry creation is quarantined. Local clock rollback,
skew, or delivery delay never extends a right.

Authoritative time and revocation use signed monotonic statements, append-only
checkpoints and consistency proofs, governed authority keys, explicit quorum or
diversity policy, and bounded offline freshness. Required unavailable,
conflicting, stale, revoked, or equivocal authority state yields unknown or
quarantine. Requests use opaque handles and configured batching/padding rather
than disclosing source paths or object identities.

Optional encrypted duplicate-equivalence evidence may guide future references,
but it cannot rewrite historical signatures or collapse different plaintext
that happens to share a keyed locator. Representative transitions compare and
swap the expected equivalence root and prior representative. Concurrent choices
remain explicit conflict heads until an authorized multi-parent resolution;
arrival order and attacker-controlled blinded values never choose the winner.

Authorized peers reconcile authenticated forward locator search roots and
reverse semantic-commitment indexes. Blind remotes receive neither locators,
semantic commitments, duplicate relationships, nor private transparency-
monitor metadata.

## Cross-Compartment Movement

Compartment identity is committed into sealed-private semantic identity.
Copying or moving content to another compartment therefore creates a signed
Merkle-chunked recursive graph-translation transition with new target semantic
commitments, private locators, encryption instances, DEKs, selectors, and
target-policy evaluation for every compartment-bound reachable descendant.
Leaf proofs establish authenticated content equality. Tree/container proofs
establish structural isomorphism under the manifest mapping and commit every
transformed metadata field; rewritten containers are not claimed byte-identical.
Shared subgraphs translate once per compatible target-policy domain, and the
target root proves that no source-compartment identity remains reachable.

The complete bridge manifest is encrypted only to actors authorized for both
compartments or a governed audit role. Target-only recipients receive a minimal
target-scoped attestation without source identifiers or graph relationships;
source-only recipients receive no target identifiers; blind stores receive
neither mapping. Repeated translations cannot expose a stable public correlation
handle.

The target-only attestation is an admitted issuer's revocable, expiring,
audience- and replay-bound assertion that the target transition/root was
accepted under a named policy. It is not independent cryptographic proof of
hidden source equality or isomorphism. Policies requiring that stronger claim
must grant bridge/opening access or wait for an admitted post-1.0 hidden-witness
proof system.

Current target-attestation validity requires a stapled issuer checkpoint,
revocation root, and consistency proof within policy freshness. An offline
verifier can claim only the state covered by its latest admitted staple.

The transition compares and swaps the expected source frontier/root, target
absence or admitted replacement, and target policy root. Stale source, occupied
target, or policy change produces explicit multi-head conflict. The target
becomes durable and verifiable before a move records logical removal from the
source. Historical source references remain unchanged, and source reviews or
proofs do not automatically authorize target identities. Removing or
cryptographically erasing source ciphertext is a separate redaction operation.

Translation fetches and verifies every required promised body and commitment
opening. Missing, unavailable, or redacted descendants cannot be treated as
equivalent target content. Only an explicitly compartment-neutral typed opaque
boundary may remain untranslated. Construction and verification are bounded,
resumable, cancellable, GC-pinned, and authoritative only at the atomic final
manifest/root commit. Neutral identity uses a separate commitment domain with
allowlisted types, neutral-only references, dedicated locator, commitment,
encryption, wrapping, recipient and recovery key lifecycles, and no
compartment-bound metadata. Reuse intentionally creates disclosed
cross-compartment linkability.

A canonical `RedactedPlaceholder` is a non-content target object with encrypted
audit provenance. It exposes no source commitment to target-only recipients,
makes no equivalence claim, cannot satisfy availability, completeness, repair,
build/test input, or proof obligations, and cannot be replaced by stale
ciphertext. Reintroduction requires a new authorized transition and encryption
instance.

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

Destruction evidence is a canonical envelope authenticated by a provider
signature, hardware attestation, or governed local key agent and binds the exact
operation, provider/key epoch, key or slot, idempotency token, request
transcript, result, assurance level, and checkpoint. TLS caching, unsigned API
results, file absence, or logs are not transferable proof.

The full evidence envelope and provider/key/timing metadata remain encrypted
inside authorized semantic views. Blind stores and ordinary logs, telemetry,
filenames, crash output, or locked status receive no evidence ID or stable
correlator. Selective disclosure creates a separate audience- and purpose-bound
minimal statement.

Deleting or overwriting a local DEK wrapper is not erasure while a surviving
parent key can decrypt recovered journal, snapshot, CoW, or media remnants.
Sagnir requires an independently destroyable erasure-unit KEK/key slot or a
complete parent wrapping-epoch rotation with surviving-DEK rewrap and old-epoch
destruction evidence.

Permanent ambiguity may close operationally only as signed
`ResidualUncertainty`. It remains non-abortable, retains the tombstone and
evidence, never claims erasure, and can advance later only if valid evidence
arrives.

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
