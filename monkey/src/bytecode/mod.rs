mod bytes;
mod opcode;
#[cfg(test)]
mod tests;

use opcode::*;
use std::fmt;
use Instruction::*;

pub use bytes::Bytes;
use opcode::Definition;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    // pointer to the constant
    // TODO we want to change this to put the `object::Object` here instead so
    // we don't have to deal with a separate constants array. We should only
    // need to play with bytes and indexing when converting from this to bytes.
    OpConstant(u16),
    OpPop,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
    OpGreaterThan,
    OpEqual,
    OpNotEqual,
    OpNeg,
    OpNot,
    OpJumpNotTruthy(u16),
}

impl Instruction {
    pub fn size(&self) -> u16 {
        let definition: Definition = self.into();
        definition.size
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let definition: Definition = self.into();

        match self {
            OpConstant(pointer) => write!(f, "{} {}", definition.name, pointer),
            _ => write!(f, "{}", definition.name),
        }
    }
}

#[derive(Debug)]
pub struct Error {}
