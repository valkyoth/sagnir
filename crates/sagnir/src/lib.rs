#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub use sagnir_change as change;
pub use sagnir_codec as codec;
pub use sagnir_core as core;
pub use sagnir_crypto as crypto;
pub use sagnir_fact as fact;
pub use sagnir_object as object;
pub use sagnir_policy as policy;
pub use sagnir_proof as proof;
pub use sagnir_store as store;
pub use sagnir_sync as sync;
pub use sagnir_worktree as worktree;
pub use sagnir_world as world;

#[must_use]
pub const fn format_version() -> u16 {
    core::FORMAT_VERSION.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_crate_exposes_format_version() {
        assert_eq!(format_version(), 1);
    }
}
