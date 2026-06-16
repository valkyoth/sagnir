# Sagnir Supply-Chain Security

Status: policy

Sagnir treats dependency updates, build scripts, CI edits, release scripts,
procedural macros, and native build dependencies as executable supply-chain
changes.

Cryptographic provider changes are high-risk supply-chain changes. A provider
for password hashing, AEAD encryption, key wrapping, hardware keys, OS keychain
integration, or post-quantum algorithms must have explicit admission notes,
license review, maintenance review, tests, and release-note coverage.

Rules:

- use crates.io releases unless a documented exception is approved;
- deny unknown registries and unknown git sources;
- deny wildcard dependency requirements;
- keep license exceptions narrow;
- check latest stable crate versions before adding or updating dependencies;
- run `scripts/security_tool_gate.sh` before release;
- install CI security tools from checksum-verified crate archives;
- update release notes for dependency changes;
- add tests for behavior introduced by a dependency.

The scaffold uses local timing-hardened byte comparison helpers only to prevent
accidental use of derived equality in future verification paths. Before live
signature verification, HMAC verification, AEAD tag checks, or secret-dependent
proof checks rely on constant-time behavior, Sagnir must admit `subtle` or an
equivalent formally specified primitive through this policy.

The security-policy validator rejects known crypto-provider crates in
`Cargo.lock` unless `subtle` and `zeroize` are also admitted. This keeps
constant-time comparison and zero-on-drop policy admission ahead of live crypto
implementation.

The hardcoded credential scanner supports `scanner:allow` on intentionally
non-secret placeholder lines. Use it only for test fixtures or documentation
examples, and keep the surrounding value obviously non-production.

The initial scaffold has no third-party Rust dependencies.
