#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/security_tool_gate.sh
test -f fuzz/Cargo.toml
rg 'libfuzzer_sys::fuzz_target' fuzz/fuzz_targets/object_header_parse.rs >/dev/null
cargo deny --manifest-path fuzz/Cargo.toml --config fuzz/deny.toml check
scripts/validate-release-notes.sh 0.6.0
scripts/validate-pentest-report.sh v0.6.0
scripts/validate-pentest-pass.sh v0.6.0
