# Sagnir Security Controls

Status: baseline control map

| Area | Control | Current Status | Evidence |
| --- | --- | --- | --- |
| Toolchain | Rust stable `1.96.0` pinned | Active | `rust-toolchain.toml` |
| Release arithmetic | Release profile keeps overflow checks enabled | Active | `Cargo.toml` |
| Core runtime | Trusted library crates are `no_std` where practical | Active | crate roots |
| Dependency policy | License, source, wildcard, and advisory checks | Configured | `deny.toml` |
| Security reporting | Private-first vulnerability handling | Configured | `SECURITY.md` |
| Unsafe code | Forbidden in trusted scaffold | Active | crate roots and `scripts/validate-security-policy.sh` |
| Hardcoded credentials | Targeted scan rejects common credential, token, key, PEM private key, and JWT literal patterns | Active | `scripts/validate-security-policy.sh` |
| Modularity | Focused crates and file-size gate | Active | `docs/modularity-policy.md` |
| Canonical identity | Object type is part of identity metadata and object graph references are checked before persistence with bounded iterative traversal | Scaffolded | `sagnir-object` |
| Object ID hashing | `TypedId` and `ObjectId` map users must keep Rust's randomized default hasher or an audited keyed hasher for attacker-influenced sets | Policy | `sagnir-core`, `sagnir-object` |
| Local store | `.saga/` layout, WAL frame kinds, and WAL CRC-32C metadata bound to frame kind, transaction ID, and payload for crash-corruption detection | Scaffolded | `sagnir-store` |
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
| Private object IDs | Sealed private mode avoids public plaintext hash membership leaks | Scaffolded | `sagnir-object`, `docs/vault-encryption.md` |
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
