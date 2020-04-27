use super::Instruction;

pub type OpCode = u8;

pub const OP_CONSTANT: OpCode = 1;
pub const OP_POP: OpCode = 3;
// TODO fix the ordering.
pub const OP_ADD: OpCode = 2;
pub const OP_SUB: OpCode = 4;
pub const OP_MUL: OpCode = 5;
pub const OP_DIV: OpCode = 6;
pub const OP_TRUE: OpCode = 7;
pub const OP_FALSE: OpCode = 8;

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
            OpPop => Self {
                name: "OpPop",
                code: OP_POP,
                size: 1,
            },
            OpTrue => Self {
                name: "OpTrue",
                code: OP_TRUE,
                size: 1,
            },
            OpFalse => Self {
                name: "OpFalse",
                code: OP_FALSE,
                size: 1,
            },
            OpAdd => Self {
                name: "OpAdd",
                code: OP_ADD,
                size: 1,
            },
            OpSub => Self {
                name: "OpSub",
                code: OP_SUB,
                size: 1,
            },
            OpMul => Self {
                name: "OpMul",
                code: OP_MUL,
                size: 1,
            },
            OpDiv => Self {
                name: "OpDiv",
                code: OP_DIV,
                size: 1,
            },
        }
    }
}
