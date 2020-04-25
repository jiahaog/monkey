use crate::vm::core;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Internal(core::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VM error: {:?}", self)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Internal(err) => Some(err),
        }
    }
}

impl From<core::Error> for Error {
    fn from(core_error: core::Error) -> Self {
        Error::Internal(core_error)
    }
}
