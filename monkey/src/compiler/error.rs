use std::{fmt, num};

#[derive(Debug)]
pub enum Error {
    // TODO: Add variants.
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compiler error: {:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<num::TryFromIntError> for Error {
    fn from(_: num::TryFromIntError) -> Self {
        Error::Overflow
    }
}
