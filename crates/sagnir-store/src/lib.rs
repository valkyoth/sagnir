#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub const STORE_DIR: &str = ".saga";
pub const FORMAT_FILE: &str = ".saga/FORMAT";
pub const CONFIG_FILE: &str = ".saga/config.toml";

pub const REQUIRED_DIRS: [&str; 12] = [
    ".saga/objects",
    ".saga/wal",
    ".saga/indexes",
    ".saga/worlds",
    ".saga/changes",
    ".saga/facts",
    ".saga/ops",
    ".saga/keys",
    ".saga/policies",
    ".saga/projections",
    ".saga/tmp",
    ".saga/locks",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WalFrameKind {
    BeginTx,
    PutObject,
    PutFact,
    PutWorldState,
    UpdateAlias,
    CommitTx,
    AbortTx,
}

#[must_use]
pub fn is_required_store_dir(path: &str) -> bool {
    REQUIRED_DIRS.contains(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_dir_is_saga() {
        assert_eq!(STORE_DIR, ".saga");
    }

    #[test]
    fn required_dirs_include_objects_and_wal() {
        assert!(is_required_store_dir(".saga/objects"));
        assert!(is_required_store_dir(".saga/wal"));
    }

    #[test]
    fn wal_frame_kind_has_explicit_abort() {
        assert_eq!(WalFrameKind::AbortTx, WalFrameKind::AbortTx);
    }
}
