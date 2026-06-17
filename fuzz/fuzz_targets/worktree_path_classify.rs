#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        let _ = sagnir_worktree::classify_relative_path(input);
        let _ = sagnir_worktree::WorktreePath::new(input);
    }
});
