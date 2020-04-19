use crate::ast;
use crate::bytecode;
use crate::object::Object;
use std::{iter, vec};

#[cfg(test)]
mod tests;

pub enum CompileInstruction {
    Constant(Object),
    Add,
}

type CompileInstructions = Vec<CompileInstruction>;
type Result<T> = std::result::Result<T, Error>;

pub fn compile(program: ast::Program) -> Result<Output> {
    let nested_instructions = program
        .into_iter()
        .collect::<Result<Vec<CompileInstructions>>>()?;

    let bytecode = nested_instructions
        .into_iter()
        .flatten()
        .collect::<Output>();

    Ok(bytecode)
}

pub struct CompilerIntoIter(vec::IntoIter<ast::Statement>);

impl IntoIterator for ast::Program {
    type Item = Result<CompileInstructions>;
    type IntoIter = CompilerIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        CompilerIntoIter(self.statements.into_iter())
    }
}

impl Iterator for CompilerIntoIter {
    type Item = Result<CompileInstructions>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next()? {
            ast::Statement::Expression(expression) => Some(compile_expr(expression)),
            _ => unimplemented!(),
        }
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

impl iter::FromIterator<CompileInstruction> for Output {
    fn from_iter<I: IntoIterator<Item = CompileInstruction>>(iter: I) -> Self {
        iter.into_iter()
            .fold(Output::new(), |bytecode, ins| bytecode.add(ins))
    }
}

// TODO implement this
#[derive(Debug)]
pub struct Error {}
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
