use crate::ast;
use crate::bytecode;
use crate::object::Object;
pub use error::Error;
use std::{iter, vec};

#[cfg(test)]
mod tests;

mod error;

pub enum CompileInstruction {
    Constant(Object),
    Add,
}

type CompileInstructions = Vec<CompileInstruction>;
type Result<T> = std::result::Result<T, Error>;

pub fn compile(program: ast::Program) -> Result<Output> {
    program.statements.into_iter().collect()
}

impl iter::FromIterator<ast::Statement> for Result<Output> {
    fn from_iter<I: IntoIterator<Item = ast::Statement>>(statements: I) -> Self {
        let nested_instructions = statements
            .into_iter()
            .map(compile_statement)
            .collect::<Result<Vec<CompileInstructions>>>()?;

        let output = nested_instructions
            .into_iter()
            .flatten()
            .fold(Output::new(), |bytecode, ins| bytecode.add(ins));

        Ok(output)
    }
}

fn compile_statement(statement: ast::Statement) -> Result<CompileInstructions> {
    match statement {
        ast::Statement::Expression(expression) => compile_expr(expression),
        _ => unimplemented!(),
    }
}

fn compile_expr(expr: ast::Expression) -> Result<CompileInstructions> {
    match expr {
        ast::Expression::Infix {
            operator,
            left,
            right,
        } => {
            let left_result = compile_expr(*left)?;
            let right_result = compile_expr(*right)?;

            // TODO maybe this is not so efficient.
            let result = left_result
                .into_iter()
                .chain(right_result)
                .chain(vec![operator.into()])
                .collect();

            Ok(result)
        }
        ast::Expression::IntegerLiteral(value) => {
            let object = Object::Integer(value as isize);

            Ok(vec![CompileInstruction::Constant(object)])
        }
        _ => unimplemented!(),
    }
}

impl From<ast::Operator> for CompileInstruction {
    fn from(operator: ast::Operator) -> Self {
        match operator {
            ast::Operator::Plus => CompileInstruction::Add,
            _ => unimplemented!(),
        }
    }
}

pub struct Output {
    pub instructions: Vec<bytecode::Instruction>,
    pub constants: Vec<Object>,
}

impl Output {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn add(mut self, ins: CompileInstruction) -> Self {
        match ins {
            CompileInstruction::Constant(object) => {
                let i = self.constants.len();
                self.constants.push(object);

                let instruction = bytecode::Instruction::OpConstant(i as u16);

                self.instructions.push(instruction);

                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::Add => {
                self.instructions.push(bytecode::Instruction::OpAdd);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
        }
    }
}
