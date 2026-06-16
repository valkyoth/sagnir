use crate::SagnirError;

pub const NAME_MAX_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedName<'a> {
    value: &'a str,
}

impl<'a> BoundedName<'a> {
    /// Creates a bounded name from path-like ASCII-safe input.
    ///
    /// Platform note: `.saga` path segments are rejected with ASCII
    /// case-folding so case-insensitive filesystems cannot alias Sagnir
    /// control paths into source-state names.
    pub fn new(value: &'a str) -> Result<Self, SagnirError> {
        if value.is_empty() {
            return Err(SagnirError::EmptyName);
        }
        if value.len() > NAME_MAX_BYTES {
            return Err(SagnirError::NameTooLong);
        }
        if !value.bytes().all(valid_name_byte) {
            return Err(SagnirError::InvalidNameByte);
        }
        if value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == ".." || is_saga_segment(part))
        {
            return Err(SagnirError::InvalidNameByte);
        }
        Ok(Self { value })
    }

    #[must_use]
    pub const fn as_str(self) -> &'a str {
        self.value
    }
}

#[must_use]
pub const fn valid_name_byte(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'/' | b'.')
}

#[must_use]
pub const fn valid_name_byte_no_slash(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.')
}

/// Returns true if `segment` is the `.saga` control directory name, using
/// ASCII case-folding.
///
/// Platform note: Windows NTFS strips trailing dots at the Win32 API level, so
/// `.saga.` can resolve to `.saga` on that boundary but is not caught here.
/// Full Windows path normalization is deferred to the v0.15.0 worktree path
/// scanner milestone.
#[must_use]
pub fn is_saga_segment(segment: &str) -> bool {
    segment.eq_ignore_ascii_case(".saga")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_name_accepts_world_path() {
        let name = BoundedName::new("draft/wal-recovery");
        assert_eq!(name.map(BoundedName::as_str), Ok("draft/wal-recovery"));
    }

    #[test]
    fn bounded_name_rejects_empty() {
        assert_eq!(BoundedName::new(""), Err(SagnirError::EmptyName));
    }

    #[test]
    fn bounded_name_rejects_oversize_name() {
        let oversize = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        assert_eq!(oversize.len(), NAME_MAX_BYTES + 1);
        assert_eq!(BoundedName::new(oversize), Err(SagnirError::NameTooLong));
    }

    #[test]
    fn bounded_name_rejects_path_traversal_segments() {
        assert_eq!(
            BoundedName::new("../outside"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("draft/../main"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("draft//main"),
            Err(SagnirError::InvalidNameByte)
        );
    }

    #[test]
    fn bounded_name_rejects_case_folded_saga_control_path() {
        assert_eq!(
            BoundedName::new(".Saga/config"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("sub/.SAGA"),
            Err(SagnirError::InvalidNameByte)
        );
    }

    #[test]
    fn bounded_name_rejects_non_admitted_bytes() {
        assert_eq!(
            BoundedName::new("draft space"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("draft/\u{1b}[31m"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(
            BoundedName::new("draft/å"),
            Err(SagnirError::InvalidNameByte)
        );
    }

    #[test]
    fn valid_name_byte_no_slash_rejects_slash() {
        assert!(valid_name_byte(b'/'));
        assert!(!valid_name_byte_no_slash(b'/'));
    }

    #[test]
    fn is_saga_segment_documents_trailing_dot_windows_gap() {
        assert!(!is_saga_segment(".saga."));
        assert!(!is_saga_segment(".SAGA."));
    }
}
