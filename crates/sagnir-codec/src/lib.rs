#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

mod scalar;

pub use scalar::{
    LIST_LEN_BYTES, U16_BYTES, U32_BYTES, U64_BYTES, encoded_u16, encoded_u32, encoded_u64,
    read_byte_string, read_len_prefixed, read_list_len, read_u16, read_u32, read_u64,
    write_byte_string, write_bytes, write_len_prefixed, write_list_len, write_u16, write_u32,
    write_u64,
};
