use std::fmt;
#[derive(Debug)]
pub enum Error {
    // TODO: Add variants.
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compiler error: {:?}", self)
    }
}

impl std::error::Error for Error {}
