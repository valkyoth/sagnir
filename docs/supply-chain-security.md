# Sagnir Supply-Chain Security

Status: policy

Sagnir treats dependency updates, build scripts, CI edits, release scripts,
procedural macros, and native build dependencies as executable supply-chain
changes.

Rules:

- use crates.io releases unless a documented exception is approved;
- deny unknown registries and unknown git sources;
- keep license exceptions narrow;
- check latest stable crate versions before adding or updating dependencies;
- run `cargo deny check`;
- run `cargo audit`;
- update release notes for dependency changes;
- add tests for behavior introduced by a dependency.

The initial scaffold has no third-party Rust dependencies.
