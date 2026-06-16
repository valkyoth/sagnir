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
- keep license exceptions narrow;
- check latest stable crate versions before adding or updating dependencies;
- run `scripts/security_tool_gate.sh` before release;
- update release notes for dependency changes;
- add tests for behavior introduced by a dependency.

The initial scaffold has no third-party Rust dependencies.
