# Sagnir

Sagnir is an open source source-state engine, object format, proof model, local
store, and protocol.

The command-line interface is `saga`.

Sagnir is local-first. A developer must be able to initialize, inspect, seal,
prove, promote, bundle, and sync source state without any hosted service or
external database.

## Current Status

Status: v0.1.0 foundation scaffold

This repository currently contains:

- Rust 2024 workspace pinned to Rust stable `1.96.0`.
- EUPL-1.2 license baseline.
- Focused crates for core IDs, canonical encoding, objects, store metadata,
  worktree rules, changes, worlds, facts, policy, crypto envelopes, proofs,
  sync bundles, the `saga` CLI, and the future `sagad` daemon.
- Planning for native encrypted realms through `saga encrypt project`,
  `saga unlock`, `saga lock`, encrypted bundles, recipient keys, and future
  compartment encryption.
- `no_std` trusted library scaffolds where practical.
- Security, modularity, release, and implementation planning docs.
- Local check scripts for format, lint, tests, docs, security policy, and
  modularity policy.
- Rootless Podman build scaffold.

## Build

```bash
cargo check --workspace
cargo test --workspace
cargo run -p sagnir-cli --bin saga -- version
```

## Local Gate

```bash
scripts/checks.sh
```

## Documentation

- [Implementation Plan](docs/IMPLEMENTATION_PLAN.md)
- [Version Plan](docs/VERSION_PLAN.md)
- [Architecture](docs/architecture.md)
- [Command Design](docs/command-design.md)
- [Vault Encryption](docs/vault-encryption.md)
- [Security Controls](docs/security-controls.md)
- [Container Image Policy](docs/container-image-policy.md)
- [Threat Model](docs/threat-model.md)
- [Toolchain Policy](docs/toolchain-policy.md)
- [Modularity Policy](docs/modularity-policy.md)
- [Unsafe Policy](docs/unsafe-policy.md)

## License

Sagnir is licensed under the European Union Public Licence 1.2.
