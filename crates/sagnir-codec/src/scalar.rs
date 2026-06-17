use sagnir_core::SagnirError;

pub const U16_BYTES: usize = 2;
pub const U32_BYTES: usize = 4;
pub const U64_BYTES: usize = 8;
pub const LIST_LEN_BYTES: usize = U64_BYTES;

#[must_use]
pub const fn encoded_u16(value: u16) -> [u8; U16_BYTES] {
    value.to_le_bytes()
}

#[must_use]
pub const fn encoded_u32(value: u32) -> [u8; U32_BYTES] {
    value.to_le_bytes()
}

#[must_use]
pub const fn encoded_u64(value: u64) -> [u8; U64_BYTES] {
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

pub fn write_u16(out: &mut [u8], value: u16) -> Result<&mut [u8], SagnirError> {
    write_bytes(out, &encoded_u16(value))
}

pub fn write_u32(out: &mut [u8], value: u32) -> Result<&mut [u8], SagnirError> {
    write_bytes(out, &encoded_u32(value))
}

pub fn write_u64(out: &mut [u8], value: u64) -> Result<&mut [u8], SagnirError> {
    write_bytes(out, &encoded_u64(value))
}

pub fn read_u16(input: &[u8]) -> Result<(u16, &[u8]), SagnirError> {
    let bytes = input.get(..U16_BYTES).ok_or(SagnirError::BufferTooSmall)?;
    let value = u16::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_error| SagnirError::InvalidValue)?,
    );
    Ok((value, &input[U16_BYTES..]))
}

pub fn read_u32(input: &[u8]) -> Result<(u32, &[u8]), SagnirError> {
    let bytes = input.get(..U32_BYTES).ok_or(SagnirError::BufferTooSmall)?;
    let value = u32::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_error| SagnirError::InvalidValue)?,
    );
    Ok((value, &input[U32_BYTES..]))
}

pub fn read_u64(input: &[u8]) -> Result<(u64, &[u8]), SagnirError> {
    let bytes = input.get(..U64_BYTES).ok_or(SagnirError::BufferTooSmall)?;
    let value = u64::from_le_bytes(
        bytes
            .try_into()
            .map_err(|_error| SagnirError::InvalidValue)?,
    );
    Ok((value, &input[U64_BYTES..]))
}

pub fn write_byte_string<'a>(out: &'a mut [u8], input: &[u8]) -> Result<&'a mut [u8], SagnirError> {
    let len = u64::try_from(input.len()).map_err(|_error| SagnirError::InvalidValue)?;
    let out = write_u64(out, len)?;
    write_bytes(out, input)
}

pub fn read_byte_string(input: &[u8], max_payload: usize) -> Result<(&[u8], &[u8]), SagnirError> {
    let (declared, rest) = read_u64(input)?;
    let declared = usize::try_from(declared).map_err(|_error| SagnirError::InvalidValue)?;
    if declared > max_payload {
        return Err(SagnirError::InvalidValue);
    }

    let payload = rest.get(..declared).ok_or(SagnirError::BufferTooSmall)?;
    Ok((payload, &rest[declared..]))
}

pub fn write_len_prefixed<'a>(
    out: &'a mut [u8],
    input: &[u8],
) -> Result<&'a mut [u8], SagnirError> {
    write_byte_string(out, input)
}

pub fn read_len_prefixed(input: &[u8], max_payload: usize) -> Result<(&[u8], &[u8]), SagnirError> {
    read_byte_string(input, max_payload)
}

pub fn write_list_len(out: &mut [u8], count: usize) -> Result<&mut [u8], SagnirError> {
    let count = u64::try_from(count).map_err(|_error| SagnirError::InvalidValue)?;
    write_u64(out, count)
}

pub fn read_list_len(input: &[u8], max_items: usize) -> Result<(usize, &[u8]), SagnirError> {
    let (count, rest) = read_u64(input)?;
    let count = usize::try_from(count).map_err(|_error| SagnirError::InvalidValue)?;
    if count > max_items {
        return Err(SagnirError::InvalidValue);
    }
    Ok((count, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_encoding_is_little_endian() {
        assert_eq!(encoded_u16(0x0102), [2, 1]);
        assert_eq!(encoded_u32(0x0102_0304), [4, 3, 2, 1]);
        assert_eq!(encoded_u64(0x0102_0304_0506_0708), [8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn integer_readers_return_value_and_tail() {
        assert_eq!(read_u16(&[2, 1, 9]), Ok((0x0102, &b"\t"[..])));
        assert_eq!(read_u32(&[4, 3, 2, 1, 9]), Ok((0x0102_0304, &b"\t"[..])));
        assert_eq!(
            read_u64(&[8, 7, 6, 5, 4, 3, 2, 1, 9]),
            Ok((0x0102_0304_0506_0708, &b"\t"[..]))
        );
    }

    #[test]
    fn integer_readers_reject_short_buffers() {
        assert_eq!(read_u16(&[0]), Err(SagnirError::BufferTooSmall));
        assert_eq!(read_u32(&[0; 3]), Err(SagnirError::BufferTooSmall));
        assert_eq!(read_u64(&[0; 7]), Err(SagnirError::BufferTooSmall));
    }

    #[test]
    fn integer_writers_fail_closed_on_small_buffers() {
        let mut one = [0_u8; 1];
        let mut three = [0_u8; 3];
        let mut seven = [0_u8; 7];

        assert_eq!(write_u16(&mut one, 1), Err(SagnirError::BufferTooSmall));
        assert_eq!(write_u32(&mut three, 1), Err(SagnirError::BufferTooSmall));
        assert_eq!(write_u64(&mut seven, 1), Err(SagnirError::BufferTooSmall));
    }

    #[test]
    fn byte_string_writer_rejects_small_buffer() {
        let mut out = [0_u8; 9];
        assert_eq!(
            write_byte_string(&mut out, b"ab").map(|tail| tail.len()),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn byte_string_writer_writes_length_and_bytes() {
        let mut out = [0_u8; 10];
        let tail_len = write_byte_string(&mut out, b"ab").map(|tail| tail.len());

        assert_eq!(tail_len, Ok(0));
        assert_eq!(out, [2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b']);
    }

    #[test]
    fn byte_string_reader_returns_payload_and_tail() {
        let input = [2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b', b'c'];

        assert_eq!(read_byte_string(&input, 2), Ok((&b"ab"[..], &b"c"[..])));
    }

    #[test]
    fn byte_string_reader_rejects_missing_length() {
        assert_eq!(
            read_byte_string(&[0; 7], 8),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn byte_string_reader_rejects_payload_over_caller_bound() {
        let input = [3, 0, 0, 0, 0, 0, 0, 0, b'a', b'b', b'c'];

        assert_eq!(read_byte_string(&input, 2), Err(SagnirError::InvalidValue));
    }

    #[test]
    fn byte_string_reader_rejects_truncated_payload() {
        let input = [3, 0, 0, 0, 0, 0, 0, 0, b'a', b'b'];

        assert_eq!(
            read_byte_string(&input, 3),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn len_prefixed_aliases_match_byte_string_encoding() {
        let mut out = [0_u8; 10];
        let input = [2, 0, 0, 0, 0, 0, 0, 0, b'a', b'b', b'c'];

        assert_eq!(
            write_len_prefixed(&mut out, b"ab").map(|tail| tail.len()),
            Ok(0)
        );
        assert_eq!(read_len_prefixed(&input, 2), Ok((&b"ab"[..], &b"c"[..])));
    }

    #[test]
    fn list_length_round_trips_with_tail() {
        let mut out = [0_u8; LIST_LEN_BYTES + 1];
        out[LIST_LEN_BYTES] = 9;

        let tail = write_list_len(&mut out, 3).map(|tail| tail.len());

        assert_eq!(tail, Ok(1));
        assert_eq!(read_list_len(&out, 3), Ok((3, &b"\t"[..])));
    }

    #[test]
    fn list_length_reader_rejects_missing_length() {
        assert_eq!(
            read_list_len(&[0; LIST_LEN_BYTES - 1], 1),
            Err(SagnirError::BufferTooSmall)
        );
    }

    #[test]
    fn list_length_reader_rejects_count_over_caller_bound() {
        let input = [2, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(read_list_len(&input, 1), Err(SagnirError::InvalidValue));
    }
}
