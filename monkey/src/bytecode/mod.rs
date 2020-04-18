use std::fmt;

mod bytes;
mod opcode;

pub use bytes::Bytes;
pub use opcode::*;

#[cfg(test)]
mod tests;

use Instruction::*;
#[derive(PartialEq, Debug)]
pub enum Instruction {
    // pointer to the constant
    // TODO we want to change this to put the `object::Object` here instead so
    // we don't have to deal with a separate constants array. We should only
    // need to play with bytes and indexing when converting from this to bytes.
    OpConstant(u16),
    OpAdd,
}

impl Instruction {
    fn opcode(&self) -> OpCode {
        match self {
            OpConstant(_) => OP_CONSTANT,
            OpAdd => OP_ADD,
        }
    }

    fn opcode_name(&self) -> &str {
        match self {
            OpConstant(_) => "OpConstant",
            OpAdd => "OpAdd",
        }
    }

    fn size(&self) -> usize {
        let operand_size = match self {
            OpConstant(_) => 2,
            OpAdd => 0,
        };

        operand_size + 1
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpConstant(pointer) => write!(f, "{} {}", self.opcode_name(), pointer),
            OpAdd => write!(f, "{}", self.opcode_name()),
        }
    }
}

#[derive(Debug)]
pub struct Error {}
