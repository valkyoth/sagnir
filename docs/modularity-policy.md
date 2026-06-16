# Sagnir Modularity Policy

Status: policy

Sagnir must not become a few huge source files.

Rules:

- use focused workspace crates for subsystems;
- keep `lib.rs` as module wiring, not implementation dumping ground;
- keep `main.rs` as CLI or daemon orchestration only;
- split parsing, validation, policy, state, I/O, and tests into separate
  modules;
- keep normal implementation files under 300 lines where practical;
- split non-generated `.rs` files before they exceed 500 lines unless a
  temporary exception is documented here.

The release gate runs `scripts/validate-modularity-policy.sh`.
