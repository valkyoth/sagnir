#!/usr/bin/env sh
set -eu

case "${1:-release}" in
    release|pentest-handoff) ;;
    *) exit 2 ;;
esac
scripts/checks.sh
scripts/validate-release-notes.sh 0.10.1
cargo check --locked --no-default-features -p sagnir-store
cargo check --locked --manifest-path fuzz/Cargo.toml --bins
cargo deny --manifest-path fuzz/Cargo.toml --config fuzz/deny.toml check
scripts/podman_smoke.sh
if [ "${1:-release}" = release ]; then
    scripts/validate-pentest-report.sh v0.10.1
    scripts/validate-pentest-pass.sh v0.10.1
fi
