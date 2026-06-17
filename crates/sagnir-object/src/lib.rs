#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

mod header;
mod identity;

pub use header::{
    HEADER_LEN, MAGIC_LEN, OBJECT_BODY_BYTES_MAX, OBJECT_HEADER_MAGIC, ObjectHeader,
    ObjectHeaderField, ObjectHeaderFields, ObjectHeaderFlags, parse_object_header,
    write_object_header,
};
pub use identity::{
    HashAlgorithm, ObjectId, ObjectType, StateRootRef, domain_tag, parse_hash_algorithm,
    parse_object_type,
};
