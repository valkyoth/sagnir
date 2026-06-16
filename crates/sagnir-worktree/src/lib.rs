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
    if path == ".saga" || path.starts_with(".saga/") || path.contains("/.saga/") {
        return PathClass::Control;
    }
    if path
        .split('/')
        .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return PathClass::Invalid;
    }
    if path.as_bytes().contains(&b'\\') {
        return PathClass::Invalid;
    }
    PathClass::TrackedCandidate
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
    }

    #[test]
    fn parent_escape_is_invalid() {
        assert_eq!(classify_relative_path("../outside"), PathClass::Invalid);
    }
}
