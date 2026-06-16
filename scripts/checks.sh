#!/usr/bin/env sh
set -eu

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/check_doc_links.sh
scripts/validate-release-metadata.sh
scripts/validate-modularity-policy.sh check
scripts/validate-security-policy.sh check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

if command -v cargo-deny >/dev/null 2>&1; then
    cargo deny check
else
    echo "cargo-deny not installed; skipping local dependency policy check" >&2
fi

if command -v cargo-audit >/dev/null 2>&1; then
    cargo audit
else
    echo "cargo-audit not installed; skipping local advisory check" >&2
fi
