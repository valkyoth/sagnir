use sagnir_core::{FormatVersion, ID_BYTES, SagnirError, StateId, constant_time_bytes_choice};

pub const OBJECT_ID_PREFIX: &str = "sagnir-object-v1";
pub const OBJECT_ID_DIGEST_HEX_LEN: usize = ID_BYTES * 2;
pub const OBJECT_ID_MAX_LEN: usize = OBJECT_ID_PREFIX.len()
    + 1
    + OBJECT_TYPE_NAME_MAX_LEN
    + 1
    + HASH_ALGORITHM_NAME_MAX_LEN
    + 1
    + OBJECT_ID_DIGEST_HEX_LEN;
pub const OBJECT_TYPE_NAME_MAX_LEN: usize = 15;
pub const HASH_ALGORITHM_NAME_MAX_LEN: usize = 8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum HashAlgorithm {
    Sha256,
    Sha3_256,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ObjectType {
    Blob,
    Tree,
    StateRoot,
    Change,
    ChangeRevision,
    World,
    Fact,
    Operation,
    Bundle,
}

#[derive(Clone, Copy, Eq)]
pub struct ObjectId {
    algorithm: HashAlgorithm,
    object_type: ObjectType,
    digest: [u8; ID_BYTES],
}

impl core::fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ObjectId")
            .field("algorithm", &self.algorithm)
            .field("object_type", &self.object_type)
            .field(
                "digest",
                &format_args!("[{} bytes redacted]", self.digest.len()),
            )
            .finish()
    }
}

impl core::hash::Hash for ObjectId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.algorithm, state);
        core::hash::Hash::hash(&self.object_type, state);
        core::hash::Hash::hash(&self.digest, state);
    }
}

impl PartialEq for ObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(other)
    }
}

pub const fn parse_hash_algorithm(raw: u16) -> Result<HashAlgorithm, SagnirError> {
    match raw {
        1 => Ok(HashAlgorithm::Sha256),
        2 => Ok(HashAlgorithm::Sha3_256),
        _ => Err(SagnirError::UnknownAlgorithm),
    }
}

pub fn parse_hash_algorithm_name(name: &str) -> Result<HashAlgorithm, SagnirError> {
    match name.as_bytes() {
        b"sha256" => Ok(HashAlgorithm::Sha256),
        b"sha3-256" => Ok(HashAlgorithm::Sha3_256),
        _ => Err(SagnirError::UnknownAlgorithm),
    }
}

pub const fn parse_object_type(raw: u16) -> Result<ObjectType, SagnirError> {
    match raw {
        1 => Ok(ObjectType::Blob),
        2 => Ok(ObjectType::Tree),
        3 => Ok(ObjectType::StateRoot),
        4 => Ok(ObjectType::Change),
        5 => Ok(ObjectType::ChangeRevision),
        6 => Ok(ObjectType::World),
        7 => Ok(ObjectType::Fact),
        8 => Ok(ObjectType::Operation),
        9 => Ok(ObjectType::Bundle),
        _ => Err(SagnirError::UnknownObjectType),
    }
}

pub fn parse_object_type_name(name: &str) -> Result<ObjectType, SagnirError> {
    match name.as_bytes() {
        b"blob" => Ok(ObjectType::Blob),
        b"tree" => Ok(ObjectType::Tree),
        b"state-root" => Ok(ObjectType::StateRoot),
        b"change" => Ok(ObjectType::Change),
        b"change-revision" => Ok(ObjectType::ChangeRevision),
        b"world" => Ok(ObjectType::World),
        b"fact" => Ok(ObjectType::Fact),
        b"operation" => Ok(ObjectType::Operation),
        b"bundle" => Ok(ObjectType::Bundle),
        _ => Err(SagnirError::UnknownObjectType),
    }
}

impl ObjectId {
    #[must_use]
    pub(crate) const fn new(
        algorithm: HashAlgorithm,
        object_type: ObjectType,
        digest: [u8; ID_BYTES],
    ) -> Self {
        Self {
            algorithm,
            object_type,
            digest,
        }
    }

    #[doc(hidden)]
    /// Builds an object ID from externally supplied digest bytes.
    ///
    /// Trust model: this only admits algorithm/type/length shape. Callers must
    /// already have verified that the digest was computed from canonical object
    /// bytes for the same domain.
    pub fn from_digest_slice(
        algorithm: HashAlgorithm,
        object_type: ObjectType,
        digest: &[u8],
    ) -> Result<Self, SagnirError> {
        if digest.len() != digest_len(algorithm) {
            return Err(SagnirError::InvalidValue);
        }

        let mut admitted = [0_u8; ID_BYTES];
        admitted.copy_from_slice(digest);
        Ok(Self::new(algorithm, object_type, admitted))
    }

    #[must_use]
    pub const fn object_type(self) -> ObjectType {
        self.object_type
    }

    #[must_use]
    pub const fn digest(self) -> [u8; ID_BYTES] {
        self.digest
    }

    #[must_use]
    pub const fn algorithm(self) -> HashAlgorithm {
        self.algorithm
    }

    /// Timing-hardened digest equality for verification scaffolds. Before live
    /// signature or HMAC verification relies on this path, Sagnir must admit a
    /// formally specified constant-time primitive.
    #[must_use]
    pub fn ct_eq(&self, other: &Self) -> bool {
        let algorithm_eq = constant_time_bytes_choice(
            &hash_algorithm_raw(self.algorithm).to_le_bytes(),
            &hash_algorithm_raw(other.algorithm).to_le_bytes(),
        );
        let object_type_eq = constant_time_bytes_choice(
            &object_type_raw(self.object_type).to_le_bytes(),
            &object_type_raw(other.object_type).to_le_bytes(),
        );
        let digest_eq = constant_time_bytes_choice(&self.digest, &other.digest);

        (algorithm_eq & object_type_eq & digest_eq).into()
    }
}

impl core::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}:{}:{}:",
            OBJECT_ID_PREFIX,
            object_type_name(self.object_type),
            hash_algorithm_name(self.algorithm)
        )?;

        for byte in self.digest {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RedactedObjectId<'a>(&'a ObjectId);

impl ObjectId {
    #[must_use]
    pub const fn redacted(&self) -> RedactedObjectId<'_> {
        RedactedObjectId(self)
    }
}

impl core::fmt::Display for RedactedObjectId<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}:{}:{}:[redacted]",
            OBJECT_ID_PREFIX,
            object_type_name(self.0.object_type),
            hash_algorithm_name(self.0.algorithm)
        )
    }
}

impl core::str::FromStr for ObjectId {
    type Err = SagnirError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_object_id(input)
    }
}

pub fn parse_object_id(input: &str) -> Result<ObjectId, SagnirError> {
    if input.len() > OBJECT_ID_MAX_LEN {
        return Err(SagnirError::InvalidValue);
    }

    let mut fields = input.split(':');
    let prefix = fields.next().ok_or(SagnirError::InvalidValue)?;
    let object_type = fields.next().ok_or(SagnirError::InvalidValue)?;
    let algorithm = fields.next().ok_or(SagnirError::InvalidValue)?;
    let digest = fields.next().ok_or(SagnirError::InvalidValue)?;

    if fields.next().is_some() || prefix != OBJECT_ID_PREFIX {
        return Err(SagnirError::InvalidValue);
    }

    let object_type = parse_object_type_name(object_type)?;
    let algorithm = parse_hash_algorithm_name(algorithm)?;
    let digest = parse_digest_hex(digest, algorithm)?;

    Ok(ObjectId::new(algorithm, object_type, digest))
}

const fn hash_algorithm_raw(algorithm: HashAlgorithm) -> u16 {
    match algorithm {
        HashAlgorithm::Sha256 => 1,
        HashAlgorithm::Sha3_256 => 2,
    }
}

#[must_use]
pub const fn digest_len(algorithm: HashAlgorithm) -> usize {
    match algorithm {
        HashAlgorithm::Sha256 => ID_BYTES,
        HashAlgorithm::Sha3_256 => ID_BYTES,
    }
}

#[must_use]
pub const fn hash_algorithm_name(algorithm: HashAlgorithm) -> &'static str {
    match algorithm {
        HashAlgorithm::Sha256 => "sha256",
        HashAlgorithm::Sha3_256 => "sha3-256",
    }
}

#[must_use]
pub const fn object_type_name(object_type: ObjectType) -> &'static str {
    match object_type {
        ObjectType::Blob => "blob",
        ObjectType::Tree => "tree",
        ObjectType::StateRoot => "state-root",
        ObjectType::Change => "change",
        ObjectType::ChangeRevision => "change-revision",
        ObjectType::World => "world",
        ObjectType::Fact => "fact",
        ObjectType::Operation => "operation",
        ObjectType::Bundle => "bundle",
    }
}

fn parse_digest_hex(input: &str, algorithm: HashAlgorithm) -> Result<[u8; ID_BYTES], SagnirError> {
    let expected_hex_len = digest_len(algorithm) * 2;
    if input.len() != expected_hex_len {
        return Err(SagnirError::InvalidValue);
    }

    let mut digest = [0_u8; ID_BYTES];
    for (index, pair) in input.as_bytes().chunks_exact(2).enumerate() {
        let high = decode_lower_hex(pair[0])?;
        let low = decode_lower_hex(pair[1])?;
        digest[index] = (high << 4) | low;
    }
    Ok(digest)
}

fn decode_lower_hex(byte: u8) -> Result<u8, SagnirError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(SagnirError::InvalidValue),
    }
}

pub(crate) const fn object_type_raw(object_type: ObjectType) -> u16 {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateRootRef {
    state_id: StateId,
    content_root: ObjectId,
    format_version: FormatVersion,
}

impl StateRootRef {
    #[must_use]
    pub const fn new(
        state_id: StateId,
        content_root: ObjectId,
        format_version: FormatVersion,
    ) -> Self {
        Self {
            state_id,
            content_root,
            format_version,
        }
    }

    #[must_use]
    pub const fn format_version(self) -> FormatVersion {
        self.format_version
    }
}

#[must_use]
pub const fn domain_tag(object_type: ObjectType) -> &'static [u8] {
    match object_type {
        ObjectType::Blob => b"sagnir.object.v1.blob",
        ObjectType::Tree => b"sagnir.object.v1.tree",
        ObjectType::StateRoot => b"sagnir.object.v1.state-root",
        ObjectType::Change => b"sagnir.object.v1.change",
        ObjectType::ChangeRevision => b"sagnir.object.v1.change-revision",
        ObjectType::World => b"sagnir.object.v1.world",
        ObjectType::Fact => b"sagnir.object.v1.fact",
        ObjectType::Operation => b"sagnir.object.v1.operation",
        ObjectType::Bundle => b"sagnir.object.v1.bundle",
    }
}

#[cfg(test)]
mod tests;
