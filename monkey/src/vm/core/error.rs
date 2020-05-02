use crate::object;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    StackOutOfRange,
    ObjectError(object::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ObjectError(err) => write!(f, "VM error: {}", err),
            _ => write!(f, "VM error: {:?}", self),
        }
    }
}

impl std::error::Error for Error {}

impl From<object::Error> for Error {
    fn from(err: object::Error) -> Self {
        Error::ObjectError(err)
    }
}
