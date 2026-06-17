#![no_std]
#![forbid(unsafe_code)]
#![deny(unused_must_use)]

mod error;
mod id;
mod name;

pub use error::SagnirError;
pub use id::{
    BundleId, ChangeId, FORMAT_VERSION, FactId, FormatVersion, ID_BYTES, IdKind, ObjectIdRef,
    OperationId, RealmId, RevisionId, StateId, TypedId, WorldId, constant_time_bytes_choice,
    constant_time_bytes_eq,
};
pub use name::{
    BoundedName, NAME_MAX_BYTES, has_windows_path_alias, is_dotfile_segment, is_saga_segment,
    is_windows_reserved_name, valid_name_byte, valid_name_byte_no_slash,
};
