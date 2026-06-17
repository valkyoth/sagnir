#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

mod graph;
mod header;
mod identity;

pub use graph::{
    OBJECT_GRAPH_ENTRIES_MAX, OBJECT_GRAPH_REFS_MAX, ObjectGraph, ObjectGraphEntry,
    ObjectGraphReport, ObjectReference,
};
pub use header::{
    HEADER_LEN, MAGIC_LEN, OBJECT_BODY_BYTES_MAX, OBJECT_HEADER_MAGIC, ObjectHeader,
    ObjectHeaderField, ObjectHeaderFields, ObjectHeaderFlags, ParsedObjectHeader,
    parse_object_header, write_object_header,
};
pub use identity::{
    HASH_ALGORITHM_NAME_MAX_LEN, HashAlgorithm, OBJECT_ID_DIGEST_HEX_LEN, OBJECT_ID_MAX_LEN,
    OBJECT_ID_PREFIX, OBJECT_TYPE_NAME_MAX_LEN, ObjectId, ObjectType, RedactedObjectId,
    StateRootRef, digest_len, domain_tag, hash_algorithm_name, object_type_name,
    parse_hash_algorithm, parse_hash_algorithm_name, parse_object_id, parse_object_type,
    parse_object_type_name,
};
