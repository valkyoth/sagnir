#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SagnirError {
    EmptyName,
    NameTooLong,
    InvalidNameByte,
    BufferTooSmall,
    InvalidValue,
}
