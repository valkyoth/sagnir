# Sagnir Security Controls

Status: baseline control map

| Area | Control | Current Status | Evidence |
| --- | --- | --- | --- |
| Toolchain | Rust stable `1.97.1` pinned | Active | `rust-toolchain.toml` |
| Release arithmetic | Release profile keeps overflow checks enabled | Active | `Cargo.toml` |
| Core runtime | Trusted library crates are `no_std` where practical | Active | crate roots |
| Dependency policy | License, source, wildcard, and advisory checks | Configured | `deny.toml` |
| Security reporting | Private-first vulnerability handling | Configured | `SECURITY.md` |
| Unsafe code | Forbidden in trusted scaffold | Active | crate roots and `scripts/validate-security-policy.sh` |
| Hardcoded credentials | Targeted scan rejects common credential, token, key, PEM private key, and JWT literal patterns | Active | `scripts/validate-security-policy.sh` |
| Modularity | Focused crates and file-size gate | Active | `docs/modularity-policy.md` |
| Canonical identity | Object type is part of identity metadata and object graph references are checked before persistence with bounded iterative traversal | Scaffolded | `sagnir-object` |
| Verification scale | Large-world verification uses bounded chunks, changed-cone traversal, cached proofs, and explicit resource budgets for full-world mode | Planned | `docs/IMPLEMENTATION_PLAN.md`, `docs/VERSION_PLAN.md` |
| Remote trust preflight | Clone, bundle import, and sync compare remote verification requirements with local budgets before trust or materialization | Planned | `docs/IMPLEMENTATION_PLAN.md`, `docs/VERSION_PLAN.md` |
| Verifiable archival | Future compressed archive packs retain immutable receipts and root commitments before any cold-history pruning | Planned | `docs/IMPLEMENTATION_PLAN.md`, `docs/VERSION_PLAN.md` |
| Object ID hashing | `TypedId` and `ObjectId` map users must keep Rust's randomized default hasher or an audited keyed hasher for attacker-influenced sets | Policy | `sagnir-core`, `sagnir-object` |
| Local store init | Unix `.saga/` layout creation, portable dry-run planning, fail-closed unsupported backends, canonical root refusal, retained root/store handles, ownership and attachment checks, no-follow traversal, bounded complete reads, OS file locking, and interrupted-init temp cleanup | Active | `sagnir-store`, `sagnir-cli` |
| Realm identity | Nonzero 256-bit realm IDs come from the operating-system random source and use strict lowercase canonical encoding | Active | `sagnir-store`, `sagnir-cli` |
| Local configuration | Allocation-free bounded parsing rejects unknown, duplicate, malformed, oversized, and out-of-range realm/config metadata | Active | `sagnir-store` |
| Metadata persistence | Owner-only temporary files remain open through handle-relative rename; device/inode checks verify the temporary and committed file, while file sync, directory sync, regular-file checks, and hard-link refusal protect realm/config writes | Active on Unix | `sagnir-cli`, `docs/local-store.md` |
| Local store | WAL frame kinds and WAL CRC-32C metadata bound to frame kind, transaction ID, and payload for crash-corruption detection | Scaffolded | `sagnir-store` |
| WAL authentication | WAL data must not gate security decisions, network sync, or trusted replay until keyed frame authentication or encrypted frame authentication is implemented | Planned | `sagnir-store`, `docs/local-store.md` |
| Worktree safety | Control paths, traversal, separators, unsafe path bytes, and unverified symlink boundaries are rejected before source-state I/O | Scaffolded | `sagnir-worktree` |
| Policy | Aggregate policy decision type | Scaffolded | `sagnir-policy` |
| Proof | Verification report type | Scaffolded | `sagnir-proof` |
| Crypto agility | Signature algorithm and envelope metadata | Scaffolded | `sagnir-crypto` |
| Algorithm admission | Unknown hash and signature algorithms fail closed at parse boundaries | Scaffolded | `sagnir-object`, `sagnir-crypto` |
| Signature bounds | Empty and algorithm-oversized signatures rejected | Scaffolded | `sagnir-crypto` |
| Signature storage | `OwnedSignature` inline stack budget is documented and compile-time guarded | Scaffolded | `sagnir-crypto` |
| Hybrid signatures | Hybrid signature composition must bind classical and post-quantum components | Scaffolded | `sagnir-crypto`, `docs/signature-policy.md` |
| Redacted debug output | Signature envelopes, typed IDs, and object IDs redact sensitive bytes in `Debug` output | Scaffolded | `sagnir-core`, `sagnir-object`, `sagnir-crypto` |
| Native encrypted realms | Encrypted `.saga/` storage, lock/unlock, recipient wrapping, crypto epochs, and leak scanning | Planned | `docs/vault-encryption.md` |
| Sealed-private identity | Immutable confidential semantic commitments are separated from rotatable private locators and ciphertext storage IDs; blind-store metadata must expose none of the semantic commitment or translation mapping | Planned | `sagnir-object`, `docs/vault-encryption.md`, `docs/VERSION_PLAN.md` |
| Invitation lifecycle | Realm invitations have governed issuance, scoped acceptance, one-time or bounded use, expiry, revocation, supersession, and replay protection | Planned | `docs/VERSION_PLAN.md` |
| Key transparency | Recipient and device keys use canonical authenticated-map inclusion, absence, consistency, checkpoint, monitor, and split-view semantics | Planned | `docs/VERSION_PLAN.md` |
| Emergency recovery | Threshold recovery follows an end-to-end ceremony that protects shares, advances epochs, and rejects stale authority | Planned | `docs/VERSION_PLAN.md` |
| Redaction propagation | Signed tombstones and distinct `RedactedBody` state prevent sync, repair, receipts, or archival from resurrecting erased encryption instances | Planned | `docs/VERSION_PLAN.md` |
| Release provenance | Release artifacts, checksums, SBOMs, and provenance attestations bind to the exact source, signed tag, toolchain, dependency lock, target, and release-gate result | Planned | `docs/VERSION_PLAN.md` |
| Security boundary documentation | Every changed parser, trust, crypto, persistence, privilege, network, disclosure, recovery, or release-signing boundary updates its threat model and control-map evidence before release | Policy | `docs/VERSION_PLAN.md` |
| Encrypted bundles | Recipient-targeted bundles and blind/split-trust sync modes | Planned | `docs/vault-encryption.md`, `docs/protocol.md` |
| Facts | Confidence bounds and fact kinds | Scaffolded | `sagnir-fact` |
| Events | Bounded command events are separated from authoritative facts | Planned | `docs/causal-memory.md` |
| Fact compiler | Stable facts are derived deterministically from admitted inputs | Planned | `docs/causal-memory.md` |
| Explanation objects | Explanations cite evidence, redactions, and missing facts | Planned | `docs/causal-memory.md` |
| Context packs | Diagnostic and optional AI context is bounded and redacted | Planned | `docs/causal-memory.md` |
| AI boundary | AI may summarize evidence but cannot create authority or override policy | Planned | `docs/causal-memory.md` |
| Blast radius | Local causal traversal for taint and quarantine | Planned | `docs/IMPLEMENTATION_PLAN.md` |
| Bundles | Bundle manifest and protocol metadata | Scaffolded | `sagnir-sync` |
| Rootless container | Podman CLI image scaffold | Scaffolded | `Containerfile` |
| Container digest pinning | Release images must pin base images by digest before publication | Planned | `docs/container-image-policy.md` |
| CI security tools | Security tools are installed from checksum-verified crate archives | Active | `scripts/install_security_tools.sh` |

## Admission Rule

Security-sensitive features do not graduate from planned to active until they
have tests, documentation, failure-mode analysis, and release-gate coverage.
