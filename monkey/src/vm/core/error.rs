use crate::object::TypeMismatchError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    StackOutOfRange,
    TypeMismatch(TypeMismatchError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TypeMismatch(err) => write!(f, "VM error: {}", err),
            _ => write!(f, "VM error: {:?}", self),
        }
    }
}

impl std::error::Error for Error {}

impl From<TypeMismatchError> for Error {
    fn from(err: TypeMismatchError) -> Self {
        Error::TypeMismatch(err)
    }
}
