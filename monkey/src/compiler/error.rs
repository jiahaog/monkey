use std::fmt;
// TODO
#[derive(Debug)]
pub enum Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compiler error: {:?}", self)
    }
}

impl std::error::Error for Error {}
