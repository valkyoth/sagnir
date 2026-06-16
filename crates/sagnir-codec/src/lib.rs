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
}
