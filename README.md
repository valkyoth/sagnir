<p align="center">
  <b>Open source source-state engine, object format, proof model, local store, and protocol.</b><br>
  Intent first. Evidence backed. Local first. Built for proof-carrying source-state work.
</p>

<div align="center">
  <a href="docs/IMPLEMENTATION_PLAN.md">Implementation Plan</a>
  -
  <a href="docs/VERSION_PLAN.md">Version Plan</a>
  -
  <a href="docs/causal-memory.md">Causal Memory</a>
  -
  <a href="SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <img src="./.github/images/sagnir.webp" alt="Sagnir overview">
</p>

# Sagnir

Sagnir is an open source source-state engine, object format, proof model, local
store, and protocol.

The command-line interface is `saga`.

Sagnir is not a Git rewrite. It is a local-first system for intent, changes,
evidence, world transitions, and artifacts. The 1.0 target is a serious
production-ready CLI that can initialize a local realm, inspect source state,
seal changes, record evidence, verify proofs, promote worlds, protect encrypted
realms, build bundles, and sync without requiring a hosted service or external
database.

Sagnir is licensed under the European Union Public Licence 1.2.

## What Works Today

### Repository Foundation

| Capability | Status | Notes |
| --- | --- | --- |
| Rust workspace | Active | Rust 2024 workspace pinned to Rust stable `1.96.0`. |
| License baseline | Active | EUPL-1.2. |
| CLI scaffold | Active | `saga version` reports the CLI and Sagnir format version. |
| Focused crates | Active | Core, codec, object, store, worktree, change, world, fact, policy, crypto, proof, sync, CLI, and daemon scaffolds. |
| `no_std` trusted crates | Active | Core library scaffolds use `#![no_std]` where practical. |
| Unsafe policy | Active | Trusted crates forbid unsafe Rust. |
| Modularity policy | Active | File-size and module-boundary checks prevent oversized implementation files. |

### Security And Release Gates

| Capability | Status | Notes |
| --- | --- | --- |
| Local check gate | Active | `scripts/checks.sh` runs formatting, docs, metadata, modularity, security policy, dependency policy, lint, and tests. |
| Dependency policy | Active | `cargo deny check` and `cargo audit` are required through `scripts/security_tool_gate.sh`. |
| Pentest stop rule | Active | Release gates refuse to tag until the matching permanent pentest report is `Status: PASS`. |
| Release notes validation | Active | Release notes must use the Sagnir release-note shape. |
| Pentest report validation | Active | Permanent pentest reports must include status, commit, tester, date, scope, and notes. |
| Container base pinning | Active | Rootless container build paths pin base images by digest. |
| CI supply-chain hardening | Active | GitHub Actions checkout and security-tool install versions are pinned. |
| CodeQL | Repository setting | GitHub CodeQL default setup must be enabled in repository security settings. |

### Source-State Model

| Capability | Status | Notes |
| --- | --- | --- |
| Core IDs and bounds | Scaffolded | Typed IDs, bounded names, case-folded `.saga` control-path rejection, and constant-time equality APIs for sensitive IDs. |
| Canonical codec | Scaffolded | Bounded scalar encoding helpers. |
| Object identity | Scaffolded | Domain-separated object types and fail-closed hash algorithm parsing. |
| Local store metadata | Scaffolded | `.saga/` layout and WAL frame kind scaffolds. |
| Worktree path rules | Scaffolded | Control-path exclusion, path traversal rejection, separator policy, and symlink-boundary documentation. |
| Policy metadata | Scaffolded | Policy results, validated obligation bitmasks, and named obligation checks. |
| Crypto envelope metadata | Scaffolded | Algorithm admission, signature bounds, redacted debug output, and constant-time signature equality API. |
| Bundle metadata | Scaffolded | Bundle manifest counts are bounded before future parser allocation paths. |

### Planned Core Tracks

| Track | Status | Target |
| --- | --- | --- |
| Canonical local store | Planned | Durable objects, WAL recovery, local fsck, and rebuildable indexes. |
| Worktree and worlds | Planned | `saga status`, `saga diff`, world open/list/switch, and dirty-worktree protection. |
| Changes and sealing | Planned | Intent-first changes, immutable revisions, amend chains, and operation ledger. |
| Proofs and promotion | Planned | Offline object proofs, local policy files, deterministic promotion preflight, and rollback preflight. |
| Causal memory | Planned | Events, facts, causal indexes, explanations, context packs, `saga why`, `saga explain`, `saga trace`, `saga impact`, and bounded `saga ask`. |
| Native encrypted realms | Planned | `saga encrypt project`, `saga unlock`, `saga lock`, encrypted local storage, recipient slots, rekeying, leak scanning, and future compartments. |
| Bundles and sync | Planned | Proof-carrying bundles, encrypted bundles, blind/split-trust sync modes, and optional `sagad` remote support. |
| Production hardening | Planned | Malicious corpora, expanded fuzz/model tests, cross-platform gates, rootless Podman release gates, SBOMs, and 1.0 release evidence. |

## Why Sagnir

- **Intent first**: a change starts with intent, not just a file delta.
- **Evidence backed**: tests, reviews, policy decisions, proofs, and facts are
  first-class release inputs.
- **World based**: source state moves through named worlds by proof and policy,
  not by destructive history mutation.
- **Causal memory**: Sagnir is designed to explain why a change happened, what
  proved it, what trusted it, and what depends on it.
- **Local first**: useful source-state work must not require a hosted service.
- **Security first**: parsers, bundles, worktree paths, release gates, and
  supply-chain inputs are treated as hostile until verified.
- **Modular Rust**: focused crates keep implementation boundaries testable and
  prevent thousand-line core files.

## Quick Start

Build the workspace:

```bash
cargo build --workspace
```

Run the tests:

```bash
cargo test --workspace
```

Run the CLI scaffold:

```bash
cargo run -p sagnir-cli --bin saga -- version
```

Run the normal local gate:

```bash
scripts/checks.sh
```

Run the security tool gate directly:

```bash
scripts/security_tool_gate.sh
```

Run the rootless Podman smoke path:

```bash
scripts/podman_smoke.sh
```

## Current Release Line

The repository is past `v0.1.0` and is currently working through the `v0.2.0`
release-gate baseline.

Current release discipline:

- implementation reaches a clean version stop;
- the exact commit is handed to pentest;
- root `PENTEST.md` is scratch input only and must not be committed;
- findings are fixed before tag;
- permanent reports live under `security/pentest/`;
- release gates require `Status: PASS` before tagging;
- tags are created only after explicit maintainer instruction.

## Documentation

- [Implementation Plan](docs/IMPLEMENTATION_PLAN.md)
- [Version Plan](docs/VERSION_PLAN.md)
- [Architecture](docs/architecture.md)
- [Command Design](docs/command-design.md)
- [Causal Memory](docs/causal-memory.md)
- [Object Format](docs/object-format.md)
- [Local Store](docs/local-store.md)
- [World Model](docs/world-model.md)
- [Proof Model](docs/proof-model.md)
- [Vault Encryption](docs/vault-encryption.md)
- [Protocol](docs/protocol.md)
- [Security Controls](docs/security-controls.md)
- [Supply-Chain Security](docs/supply-chain-security.md)
- [Container Image Policy](docs/container-image-policy.md)
- [Threat Model](docs/threat-model.md)
- [Toolchain Policy](docs/toolchain-policy.md)
- [Modularity Policy](docs/modularity-policy.md)
- [Unsafe Policy](docs/unsafe-policy.md)
- [Release Runbook](docs/release-runbook.md)

## License

Sagnir is licensed under the European Union Public Licence 1.2.
