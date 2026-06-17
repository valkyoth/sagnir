#!/usr/bin/env sh
set -eu

scripts/checks.sh
scripts/security_tool_gate.sh
cargo test -p sagnir-object
test -f fuzz/Cargo.toml
rg 'libfuzzer_sys::fuzz_target' fuzz/fuzz_targets/object_header_parse.rs >/dev/null
rg 'sagnir_object::parse_object_id' fuzz/fuzz_targets/object_id_parse.rs >/dev/null
rg 'sagnir_core::BoundedName::new' fuzz/fuzz_targets/bounded_name_parse.rs >/dev/null
rg 'sagnir_worktree::WorktreePath::new' fuzz/fuzz_targets/worktree_path_classify.rs >/dev/null
rg 'sagnir_codec::read_byte_string' fuzz/fuzz_targets/codec_byte_string_read.rs >/dev/null
cargo deny --manifest-path fuzz/Cargo.toml check --config fuzz/deny.toml
scripts/validate-release-notes.sh 0.8.0
scripts/validate-pentest-report.sh v0.8.0
scripts/validate-pentest-pass.sh v0.8.0
