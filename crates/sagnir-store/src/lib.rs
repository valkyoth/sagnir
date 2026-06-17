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
#[non_exhaustive]
pub enum WalFrameKind {
    BeginTx,
    PutObject,
    PutFact,
    PutWorldState,
    UpdateAlias,
    CommitTx,
    AbortTx,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WalFrameChecksum(u32);

impl WalFrameChecksum {
    pub const fn new(raw: u32) -> Result<Self, sagnir_core::SagnirError> {
        if raw == 0 {
            return Err(sagnir_core::SagnirError::InvalidValue);
        }
        Ok(Self(raw))
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WalFrameHeader {
    kind: WalFrameKind,
    tx_id: u64,
    payload_len: u64,
    checksum: WalFrameChecksum,
}

impl WalFrameHeader {
    pub fn new(
        kind: WalFrameKind,
        tx_id: u64,
        payload: &[u8],
    ) -> Result<Self, sagnir_core::SagnirError> {
        let payload_len =
            u64::try_from(payload.len()).map_err(|_| sagnir_core::SagnirError::InvalidValue)?;
        let checksum = WalFrameChecksum::new(crc32c(payload))?;
        Ok(Self {
            kind,
            tx_id,
            payload_len,
            checksum,
        })
    }

    #[must_use]
    pub const fn checksum(self) -> WalFrameChecksum {
        self.checksum
    }

    #[must_use]
    pub fn verifies(self, payload: &[u8]) -> bool {
        u64::try_from(payload.len()).is_ok_and(|len| len == self.payload_len)
            && crc32c(payload) == self.checksum.get()
    }
}

#[must_use]
pub fn crc32c(payload: &[u8]) -> u32 {
    let mut crc = !0_u32;
    let mut index = 0;
    while index < payload.len() {
        crc ^= u32::from(payload[index]);
        let mut bit = 0;
        while bit < 8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0x82f6_3b78 & mask);
            bit += 1;
        }
        index += 1;
    }
    !crc
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

    #[test]
    fn wal_frame_header_checks_payload_integrity() {
        let header = WalFrameHeader::new(WalFrameKind::PutFact, 7, b"payload");

        assert!(header.is_ok_and(|value| value.verifies(b"payload")));
        assert!(
            !WalFrameHeader::new(WalFrameKind::PutFact, 7, b"payload")
                .is_ok_and(|value| value.verifies(b"payloae"))
        );
    }

    #[test]
    fn wal_frame_checksum_rejects_zero_sentinel() {
        assert_eq!(
            WalFrameChecksum::new(0),
            Err(sagnir_core::SagnirError::InvalidValue)
        );
    }
}
