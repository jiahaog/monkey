use std::collections::HashMap;
use std::fmt;
use std::iter;
use std::ops;

#[cfg(test)]
mod tests;

pub struct Instruction {
  opcode: OpCode,
  // TODO change to signed integers
  operands: Vec<u16>,
}

impl Instruction {
  pub fn new(opcode: OpCode, operands: Vec<u16>) -> Self {
    Self { opcode, operands }
  }

  // TODO change to use From trait?
  pub fn make(self) -> Bytes {
    let definition = lookup(self.opcode);

    // TODO get rid of this?
    // let instruction_len: isize = definition.operand_widths.iter().sum();

    let mut bytes = Vec::new();
    bytes.push(self.opcode);

    for operand in self.operands.iter() {
      // We don't need to check operand_widths because of type safety.

      let byte_slice = operand.to_be_bytes();
      bytes.extend_from_slice(&byte_slice);
    }

    Bytes::new(bytes)
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

type OpCode = u8;

pub const OP_CONSTANT: OpCode = 1;

#[derive(Clone)]
struct Definition {
  name: &'static str,
  // TODO: Can we remove this?
  // length is the number of operands, value is the number of bytes for the operand.
  operand_widths: Vec<isize>,
}

fn lookup(opcode: OpCode) -> Definition {
  let mut definitions = HashMap::new();
  definitions.insert(
    OP_CONSTANT,
    Definition {
      name: "OpConstant",
      // 16 bits
      operand_widths: vec![2],
    },
  );

  definitions[&opcode].clone()
}

impl fmt::Display for Bytes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // todo!()
    write!(f, "hello")
  }
}
