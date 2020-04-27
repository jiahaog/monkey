use opcode::Definition;
use std::fmt;

mod bytes;
mod opcode;

pub use bytes::Bytes;
use opcode::*;

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
    OpPop,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let definition: Definition = self.into();

        match self {
            OpConstant(pointer) => write!(f, "{} {}", definition.name, pointer),
            OpPop => write!(f, "{}", definition.name),
            OpAdd => write!(f, "{}", definition.name),
            OpSub => write!(f, "{}", definition.name),
            OpMul => write!(f, "{}", definition.name),
            OpDiv => write!(f, "{}", definition.name),
            OpTrue => write!(f, "{}", definition.name),
            OpFalse => write!(f, "{}", definition.name),
        }
    }
}

#[derive(Debug)]
pub struct Error {}
