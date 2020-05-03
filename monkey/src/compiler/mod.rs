use crate::ast;
use crate::bytecode;
use crate::object::Object;
pub use error::Error;
use std::{iter, vec};

#[cfg(test)]
mod tests;

mod error;

#[derive(Debug)]
pub enum CompileInstruction {
    Constant(Object),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    True,
    False,
    GreaterThan,
    Equal,
    NotEqual,
    Neg,
    Not,
    // This is the number of bytes to jump forward relative to the current
    // position.
    JumpNotTruthy(u16),
}

type CompileInstructions = Vec<CompileInstruction>;
type Result<T> = std::result::Result<T, Error>;

pub fn compile(program: ast::Program) -> Result<Output> {
    program.statements.into_iter().collect()
}

impl iter::FromIterator<ast::Statement> for Result<Output> {
    fn from_iter<I: IntoIterator<Item = ast::Statement>>(statements: I) -> Self {
        let compiled_instructions = statements
            .into_iter()
            .collect::<Result<CompileInstructions>>()?;

        let output = compiled_instructions
            .into_iter()
            .fold(Output::new(), |bytecode, ins| bytecode.add_instruction(ins));

        Ok(output)
    }
}

impl iter::FromIterator<ast::Statement> for Result<CompileInstructions> {
    fn from_iter<I: IntoIterator<Item = ast::Statement>>(statements: I) -> Self {
        // TODO figure out a way to avoid two collects(). Maybe flat_map ?
        let nested_instructions = statements
            .into_iter()
            .map(compile_statement)
            .collect::<Result<Vec<CompileInstructions>>>()?;

        let ins = nested_instructions
            .into_iter()
            .flatten()
            .collect::<CompileInstructions>();

        Ok(ins)
    }
}

fn compile_statement(statement: ast::Statement) -> Result<CompileInstructions> {
    match statement {
        ast::Statement::Expression(expression) => compile_expr(expression).map(|mut ins| {
            ins.push(CompileInstruction::Pop);
            ins
        }),
        _ => unimplemented!(),
    }
}

fn compile_expr(expr: ast::Expression) -> Result<CompileInstructions> {
    match expr {
        ast::Expression::Prefix { operator, right } => {
            let right_result = compile_expr(*right)?;

            let mut result_operator = operator.into();

            // TODO: This is an inelegant hack because we want operator.into()
            // to return different results when it is a prefix expression.
            if let CompileInstruction::Sub = result_operator {
                result_operator = CompileInstruction::Neg;
            }

            let result = right_result
                .into_iter()
                .chain(vec![result_operator])
                .collect();

            Ok(result)
        }
        ast::Expression::Infix {
            operator,
            left,
            right,
        } => {
            let left_result = compile_expr(*left)?;
            let right_result = compile_expr(*right)?;

            // TODO maybe this is not so efficient.
            let result = left_result.into_iter().chain(right_result);

            // We don't have a OpLessThan, so do this instead.
            let result: Vec<CompileInstruction> = if let ast::Operator::LessThan = operator {
                result.rev().collect()
            } else {
                result.collect()
            };

            let result = result.into_iter().chain(vec![operator.into()]).collect();

            Ok(result)
        }
        ast::Expression::IntegerLiteral(value) => {
            let object = Object::Integer(value as isize);

            Ok(vec![CompileInstruction::Constant(object)])
        }
        ast::Expression::Boolean(value) => Ok(vec![if value {
            CompileInstruction::True
        } else {
            CompileInstruction::False
        }]),
        ast::Expression::If {
            condition,
            consequence,
            alternative: _,
        } => {
            let condition_result = compile_expr(*condition)?;
            let mut consequence_result = consequence
                .into_iter()
                .collect::<Result<CompileInstructions>>()?;

            // Keep the last statement on the stack because if blocks are assignable.
            // TODO What about when the if block is empty?
            match consequence_result.pop() {
                Some(CompileInstruction::Pop) => (),
                _ => panic!("Expected last compile instruction of a condition to be OpPop"),
            };

            let consequence_len = consequence_result
                .iter()
                .map(|ins| {
                    let ins: bytecode::Instruction = ins.into();
                    ins.size()
                })
                .sum();

            let result = condition_result
                .into_iter()
                .chain(vec![CompileInstruction::JumpNotTruthy(consequence_len)])
                .chain(consequence_result)
                .collect::<CompileInstructions>();

            Ok(result)
        }
        _ => unimplemented!(),
    }
}

impl From<ast::Operator> for CompileInstruction {
    fn from(operator: ast::Operator) -> Self {
        match operator {
            ast::Operator::Plus => CompileInstruction::Add,
            ast::Operator::Minus => CompileInstruction::Sub,
            ast::Operator::Multiply => CompileInstruction::Mul,
            ast::Operator::Divide => CompileInstruction::Div,
            ast::Operator::GreaterThan => CompileInstruction::GreaterThan,
            // Note that this is the same as GreaterThan. We reverse the
            // left and right expressions when we encounter this when
            // compiling.
            ast::Operator::LessThan => CompileInstruction::GreaterThan,
            ast::Operator::Equal => CompileInstruction::Equal,
            ast::Operator::NotEqual => CompileInstruction::NotEqual,
            ast::Operator::Not => CompileInstruction::Not,
        }
    }
}

// TODO Better way to get operand details instead of using a dummy operand.
const DUMMY_OPERAND: u16 = 999;

impl From<&CompileInstruction> for bytecode::Instruction {
    fn from(ins: &CompileInstruction) -> Self {
        match ins {
            CompileInstruction::Constant(_) => bytecode::Instruction::OpConstant(DUMMY_OPERAND),
            CompileInstruction::JumpNotTruthy(_) => {
                bytecode::Instruction::OpJumpNotTruthy(DUMMY_OPERAND)
            }
            CompileInstruction::Pop => bytecode::Instruction::OpPop,
            CompileInstruction::Add => bytecode::Instruction::OpAdd,
            CompileInstruction::Sub => bytecode::Instruction::OpSub,
            CompileInstruction::Mul => bytecode::Instruction::OpMul,
            CompileInstruction::Div => bytecode::Instruction::OpDiv,
            CompileInstruction::True => bytecode::Instruction::OpTrue,
            CompileInstruction::False => bytecode::Instruction::OpFalse,
            CompileInstruction::GreaterThan => bytecode::Instruction::OpGreaterThan,
            CompileInstruction::Equal => bytecode::Instruction::OpEqual,
            CompileInstruction::NotEqual => bytecode::Instruction::OpNotEqual,
            CompileInstruction::Neg => bytecode::Instruction::OpNeg,
            CompileInstruction::Not => bytecode::Instruction::OpNot,
        }
    }
}

#[derive(Debug)]
pub struct Output {
    pub instructions: Vec<bytecode::Instruction>,
    pub constants: Vec<Object>,
    // Index to place the next instruction, in number of bytes.
    index: u16,
}

impl Output {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            index: 0,
        }
    }

    fn add_instruction(mut self, ins: CompileInstruction) -> Self {
        // TODO: This is extremelty verbose, clean it up.
        let bytecode_ins = match ins {
            CompileInstruction::Constant(object) => {
                let i = self.constants.len();
                self.constants.push(object);

                bytecode::Instruction::OpConstant(i as u16)
            }
            CompileInstruction::JumpNotTruthy(location) => {
                let current_size = bytecode::Instruction::from(&ins).size();

                let jump_address = self.index + current_size + location;
                bytecode::Instruction::OpJumpNotTruthy(jump_address)
            }
            // Zero operand instructions.
            ins => (&ins).into(),
        };

        let ins_size = bytecode_ins.size();
        self.instructions.push(bytecode_ins);
        Self {
            instructions: self.instructions,
            constants: self.constants,
            index: self.index + ins_size,
        }
    }
}
