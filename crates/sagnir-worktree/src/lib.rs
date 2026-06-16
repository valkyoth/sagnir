#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathClass {
    TrackedCandidate,
    Control,
    Invalid,
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
    if path
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
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
    fn windows_separator_control_path_is_invalid_by_policy() {
        assert_eq!(classify_relative_path(".saga\\objects"), PathClass::Invalid);
    }

    #[test]
    fn parent_escape_is_invalid() {
        assert_eq!(classify_relative_path("../outside"), PathClass::Invalid);
    }

    #[test]
    fn symlink_policy_is_not_handled_by_string_classification() {
        assert_eq!(
            classify_relative_path("src/link"),
            PathClass::TrackedCandidate
        );
    }
}
