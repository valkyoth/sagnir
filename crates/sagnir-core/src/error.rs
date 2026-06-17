#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SagnirError {
    EmptyName,
    NameTooLong,
    InvalidNameByte,
    BufferTooSmall,
    InvalidValue,
}

impl core::fmt::Display for SagnirError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyName => f.write_str("name is empty"),
            Self::NameTooLong => f.write_str("name exceeds length limit"),
            Self::InvalidNameByte => f.write_str("name contains an invalid byte"),
            Self::BufferTooSmall => f.write_str("buffer too small"),
            Self::InvalidValue => f.write_str("invalid value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::format;

    #[test]
    fn display_messages_are_controlled() {
        assert_eq!(format!("{}", SagnirError::EmptyName), "name is empty");
        assert_eq!(
            format!("{}", SagnirError::NameTooLong),
            "name exceeds length limit"
        );
        assert_eq!(
            format!("{}", SagnirError::InvalidNameByte),
            "name contains an invalid byte"
        );
        assert_eq!(
            format!("{}", SagnirError::BufferTooSmall),
            "buffer too small"
        );
        assert_eq!(format!("{}", SagnirError::InvalidValue), "invalid value");
    }
}
