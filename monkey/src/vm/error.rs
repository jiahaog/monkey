use crate::compiler;
use crate::parser::ParseErrors;
use crate::vm::core;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Internal(core::Error),
    Compile(compiler::Error),
    Parse(ParseErrors),
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

impl From<ParseErrors> for Error {
    fn from(err: ParseErrors) -> Self {
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
