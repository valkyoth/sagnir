# Sagnir Security Controls

Status: baseline control map

| Area | Control | Current Status | Evidence |
| --- | --- | --- | --- |
| Toolchain | Rust stable `1.96.0` pinned | Active | `rust-toolchain.toml` |
| Release arithmetic | Release profile keeps overflow checks enabled | Active | `Cargo.toml` |
| Core runtime | Trusted library crates are `no_std` where practical | Active | crate roots |
| Dependency policy | License, source, and advisory checks | Configured | `deny.toml` |
| Security reporting | Private-first vulnerability handling | Configured | `SECURITY.md` |
| Unsafe code | Forbidden in trusted scaffold | Active | crate roots and `scripts/validate-security-policy.sh` |
| Modularity | Focused crates and file-size gate | Active | `docs/modularity-policy.md` |
| Canonical identity | Object type is part of identity metadata | Scaffolded | `sagnir-object` |
| Local store | `.saga/` layout and WAL frame kinds | Scaffolded | `sagnir-store` |
| Worktree safety | Control paths and traversal are rejected | Scaffolded | `sagnir-worktree` |
| Policy | Aggregate policy decision type | Scaffolded | `sagnir-policy` |
| Proof | Verification report type | Scaffolded | `sagnir-proof` |
| Crypto agility | Signature algorithm and envelope metadata | Scaffolded | `sagnir-crypto` |
| Signature bounds | Empty and oversized signatures rejected | Scaffolded | `sagnir-crypto` |
| Native encrypted realms | Encrypted `.saga/` storage, lock/unlock, recipient wrapping, crypto epochs, and leak scanning | Planned | `docs/vault-encryption.md` |
| Private object IDs | Sealed private mode avoids public plaintext hash membership leaks | Planned | `docs/vault-encryption.md` |
| Encrypted bundles | Recipient-targeted bundles and blind/split-trust sync modes | Planned | `docs/vault-encryption.md`, `docs/protocol.md` |
| Facts | Confidence bounds and fact kinds | Scaffolded | `sagnir-fact` |
| Blast radius | Local causal traversal for taint and quarantine | Planned | `docs/IMPLEMENTATION_PLAN.md` |
| Bundles | Bundle manifest and protocol metadata | Scaffolded | `sagnir-sync` |
| Rootless container | Podman CLI image scaffold | Scaffolded | `Containerfile` |

## Admission Rule

Security-sensitive features do not graduate from planned to active until they
have tests, documentation, failure-mode analysis, and release-gate coverage.
