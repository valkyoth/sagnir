#!/usr/bin/env sh
set -eu

tmp=$(mktemp "${TMPDIR:-/tmp}/sagnir-init-dry-run.XXXXXXXXXX")
trap 'rm -f "$tmp"' EXIT

scripts/checks.sh
scripts/security_tool_gate.sh
cargo test -p sagnir-store
cargo test -p sagnir-cli
cargo run -p sagnir-cli --bin saga -- init --dry-run >"$tmp"
rg '^  \.saga$' "$tmp" >/dev/null
rg '^  \.saga/FORMAT$' "$tmp" >/dev/null
rg '^No changes written\.$' "$tmp" >/dev/null
scripts/validate-release-notes.sh 0.9.0
rg '^version = "0\.9\.0"$' Cargo.toml >/dev/null
scripts/validate-pentest-report.sh v0.9.0
scripts/validate-pentest-pass.sh v0.9.0
