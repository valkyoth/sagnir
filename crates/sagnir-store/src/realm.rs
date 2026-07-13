use sagnir_core::{ID_BYTES, RealmId};

use crate::metadata::{MetadataWriter, StoreMetadataError, assignment, decimal, quoted, set_once};

pub const REALM_FILE: &str = ".saga/realm.toml";
pub const REALM_TEMP_FILE: &str = ".saga/realm.toml.tmp";
pub const REALM_FILE_MAX: usize = 256;
pub const REALM_ID_PREFIX: &str = "sagnir-realm-v1:";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealmMetadata {
    id: RealmId,
}

impl RealmMetadata {
    pub fn new(id: RealmId) -> Result<Self, StoreMetadataError> {
        if id.bytes() == [0_u8; ID_BYTES] {
            return Err(StoreMetadataError::InvalidRealmId);
        }
        Ok(Self { id })
    }

    #[must_use]
    pub const fn id(self) -> RealmId {
        self.id
    }
}

pub fn parse_realm_toml(input: &str) -> Result<RealmMetadata, StoreMetadataError> {
    if input.len() > REALM_FILE_MAX {
        return Err(StoreMetadataError::ValueOutOfRange);
    }

    let mut format = None;
    let mut id = None;
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let (key, value) = assignment(line)?;
        match key {
            "format" => set_once(&mut format, decimal(value)?)?,
            "realm_id" => set_once(&mut id, parse_realm_id(quoted(value)?)?)?,
            _ => return Err(StoreMetadataError::UnknownField),
        }
    }

    let format = format.ok_or(StoreMetadataError::MissingField)?;
    if format != 1 {
        return Err(StoreMetadataError::InvalidValue);
    }
    RealmMetadata::new(id.ok_or(StoreMetadataError::MissingField)?)
}

pub fn write_realm_toml(
    metadata: RealmMetadata,
    output: &mut [u8],
) -> Result<usize, StoreMetadataError> {
    let mut writer = MetadataWriter::new(output);
    writer.push("format = 1\nrealm_id = \"")?;
    writer.push(REALM_ID_PREFIX)?;
    for byte in metadata.id().bytes() {
        let encoded = [hex_digit(byte >> 4), hex_digit(byte & 0x0f)];
        let text = core::str::from_utf8(&encoded).map_err(|_| StoreMetadataError::InvalidValue)?;
        writer.push(text)?;
    }
    writer.push("\"\n")?;
    Ok(writer.len())
}

pub fn parse_realm_id(value: &str) -> Result<RealmId, StoreMetadataError> {
    let Some(hex) = value.strip_prefix(REALM_ID_PREFIX) else {
        return Err(StoreMetadataError::InvalidRealmId);
    };
    if hex.len() != ID_BYTES * 2 {
        return Err(StoreMetadataError::InvalidRealmId);
    }

    let mut bytes = [0_u8; ID_BYTES];
    for (index, pair) in hex.as_bytes().chunks_exact(2).enumerate() {
        let high = hex_value(pair[0]).ok_or(StoreMetadataError::InvalidRealmId)?;
        let low = hex_value(pair[1]).ok_or(StoreMetadataError::InvalidRealmId)?;
        bytes[index] = (high << 4) | low;
    }
    RealmMetadata::new(RealmId::new(bytes)).map(RealmMetadata::id)
}

const fn hex_digit(value: u8) -> u8 {
    match value {
        0..=9 => b'0' + value,
        _ => b'a' + (value - 10),
    }
}

const fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::format;

    #[test]
    fn realm_metadata_round_trips() {
        let metadata = RealmMetadata {
            id: RealmId::new([0x5a; ID_BYTES]),
        };
        let mut output = [0_u8; REALM_FILE_MAX];
        let encoded = format!(
            "format = 1\nrealm_id = \"{REALM_ID_PREFIX}{}\"\n",
            "5a".repeat(ID_BYTES)
        );

        assert_eq!(write_realm_toml(metadata, &mut output), Ok(encoded.len()));
        assert_eq!(&output[..encoded.len()], encoded.as_bytes());
        assert_eq!(parse_realm_toml(&encoded), Ok(metadata));
        assert!(encoded.contains("sagnir-realm-v1:5a5a"));
    }

    #[test]
    fn realm_parser_rejects_invalid_ids() {
        let zero = "00".repeat(ID_BYTES);
        let uppercase = "AA".repeat(ID_BYTES);
        assert_eq!(
            parse_realm_toml(&format!(
                "format = 1\nrealm_id = \"{REALM_ID_PREFIX}{zero}\"\n"
            )),
            Err(StoreMetadataError::InvalidRealmId)
        );
        assert_eq!(
            parse_realm_toml(&format!(
                "format = 1\nrealm_id = \"{REALM_ID_PREFIX}{uppercase}\"\n"
            )),
            Err(StoreMetadataError::InvalidRealmId)
        );
    }

    #[test]
    fn realm_parser_rejects_unknown_duplicate_and_missing_fields() {
        let id = "11".repeat(ID_BYTES);
        let valid_id = format!("realm_id = \"{REALM_ID_PREFIX}{id}\"\n");
        assert_eq!(
            parse_realm_toml(&format!("format = 1\nformat = 1\n{valid_id}")),
            Err(StoreMetadataError::DuplicateField)
        );
        assert_eq!(
            parse_realm_toml(&format!("format = 1\nunknown = 1\n{valid_id}")),
            Err(StoreMetadataError::UnknownField)
        );
        assert_eq!(
            parse_realm_toml("format = 1\n"),
            Err(StoreMetadataError::MissingField)
        );
    }
}
