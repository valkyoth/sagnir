use sagnir_core::{FormatVersion, SagnirError};

use crate::{ObjectType, parse_object_type};

pub const OBJECT_HEADER_MAGIC: [u8; MAGIC_LEN] = *b"SAGNOBJ\0";
pub const MAGIC_LEN: usize = 8;
pub const HEADER_LEN: usize = MAGIC_LEN + 2 + 2 + 8 + 4;
pub const OBJECT_BODY_BYTES_MAX: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectHeaderFlags(u32);

impl ObjectHeaderFlags {
    pub const NONE: Self = Self(0);
    const KNOWN_BITS: u32 = Self::NONE.0;

    pub const fn try_new(raw: u32) -> Result<Self, SagnirError> {
        if raw & !Self::KNOWN_BITS != 0 {
            return Err(SagnirError::InvalidValue);
        }
        Ok(Self(raw))
    }

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectHeader {
    object_type: ObjectType,
    format_version: FormatVersion,
    body_len: u64,
    flags: ObjectHeaderFlags,
}

impl ObjectHeader {
    pub const fn new(
        object_type: ObjectType,
        format_version: FormatVersion,
        body_len: u64,
        flags: ObjectHeaderFlags,
    ) -> Result<Self, SagnirError> {
        if body_len > OBJECT_BODY_BYTES_MAX {
            return Err(SagnirError::InvalidValue);
        }
        Ok(Self {
            object_type,
            format_version,
            body_len,
            flags,
        })
    }

    #[must_use]
    pub const fn object_type(self) -> ObjectType {
        self.object_type
    }

    #[must_use]
    pub const fn format_version(self) -> FormatVersion {
        self.format_version
    }

    #[must_use]
    pub const fn body_len(self) -> u64 {
        self.body_len
    }

    #[must_use]
    pub const fn flags(self) -> ObjectHeaderFlags {
        self.flags
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObjectHeaderField {
    Magic,
    ObjectType,
    FormatVersion,
    BodyLength,
    Flags,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObjectHeaderFields(u8);

impl ObjectHeaderFields {
    pub const NONE: Self = Self(0);
    const MAGIC: u8 = 1 << 0;
    const OBJECT_TYPE: u8 = 1 << 1;
    const FORMAT_VERSION: u8 = 1 << 2;
    const BODY_LENGTH: u8 = 1 << 3;
    const FLAGS: u8 = 1 << 4;
    const ALL: u8 =
        Self::MAGIC | Self::OBJECT_TYPE | Self::FORMAT_VERSION | Self::BODY_LENGTH | Self::FLAGS;

    pub const fn insert(self, field: ObjectHeaderField) -> Result<Self, SagnirError> {
        let bit = match field {
            ObjectHeaderField::Magic => Self::MAGIC,
            ObjectHeaderField::ObjectType => Self::OBJECT_TYPE,
            ObjectHeaderField::FormatVersion => Self::FORMAT_VERSION,
            ObjectHeaderField::BodyLength => Self::BODY_LENGTH,
            ObjectHeaderField::Flags => Self::FLAGS,
        };
        if self.0 & bit != 0 {
            return Err(SagnirError::InvalidValue);
        }
        Ok(Self(self.0 | bit))
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        self.0 == Self::ALL
    }
}

pub fn parse_object_header(input: &[u8]) -> Result<(ObjectHeader, &[u8]), SagnirError> {
    let magic = input.get(..MAGIC_LEN).ok_or(SagnirError::BufferTooSmall)?;
    if magic != OBJECT_HEADER_MAGIC {
        return Err(SagnirError::InvalidValue);
    }
    let rest = &input[MAGIC_LEN..];
    let (raw_type, rest) = sagnir_codec::read_u16(rest)?;
    let (raw_version, rest) = sagnir_codec::read_u16(rest)?;
    let (body_len, rest) = sagnir_codec::read_u64(rest)?;
    let (raw_flags, tail) = sagnir_codec::read_u32(rest)?;

    let object_type = parse_object_type(raw_type)?;
    let format_version = FormatVersion::try_new(raw_version)?;
    let flags = ObjectHeaderFlags::try_new(raw_flags)?;
    let header = ObjectHeader::new(object_type, format_version, body_len, flags)?;
    Ok((header, tail))
}

pub fn write_object_header(out: &mut [u8], header: ObjectHeader) -> Result<&mut [u8], SagnirError> {
    let out = sagnir_codec::write_bytes(out, &OBJECT_HEADER_MAGIC)?;
    let out = sagnir_codec::write_u16(out, object_type_raw(header.object_type()))?;
    let out = sagnir_codec::write_u16(out, header.format_version().get())?;
    let out = sagnir_codec::write_u64(out, header.body_len())?;
    sagnir_codec::write_u32(out, header.flags().bits())
}

const fn object_type_raw(object_type: ObjectType) -> u16 {
    match object_type {
        ObjectType::Blob => 1,
        ObjectType::Tree => 2,
        ObjectType::StateRoot => 3,
        ObjectType::Change => 4,
        ObjectType::ChangeRevision => 5,
        ObjectType::World => 6,
        ObjectType::Fact => 7,
        ObjectType::Operation => 8,
        ObjectType::Bundle => 9,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sagnir_core::FORMAT_VERSION;

    fn valid_header_bytes() -> [u8; HEADER_LEN] {
        let mut out = [0_u8; HEADER_LEN];
        let header = ObjectHeader {
            object_type: ObjectType::Blob,
            format_version: FORMAT_VERSION,
            body_len: 3,
            flags: ObjectHeaderFlags::NONE,
        };
        assert_eq!(
            write_object_header(&mut out, header).map(|tail| tail.len()),
            Ok(0)
        );
        out
    }

    #[test]
    fn object_header_round_trips_with_tail() {
        let mut bytes = [0_u8; HEADER_LEN + 1];
        bytes[..HEADER_LEN].copy_from_slice(&valid_header_bytes());
        bytes[HEADER_LEN] = 9;

        let parsed = parse_object_header(&bytes);

        assert_eq!(
            parsed,
            Ok((
                ObjectHeader {
                    object_type: ObjectType::Blob,
                    format_version: FORMAT_VERSION,
                    body_len: 3,
                    flags: ObjectHeaderFlags::NONE,
                },
                &b"\t"[..],
            ))
        );
    }

    #[test]
    fn object_header_rejects_short_buffer() {
        let bytes = valid_header_bytes();

        assert_eq!(
            parse_object_header(&bytes[..HEADER_LEN - 1]),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn object_header_rejects_wrong_magic() {
        let mut bytes = valid_header_bytes();
        bytes[0] = b'X';

        assert_eq!(parse_object_header(&bytes), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_header_rejects_unknown_type() {
        let mut bytes = valid_header_bytes();
        bytes[MAGIC_LEN..MAGIC_LEN + 2].copy_from_slice(&99_u16.to_le_bytes());

        assert_eq!(parse_object_header(&bytes), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_header_rejects_wrong_version() {
        let mut bytes = valid_header_bytes();
        bytes[MAGIC_LEN + 2..MAGIC_LEN + 4].copy_from_slice(&99_u16.to_le_bytes());

        assert_eq!(parse_object_header(&bytes), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_header_rejects_oversize_body_len() {
        let mut bytes = valid_header_bytes();
        bytes[MAGIC_LEN + 4..MAGIC_LEN + 12]
            .copy_from_slice(&(OBJECT_BODY_BYTES_MAX + 1).to_le_bytes());

        assert_eq!(parse_object_header(&bytes), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_header_rejects_unknown_flags() {
        let mut bytes = valid_header_bytes();
        bytes[MAGIC_LEN + 12..MAGIC_LEN + 16].copy_from_slice(&1_u32.to_le_bytes());

        assert_eq!(parse_object_header(&bytes), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn object_header_field_tracker_rejects_duplicates() {
        let fields = ObjectHeaderFields(ObjectHeaderFields::MAGIC);

        assert_eq!(
            fields.insert(ObjectHeaderField::Magic),
            Err(SagnirError::InvalidValue)
        );
    }

    #[test]
    fn object_header_field_tracker_detects_complete_header() {
        let fields = ObjectHeaderFields::NONE
            .insert(ObjectHeaderField::Magic)
            .and_then(|fields| fields.insert(ObjectHeaderField::ObjectType))
            .and_then(|fields| fields.insert(ObjectHeaderField::FormatVersion))
            .and_then(|fields| fields.insert(ObjectHeaderField::BodyLength))
            .and_then(|fields| fields.insert(ObjectHeaderField::Flags));

        assert_eq!(fields.map(ObjectHeaderFields::is_complete), Ok(true));
    }
}
