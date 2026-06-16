#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

mod error;
mod id;
mod name;

pub use error::SagnirError;
pub use id::{
    BundleId, ChangeId, FORMAT_VERSION, FactId, FormatVersion, ID_BYTES, IdKind, ObjectIdRef,
    OperationId, RealmId, RevisionId, StateId, TypedId, WorldId, constant_time_bytes_eq,
};
pub use name::{
    BoundedName, NAME_MAX_BYTES, is_saga_segment, valid_name_byte, valid_name_byte_no_slash,
};
