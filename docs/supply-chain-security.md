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

Sagnir admits `subtle` for constant-time byte comparison before live signature
verification, HMAC verification, AEAD tag checks, or secret-dependent proof
checks rely on those paths.

The security-policy validator rejects known crypto-provider crates in
`Cargo.lock` unless `subtle` and `sanitization` are also admitted. This keeps
constant-time comparison and explicit memory sanitization policy admission
ahead of live crypto implementation. `zeroize` is not admitted in Sagnir; use
`sanitization`, which is `no_std` by default and follows the project's security
review process.

The hardcoded credential scanner supports `scanner:allow` only in documentation,
release notes, or reviewed fixtures. It is rejected in trusted code, scripts,
CI configuration, root metadata, and root documentation. Use it only for
intentionally non-secret placeholder lines, and keep the surrounding value
obviously non-production.

Current admitted third-party Rust dependencies must remain narrowly scoped,
current, license-reviewed, and covered by release notes when they change.
`getrandom` is admitted in `sagnir-cli` for one purpose: obtaining
cross-platform operating-system entropy for new realm IDs. Realm identity
creation must fail rather than fall back to timestamps, process IDs, or a
pseudorandom generator when the operating-system source is unavailable.

Highest-assurance deployments should build release tooling from a pinned,
attested Rust distribution or compare installed `cargo deny` and `cargo audit`
outputs against independently reproducible builds. The repository release gate
verifies crate archive checksums and locked versions; toolchain provenance is an
environment control.
