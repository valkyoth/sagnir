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
        if value.split('/').any(has_windows_path_alias) {
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
#[must_use]
pub fn is_saga_segment(segment: &str) -> bool {
    segment.eq_ignore_ascii_case(".saga")
}

#[must_use]
pub fn has_windows_path_alias(segment: &str) -> bool {
    segment.ends_with('.') || is_windows_reserved_name(segment)
}

#[must_use]
pub fn is_windows_reserved_name(segment: &str) -> bool {
    let stem = match segment.as_bytes().iter().position(|byte| *byte == b'.') {
        Some(index) => &segment[..index],
        None => segment,
    };

    stem.eq_ignore_ascii_case("CON")
        || stem.eq_ignore_ascii_case("PRN")
        || stem.eq_ignore_ascii_case("AUX")
        || stem.eq_ignore_ascii_case("NUL")
        || is_windows_reserved_numbered_device(stem, "COM")
        || is_windows_reserved_numbered_device(stem, "LPT")
}

fn is_windows_reserved_numbered_device(stem: &str, prefix: &str) -> bool {
    let bytes = stem.as_bytes();
    bytes.len() == 4 && stem[..3].eq_ignore_ascii_case(prefix) && matches!(bytes[3], b'1'..=b'9')
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
        assert_eq!(
            BoundedName::new("sub/.SAGA."),
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
    fn bounded_name_rejects_windows_reserved_device_names() {
        assert_eq!(BoundedName::new("CON"), Err(SagnirError::InvalidNameByte));
        assert_eq!(
            BoundedName::new("dir/nul.txt"),
            Err(SagnirError::InvalidNameByte)
        );
        assert_eq!(BoundedName::new("COM1"), Err(SagnirError::InvalidNameByte));
        assert_eq!(
            BoundedName::new("lpt9.log"),
            Err(SagnirError::InvalidNameByte)
        );
    }

    #[test]
    fn bounded_name_rejects_trailing_dot_aliases() {
        assert_eq!(BoundedName::new("file."), Err(SagnirError::InvalidNameByte));
        assert_eq!(
            BoundedName::new("dir/file."),
            Err(SagnirError::InvalidNameByte)
        );
    }

    #[test]
    fn windows_reserved_name_detector_is_case_insensitive() {
        assert!(is_windows_reserved_name("con"));
        assert!(is_windows_reserved_name("Con.txt"));
        assert!(is_windows_reserved_name("LPT1"));
        assert!(!is_windows_reserved_name("COM0"));
        assert!(!is_windows_reserved_name("COM10"));
        assert!(!is_windows_reserved_name("config"));
    }
}
