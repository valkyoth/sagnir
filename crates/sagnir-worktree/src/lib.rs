#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathClass {
    TrackedCandidate,
    Control,
    Invalid,
}

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
    path == ".saga"
        || path.starts_with(".saga/")
        || path.contains("/.saga/")
        || path.ends_with("/.saga")
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
    fn windows_separator_control_path_is_invalid_by_policy() {
        assert_eq!(classify_relative_path(".saga\\objects"), PathClass::Invalid);
    }

    #[test]
    fn parent_escape_is_invalid() {
        assert_eq!(classify_relative_path("../outside"), PathClass::Invalid);
    }
}
