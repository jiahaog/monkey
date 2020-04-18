use std::fmt;

mod bytes;
mod opcode;

pub use bytes::Bytes;
pub use opcode::*;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    // pointer to the constant
    OpConstant(u16),
}
use Instruction::*;

impl Instruction {
    fn opcode(&self) -> OpCode {
        match self {
            OpConstant(_) => OP_CONSTANT,
        }
    }

    fn opcode_name(&self) -> &str {
        match self {
            OpConstant(_) => "OpConstant",
        }
    }

    fn size(&self) -> usize {
        let operand_size = match self {
            OpConstant(_) => 2,
        };

        operand_size + 1
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operand = match self {
            OpConstant(pointer) => pointer,
        };

        write!(f, "{} {}", self.opcode_name(), operand)
    }
}

#[derive(Debug)]
pub struct Error {}
