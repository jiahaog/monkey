use super::Instruction;

pub type OpCode = u8;

pub const OP_CONSTANT: OpCode = 1;
pub const OP_ADD: OpCode = 2;

pub struct Definition {
    pub code: OpCode,
    pub name: &'static str,
    pub size: usize,
}

impl From<&Instruction> for Definition {
    fn from(ins: &Instruction) -> Self {
        use Instruction::*;

        match ins {
            OpConstant(_) => Self {
                name: "OpConstant",
                code: OP_CONSTANT,
                size: 1 + 2, // u16 for operand.
            },
            OpAdd => Self {
                name: "OpAdd",
                code: OP_ADD,
                size: 1,
            },
        }
    }
}
