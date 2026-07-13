#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreMetadataError {
    BufferTooSmall,
    DuplicateField,
    InvalidMemoryBudget,
    InvalidProfile,
    InvalidRealmId,
    InvalidValue,
    InvalidVerificationMode,
    MalformedLine,
    MissingField,
    UnknownField,
    ValueOutOfRange,
}

impl core::fmt::Display for StoreMetadataError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(match self {
            Self::BufferTooSmall => "metadata output buffer is too small",
            Self::DuplicateField => "metadata contains a duplicate field",
            Self::InvalidMemoryBudget => "invalid verification memory budget",
            Self::InvalidProfile => "invalid Sagnir profile",
            Self::InvalidRealmId => "invalid Sagnir realm ID",
            Self::InvalidValue => "invalid metadata value",
            Self::InvalidVerificationMode => "invalid verification mode",
            Self::MalformedLine => "malformed metadata line",
            Self::MissingField => "required metadata field is missing",
            Self::UnknownField => "metadata contains an unknown field",
            Self::ValueOutOfRange => "metadata value is outside the admitted range",
        })
    }
}

pub(crate) fn assignment(line: &str) -> Result<(&str, &str), StoreMetadataError> {
    let Some((key, value)) = line.split_once('=') else {
        return Err(StoreMetadataError::MalformedLine);
    };
    let key = key.trim();
    let value = value.trim();
    if key.is_empty() || value.is_empty() || value.contains('=') {
        return Err(StoreMetadataError::MalformedLine);
    }
    Ok((key, value))
}

pub(crate) fn quoted(value: &str) -> Result<&str, StoreMetadataError> {
    let Some(value) = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    else {
        return Err(StoreMetadataError::InvalidValue);
    };
    if value.contains(['"', '\\', '\n', '\r']) {
        return Err(StoreMetadataError::InvalidValue);
    }
    Ok(value)
}

pub(crate) fn decimal(value: &str) -> Result<u64, StoreMetadataError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(StoreMetadataError::InvalidValue);
    }
    value
        .parse::<u64>()
        .map_err(|_| StoreMetadataError::ValueOutOfRange)
}

pub(crate) fn set_once<T>(slot: &mut Option<T>, value: T) -> Result<(), StoreMetadataError> {
    if slot.replace(value).is_some() {
        return Err(StoreMetadataError::DuplicateField);
    }
    Ok(())
}

pub(crate) struct MetadataWriter<'a> {
    output: &'a mut [u8],
    len: usize,
}

impl<'a> MetadataWriter<'a> {
    pub(crate) const fn new(output: &'a mut [u8]) -> Self {
        Self { output, len: 0 }
    }

    pub(crate) fn push(&mut self, value: &str) -> Result<(), StoreMetadataError> {
        let end = self
            .len
            .checked_add(value.len())
            .ok_or(StoreMetadataError::BufferTooSmall)?;
        let destination = self
            .output
            .get_mut(self.len..end)
            .ok_or(StoreMetadataError::BufferTooSmall)?;
        destination.copy_from_slice(value.as_bytes());
        self.len = end;
        Ok(())
    }

    pub(crate) fn push_u64(&mut self, mut value: u64) -> Result<(), StoreMetadataError> {
        let mut digits = [0_u8; 20];
        let mut index = digits.len();
        loop {
            index -= 1;
            let digit =
                u8::try_from(value % 10).map_err(|_| StoreMetadataError::ValueOutOfRange)?;
            digits[index] = b'0' + digit;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        let text =
            core::str::from_utf8(&digits[index..]).map_err(|_| StoreMetadataError::InvalidValue)?;
        self.push(text)
    }

    pub(crate) const fn len(&self) -> usize {
        self.len
    }
}
