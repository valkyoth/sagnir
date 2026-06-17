#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathClass {
    TrackedCandidate,
    Control,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorktreePath<'a> {
    relative: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SymlinkBoundaryProof {
    _private: (),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerifiedWorktreePath<'a> {
    relative: WorktreePath<'a>,
    _proof: SymlinkBoundaryProof,
}

impl SymlinkBoundaryProof {
    #[cfg(test)]
    const fn for_test() -> Self {
        Self { _private: () }
    }
}

impl<'a> WorktreePath<'a> {
    pub fn new(relative: &'a str) -> Result<Self, sagnir_core::SagnirError> {
        match classify_relative_path(relative) {
            PathClass::TrackedCandidate => Ok(Self { relative }),
            PathClass::Control | PathClass::Invalid => Err(sagnir_core::SagnirError::InvalidValue),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.relative
    }
}

impl<'a> VerifiedWorktreePath<'a> {
    #[must_use]
    pub const fn from_resolved(relative: WorktreePath<'a>, proof: SymlinkBoundaryProof) -> Self {
        Self {
            relative,
            _proof: proof,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.relative.as_str()
    }
}

/// Classifies a relative path by string content only.
///
/// Note: this function does not resolve symlinks. Callers performing
/// filesystem I/O must independently verify that resolved targets stay inside
/// the worktree boundary before reading or materializing a tracked candidate.
#[must_use]
pub fn classify_relative_path(path: &str) -> PathClass {
    if path.is_empty() || path.starts_with('/') || path.starts_with('\\') {
        return PathClass::Invalid;
    }
    if path.as_bytes().contains(&b'\\') {
        return PathClass::Invalid;
    }
    if is_control_path(path) {
        return PathClass::Control;
    }
    if path.split('/').any(|part| {
        part.is_empty()
            || part == "."
            || part == ".."
            || sagnir_core::is_dotfile_segment(part)
            || sagnir_core::has_windows_path_alias(part)
            || !part.bytes().all(sagnir_core::valid_name_byte_no_slash)
    }) {
        return PathClass::Invalid;
    }
    PathClass::TrackedCandidate
}

#[must_use]
pub fn is_control_path(path: &str) -> bool {
    path.split('/').any(sagnir_core::is_saga_segment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_relative_path_is_trackable() {
        assert_eq!(
            classify_relative_path("crates/sagnir-core/src/lib.rs"),
            PathClass::TrackedCandidate
        );
    }

    #[test]
    fn control_path_is_not_trackable() {
        assert_eq!(classify_relative_path(".saga/objects"), PathClass::Control);
        assert_eq!(classify_relative_path("sub/.saga"), PathClass::Control);
    }

    #[test]
    fn uppercase_control_path_is_not_trackable() {
        assert_eq!(classify_relative_path(".Saga/config"), PathClass::Control);
        assert_eq!(classify_relative_path(".SAGA/objects"), PathClass::Control);
        assert_eq!(classify_relative_path("sub/.Saga"), PathClass::Control);
        assert!(is_control_path(".Saga/config"));
    }

    #[test]
    fn non_control_dotfile_segments_are_invalid() {
        assert_eq!(classify_relative_path(".gitignore"), PathClass::Invalid);
        assert_eq!(
            classify_relative_path(".git/hooks/pre-commit"),
            PathClass::Invalid
        );
        assert_eq!(
            classify_relative_path(".ssh/authorized_keys"),
            PathClass::Invalid
        );
        assert_eq!(
            classify_relative_path(".github/workflows/ci.yml"),
            PathClass::Invalid
        );
        assert_eq!(classify_relative_path("src/.env"), PathClass::Invalid);
    }

    #[test]
    fn windows_reserved_device_names_are_invalid() {
        assert_eq!(classify_relative_path("CON"), PathClass::Invalid);
        assert_eq!(classify_relative_path("src/nul.txt"), PathClass::Invalid);
        assert_eq!(classify_relative_path("COM1"), PathClass::Invalid);
        assert_eq!(classify_relative_path("build/LPT9.log"), PathClass::Invalid);
    }

    #[test]
    fn trailing_dot_aliases_are_invalid() {
        assert_eq!(classify_relative_path("file."), PathClass::Invalid);
        assert_eq!(classify_relative_path("src/file."), PathClass::Invalid);
        assert_eq!(classify_relative_path(".saga."), PathClass::Invalid);
    }

    #[test]
    fn windows_separator_control_path_is_invalid_by_policy() {
        assert_eq!(classify_relative_path(".saga\\objects"), PathClass::Invalid);
    }

    #[test]
    fn parent_escape_is_invalid() {
        assert_eq!(classify_relative_path("../outside"), PathClass::Invalid);
    }

    #[test]
    fn control_characters_are_invalid() {
        assert_eq!(classify_relative_path("src/\0inject"), PathClass::Invalid);
        assert_eq!(classify_relative_path("src/\r\nfile"), PathClass::Invalid);
        assert_eq!(classify_relative_path("src/file\ttab"), PathClass::Invalid);
        assert_eq!(classify_relative_path("src/\u{1b}[31m"), PathClass::Invalid);
    }

    #[test]
    fn symlink_policy_is_not_handled_by_string_classification() {
        assert_eq!(
            classify_relative_path("src/link"),
            PathClass::TrackedCandidate
        );
    }

    #[test]
    fn worktree_path_rejects_untracked_strings() {
        assert!(WorktreePath::new("src/lib.rs").is_ok());
        assert_eq!(
            WorktreePath::new("../outside"),
            Err(sagnir_core::SagnirError::InvalidValue)
        );
        assert_eq!(
            WorktreePath::new(".saga/config"),
            Err(sagnir_core::SagnirError::InvalidValue)
        );
    }

    #[test]
    fn verified_worktree_path_requires_boundary_proof() {
        let path = WorktreePath::new("src/lib.rs");
        let verified = path.map(|path| {
            VerifiedWorktreePath::from_resolved(path, SymlinkBoundaryProof::for_test())
        });

        assert_eq!(verified.map(VerifiedWorktreePath::as_str), Ok("src/lib.rs"));
    }
}
