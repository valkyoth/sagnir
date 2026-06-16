#!/usr/bin/env sh
set -eu

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/check_doc_links.sh
scripts/validate-release-metadata.sh
scripts/validate-modularity-policy.sh check
scripts/validate-security-policy.sh check
scripts/check_latest_crates.sh
scripts/security_tool_gate.sh
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
