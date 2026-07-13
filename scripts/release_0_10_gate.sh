#!/usr/bin/env sh
set -eu

tmp=$(mktemp "${TMPDIR:-/tmp}/sagnir-init-dry-run.XXXXXXXXXX")
trap 'rm -f "$tmp"' EXIT

scripts/checks.sh
scripts/security_tool_gate.sh
cargo test -p sagnir-store
cargo test -p sagnir-cli
cargo check --manifest-path fuzz/Cargo.toml --bins
cargo deny --manifest-path fuzz/Cargo.toml --config fuzz/deny.toml check
cargo run -p sagnir-cli --bin saga -- init --dry-run >"$tmp"
rg '^  \.saga$' "$tmp" >/dev/null
rg '^  \.saga/FORMAT$' "$tmp" >/dev/null
rg '^  \.saga/realm\.toml$' "$tmp" >/dev/null
rg '^  \.saga/config\.toml$' "$tmp" >/dev/null
rg '^No changes written\.$' "$tmp" >/dev/null
scripts/validate-release-notes.sh 0.10.0
rg '^version = "0\.10\.0"$' Cargo.toml >/dev/null
rg '^channel = "1\.97\.0"$' rust-toolchain.toml >/dev/null
rg '^sanitization = \{ version = "1\.2\.4", default-features = false \}$' \
    crates/sagnir-crypto/Cargo.toml >/dev/null
rg '^getrandom = \{ version = "0\.4\.3", default-features = false \}$' \
    crates/sagnir-cli/Cargo.toml >/dev/null
scripts/validate-pentest-report.sh v0.10.0
scripts/validate-pentest-pass.sh v0.10.0
