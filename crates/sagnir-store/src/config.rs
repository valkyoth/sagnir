use crate::metadata::{MetadataWriter, StoreMetadataError, assignment, decimal, quoted, set_once};

pub const CONFIG_FILE: &str = ".saga/config.toml";
pub const CONFIG_TEMP_FILE: &str = ".saga/config.toml.tmp";
pub const CONFIG_FILE_MAX: usize = 2_048;
pub const MEMORY_BUDGET_MIN: u64 = 16 * 1_024 * 1_024;
pub const MEMORY_BUDGET_MAX: u64 = 1_024 * 1_024 * 1_024 * 1_024;
pub const PARALLELISM_MAX: u16 = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Profile {
    Standard,
    Solo,
    Team,
    Regulated,
}

impl Profile {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Solo => "solo",
            Self::Team => "team",
            Self::Regulated => "regulated",
        }
    }
}

impl core::str::FromStr for Profile {
    type Err = StoreMetadataError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "standard" => Ok(Self::Standard),
            "solo" => Ok(Self::Solo),
            "team" => Ok(Self::Team),
            "regulated" => Ok(Self::Regulated),
            _ => Err(StoreMetadataError::InvalidProfile),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationMode {
    BoundedBatch,
    LazyCone,
    FullWorld,
}

impl VerificationMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundedBatch => "bounded-batch",
            Self::LazyCone => "lazy-cone",
            Self::FullWorld => "full-world",
        }
    }
}

impl core::str::FromStr for VerificationMode {
    type Err = StoreMetadataError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "bounded-batch" => Ok(Self::BoundedBatch),
            "lazy-cone" => Ok(Self::LazyCone),
            "full-world" => Ok(Self::FullWorld),
            _ => Err(StoreMetadataError::InvalidVerificationMode),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryBudget(u64);

impl MemoryBudget {
    pub const fn new(bytes: u64) -> Result<Self, StoreMetadataError> {
        if bytes < MEMORY_BUDGET_MIN || bytes > MEMORY_BUDGET_MAX {
            return Err(StoreMetadataError::ValueOutOfRange);
        }
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerificationConfig {
    pub mode: VerificationMode,
    pub memory_budget: Option<MemoryBudget>,
    pub parallelism: Option<u16>,
    pub max_entries: Option<u32>,
    pub max_refs: Option<u32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RealmConfig {
    pub profile: Profile,
    pub verification: VerificationConfig,
}

impl Default for RealmConfig {
    fn default() -> Self {
        Self {
            profile: Profile::Standard,
            verification: VerificationConfig {
                mode: VerificationMode::LazyCone,
                memory_budget: MemoryBudget::new(512 * 1_024 * 1_024).ok(),
                parallelism: None,
                max_entries: None,
                max_refs: None,
            },
        }
    }
}

pub fn parse_config_toml(input: &str) -> Result<RealmConfig, StoreMetadataError> {
    if input.len() > CONFIG_FILE_MAX {
        return Err(StoreMetadataError::ValueOutOfRange);
    }

    let mut fields = ConfigFields::default();
    let mut in_verification = false;
    let mut saw_verification = false;
    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if line == "[verification]" {
            if saw_verification {
                return Err(StoreMetadataError::DuplicateField);
            }
            in_verification = true;
            saw_verification = true;
            continue;
        }
        if line.starts_with('[') {
            return Err(StoreMetadataError::UnknownField);
        }
        let (key, value) = assignment(line)?;
        if in_verification {
            fields.set_verification(key, value)?;
        } else {
            fields.set_root(key, value)?;
        }
    }
    fields.finish(saw_verification)
}

pub fn write_config_toml(
    config: RealmConfig,
    output: &mut [u8],
) -> Result<usize, StoreMetadataError> {
    validate_config(config)?;
    let mut writer = MetadataWriter::new(output);
    writer.push("format = 1\nprofile = \"")?;
    writer.push(config.profile.as_str())?;
    writer.push("\"\n\n[verification]\nmode = \"")?;
    writer.push(config.verification.mode.as_str())?;
    writer.push("\"\n")?;
    if let Some(memory) = config.verification.memory_budget {
        writer.push("memory_budget = \"")?;
        write_memory_budget(memory, &mut writer)?;
        writer.push("\"\n")?;
    }
    write_optional_number(
        "parallelism",
        config.verification.parallelism.map(u64::from),
        &mut writer,
    )?;
    write_optional_number(
        "max_entries",
        config.verification.max_entries.map(u64::from),
        &mut writer,
    )?;
    write_optional_number(
        "max_refs",
        config.verification.max_refs.map(u64::from),
        &mut writer,
    )?;
    Ok(writer.len())
}

#[derive(Default)]
struct ConfigFields {
    format: Option<u64>,
    profile: Option<Profile>,
    mode: Option<VerificationMode>,
    memory_budget: Option<MemoryBudget>,
    memory_seen: bool,
    parallelism: Option<u16>,
    parallelism_seen: bool,
    max_entries: Option<u32>,
    max_entries_seen: bool,
    max_refs: Option<u32>,
    max_refs_seen: bool,
}

impl ConfigFields {
    fn set_root(&mut self, key: &str, value: &str) -> Result<(), StoreMetadataError> {
        match key {
            "format" => set_once(&mut self.format, decimal(value)?),
            "profile" => set_once(&mut self.profile, quoted(value)?.parse()?),
            _ => Err(StoreMetadataError::UnknownField),
        }
    }

    fn set_verification(&mut self, key: &str, value: &str) -> Result<(), StoreMetadataError> {
        match key {
            "mode" => set_once(&mut self.mode, quoted(value)?.parse()?),
            "memory_budget" => {
                mark_once(&mut self.memory_seen)?;
                self.memory_budget = Some(parse_memory_budget(quoted(value)?)?);
                Ok(())
            }
            "parallelism" => {
                mark_once(&mut self.parallelism_seen)?;
                self.parallelism = Some(parse_parallelism(value)?);
                Ok(())
            }
            "max_entries" => {
                mark_once(&mut self.max_entries_seen)?;
                self.max_entries = Some(parse_nonzero_u32(value)?);
                Ok(())
            }
            "max_refs" => {
                mark_once(&mut self.max_refs_seen)?;
                self.max_refs = Some(parse_nonzero_u32(value)?);
                Ok(())
            }
            _ => Err(StoreMetadataError::UnknownField),
        }
    }

    fn finish(self, saw_verification: bool) -> Result<RealmConfig, StoreMetadataError> {
        let format = self.format.ok_or(StoreMetadataError::MissingField)?;
        if format != 1 {
            return Err(StoreMetadataError::InvalidValue);
        }
        if !saw_verification {
            return Err(StoreMetadataError::MissingField);
        }
        let config = RealmConfig {
            profile: self.profile.ok_or(StoreMetadataError::MissingField)?,
            verification: VerificationConfig {
                mode: self.mode.ok_or(StoreMetadataError::MissingField)?,
                memory_budget: self.memory_budget,
                parallelism: self.parallelism,
                max_entries: self.max_entries,
                max_refs: self.max_refs,
            },
        };
        validate_config(config)?;
        Ok(config)
    }
}

fn parse_memory_budget(value: &str) -> Result<MemoryBudget, StoreMetadataError> {
    let (number, multiplier) = [
        ("TiB", 1_024_u64.pow(4)),
        ("GiB", 1_024_u64.pow(3)),
        ("MiB", 1_024_u64.pow(2)),
        ("KiB", 1_024_u64),
        ("B", 1_u64),
    ]
    .into_iter()
    .find_map(|(suffix, multiplier)| {
        value
            .strip_suffix(suffix)
            .map(|number| (number, multiplier))
    })
    .ok_or(StoreMetadataError::InvalidMemoryBudget)?;
    let bytes = decimal(number).and_then(|number| {
        number
            .checked_mul(multiplier)
            .ok_or(StoreMetadataError::ValueOutOfRange)
    })?;
    MemoryBudget::new(bytes).map_err(|_| StoreMetadataError::InvalidMemoryBudget)
}

fn parse_parallelism(value: &str) -> Result<u16, StoreMetadataError> {
    let value = u16::try_from(decimal(value)?).map_err(|_| StoreMetadataError::ValueOutOfRange)?;
    if value == 0 || value > PARALLELISM_MAX {
        return Err(StoreMetadataError::ValueOutOfRange);
    }
    Ok(value)
}

fn parse_nonzero_u32(value: &str) -> Result<u32, StoreMetadataError> {
    let value = u32::try_from(decimal(value)?).map_err(|_| StoreMetadataError::ValueOutOfRange)?;
    if value == 0 {
        return Err(StoreMetadataError::ValueOutOfRange);
    }
    Ok(value)
}

fn validate_config(config: RealmConfig) -> Result<(), StoreMetadataError> {
    if config.verification.parallelism == Some(0)
        || config
            .verification
            .parallelism
            .is_some_and(|value| value > PARALLELISM_MAX)
        || config.verification.max_entries == Some(0)
        || config.verification.max_refs == Some(0)
    {
        return Err(StoreMetadataError::ValueOutOfRange);
    }
    Ok(())
}

fn mark_once(seen: &mut bool) -> Result<(), StoreMetadataError> {
    if *seen {
        return Err(StoreMetadataError::DuplicateField);
    }
    *seen = true;
    Ok(())
}

fn write_memory_budget(
    memory: MemoryBudget,
    writer: &mut MetadataWriter<'_>,
) -> Result<(), StoreMetadataError> {
    let bytes = memory.bytes();
    for (suffix, divisor) in [
        ("TiB", 1_024_u64.pow(4)),
        ("GiB", 1_024_u64.pow(3)),
        ("MiB", 1_024_u64.pow(2)),
        ("KiB", 1_024_u64),
    ] {
        if bytes.is_multiple_of(divisor) {
            writer.push_u64(bytes / divisor)?;
            return writer.push(suffix);
        }
    }
    writer.push_u64(bytes)?;
    writer.push("B")
}

fn write_optional_number(
    key: &str,
    value: Option<u64>,
    writer: &mut MetadataWriter<'_>,
) -> Result<(), StoreMetadataError> {
    if let Some(value) = value {
        writer.push(key)?;
        writer.push(" = ")?;
        writer.push_u64(value)?;
        writer.push("\n")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
