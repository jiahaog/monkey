use std::convert;
use std::fmt;
use std::iter;
use std::ops;

mod opcode;

pub use opcode::*;

#[cfg(test)]
mod tests;

pub enum Instruction {
  // pointer to the constant
  OpConstant(u16),
}

impl Instruction {
  fn opcode(&self) -> OpCode {
    use Instruction::*;

    match self {
      OpConstant(_) => OP_CONSTANT,
    }
  }
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // todo!()
    write!(f, "hello")
  }
}


impl convert::From<Instruction> for Bytes {
  fn from(instruction: Instruction) -> Self {
    use Instruction::*;

    let mut bytes = vec![instruction.opcode()];

    match instruction {
      OpConstant(pointer) => {
        let byte_slice = pointer.to_be_bytes();
        bytes.extend_from_slice(&byte_slice);

        Bytes::new(bytes)
      }
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Bytes(Vec<u8>);

impl Bytes {
  fn new(bytes: Vec<u8>) -> Self {
    Self(bytes)
  }

  pub fn empty() -> Self {
    Self(Vec::new())
  }
}

impl ops::Add<Bytes> for Bytes {
  type Output = Bytes;
  fn add(self, mut other: Bytes) -> Bytes {
    let mut bytes = self.0;

    bytes.append(&mut other.0);

    Self(bytes)
  }
}

impl iter::Sum for Bytes {
  fn sum<I: Iterator<Item = Bytes>>(iter: I) -> Self {
    iter.fold(Bytes::empty(), |acc, x| acc + x)
  }
}

impl fmt::Display for Bytes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // todo!()
    write!(f, "hello")
  }
}
