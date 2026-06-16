#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

use sagnir_core::SagnirError;

#[must_use]
pub const fn encoded_u16(value: u16) -> [u8; 2] {
    value.to_le_bytes()
}

#[must_use]
pub const fn encoded_u64(value: u64) -> [u8; 8] {
    value.to_le_bytes()
}

pub fn write_bytes<'a>(out: &'a mut [u8], input: &[u8]) -> Result<&'a mut [u8], SagnirError> {
    if out.len() < input.len() {
        return Err(SagnirError::BufferTooSmall);
    }
    let (head, tail) = out.split_at_mut(input.len());
    head.copy_from_slice(input);
    Ok(tail)
}

pub fn write_len_prefixed<'a>(
    out: &'a mut [u8],
    input: &[u8],
) -> Result<&'a mut [u8], SagnirError> {
    let len = u64::try_from(input.len()).map_err(|_error| SagnirError::InvalidValue)?;
    let out = write_bytes(out, &encoded_u64(len))?;
    write_bytes(out, input)
}

pub fn read_len_prefixed(input: &[u8], max_payload: usize) -> Result<(&[u8], &[u8]), SagnirError> {
    let len_bytes = input.get(..8).ok_or(SagnirError::BufferTooSmall)?;
    let declared = u64::from_le_bytes(
        len_bytes
            .try_into()
            .map_err(|_error| SagnirError::InvalidValue)?,
    );
    let declared = usize::try_from(declared).map_err(|_error| SagnirError::InvalidValue)?;
    if declared > max_payload {
        return Err(SagnirError::InvalidValue);
    }

    let payload_end = 8_usize
        .checked_add(declared)
        .ok_or(SagnirError::InvalidValue)?;
    let payload = input
        .get(8..payload_end)
        .ok_or(SagnirError::BufferTooSmall)?;
    let tail = input
        .get(payload_end..)
        .ok_or(SagnirError::BufferTooSmall)?;
    Ok((payload, tail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u64_encoding_is_little_endian() {
        assert_eq!(encoded_u64(0x0102_0304_0506_0708), [8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn len_prefixed_writer_rejects_small_buffer() {
        let mut out = [0_u8; 9];
        assert_eq!(
            write_len_prefixed(&mut out, b"ab").map(|tail| tail.len()),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn len_prefixed_writer_writes_length_and_bytes() {
        let mut out = [0_u8; 10];
        let tail_len = write_len_prefixed(&mut out, b"ab").map(|tail| tail.len());
        assert_eq!(tail_len, Ok(0));
        assert_eq!(out, [2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b']);
    }

    #[test]
    fn len_prefixed_reader_returns_payload_and_tail() {
        let input = [2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b', b'c'];

        assert_eq!(read_len_prefixed(&input, 2), Ok((&b"ab"[..], &b"c"[..])));
    }

    #[test]
    fn len_prefixed_reader_rejects_missing_length() {
        assert_eq!(
            read_len_prefixed(&[0; 7], 8),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn len_prefixed_reader_rejects_payload_over_caller_bound() {
        let input = [3, 0, 0, 0, 0, 0, 0, 0, b'a', b'b', b'c'];

        assert_eq!(read_len_prefixed(&input, 2), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn len_prefixed_reader_rejects_truncated_payload() {
        let input = [3, 0, 0, 0, 0, 0, 0, 0, b'a', b'b'];

        assert_eq!(
            read_len_prefixed(&input, 3),
            Err(SagnirError::BufferTooSmall)
        );
    }
}
