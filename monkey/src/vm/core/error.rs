use std::fmt;

#[derive(Debug)]
pub enum Error {
    StackOutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VM error: {:?}", self)
    }
}

impl std::error::Error for Error {}
