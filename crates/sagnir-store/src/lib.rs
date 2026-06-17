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
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
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
        let checksum = wal_frame_checksum(kind, tx_id, payload);
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
            && wal_frame_checksum(self.kind, self.tx_id, payload).get() == self.checksum.get()
    }
}

#[must_use]
pub fn crc32c(payload: &[u8]) -> u32 {
    !crc32c_update(!0_u32, payload)
}

#[must_use]
pub fn wal_frame_checksum(kind: WalFrameKind, tx_id: u64, payload: &[u8]) -> WalFrameChecksum {
    let mut crc = !0_u32;
    crc = crc32c_update(crc, &wal_frame_kind_raw(kind).to_le_bytes());
    crc = crc32c_update(crc, &tx_id.to_le_bytes());
    crc = crc32c_update(crc, payload);
    WalFrameChecksum::new(!crc)
}

const fn wal_frame_kind_raw(kind: WalFrameKind) -> u16 {
    match kind {
        WalFrameKind::BeginTx => 1,
        WalFrameKind::PutObject => 2,
        WalFrameKind::PutFact => 3,
        WalFrameKind::PutWorldState => 4,
        WalFrameKind::UpdateAlias => 5,
        WalFrameKind::CommitTx => 6,
        WalFrameKind::AbortTx => 7,
    }
}

fn crc32c_update(mut crc: u32, payload: &[u8]) -> u32 {
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
    crc
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
    fn wal_frame_header_handles_empty_payload() {
        let header = WalFrameHeader::new(WalFrameKind::BeginTx, 1, b"");

        assert!(header.is_ok_and(|value| value.verifies(b"")));
        assert_eq!(crc32c(b""), 0);
        assert_eq!(WalFrameChecksum::new(0).get(), 0);
    }

    #[test]
    fn wal_frame_checksum_binds_kind_and_transaction_id() {
        let checksum = wal_frame_checksum(WalFrameKind::PutFact, 7, b"payload");
        let forged_kind = WalFrameHeader {
            kind: WalFrameKind::AbortTx,
            tx_id: 7,
            payload_len: 7,
            checksum,
        };
        let forged_tx = WalFrameHeader {
            kind: WalFrameKind::PutFact,
            tx_id: 8,
            payload_len: 7,
            checksum,
        };

        assert!(!forged_kind.verifies(b"payload"));
        assert!(!forged_tx.verifies(b"payload"));
    }
}
