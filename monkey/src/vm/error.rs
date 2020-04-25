use crate::compiler;
use crate::parser::Errors;
use crate::vm::core;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Internal(core::Error),
    Compile(compiler::Error),
    Parse(Errors),
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
            Error::Parse(err) => Some(err),
            Error::Compile(err) => Some(err),
        }
    }
}

impl From<Errors> for Error {
    fn from(err: Errors) -> Self {
        Error::Parse(err)
    }
}
impl From<compiler::Error> for Error {
    fn from(err: compiler::Error) -> Self {
        Error::Compile(err)
    }
}

impl From<core::Error> for Error {
    fn from(err: core::Error) -> Self {
        Error::Internal(err)
    }
}
