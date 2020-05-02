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
            .fold(Output::new(), |bytecode, ins| bytecode.add_instruction(ins));

        Ok(output)
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
            x => unimplemented!("{} is not implemented", &x),
        }
    }
}

#[derive(Debug)]
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

    fn add_instruction(mut self, ins: CompileInstruction) -> Self {
        // TODO: This is extremelty verbose, clean it up.
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
            CompileInstruction::Pop => {
                self.instructions.push(bytecode::Instruction::OpPop);
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
            CompileInstruction::Sub => {
                self.instructions.push(bytecode::Instruction::OpSub);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::Mul => {
                self.instructions.push(bytecode::Instruction::OpMul);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::Div => {
                self.instructions.push(bytecode::Instruction::OpDiv);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::True => {
                self.instructions.push(bytecode::Instruction::OpTrue);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::False => {
                self.instructions.push(bytecode::Instruction::OpFalse);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::GreaterThan => {
                self.instructions.push(bytecode::Instruction::OpGreaterThan);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::Equal => {
                self.instructions.push(bytecode::Instruction::OpEqual);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
            CompileInstruction::NotEqual => {
                self.instructions.push(bytecode::Instruction::OpNotEqual);
                Self {
                    instructions: self.instructions,
                    constants: self.constants,
                }
            }
        }
    }
}
