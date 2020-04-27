use crate::ast::Operator;
use crate::bytecode::Instruction;
use crate::compiler;
use crate::object::{Object, FALSE, TRUE};

pub use error::Error;
mod error;

const STACK_SIZE: usize = 2048;

pub struct Vm {
    // For testing.
    // TODO: Put this into the Stack object.
    pub last_popped: Option<Object>,
}

// TODO: Make this a proper struct.
pub type Stack = Vec<Object>;

impl Vm {
    pub fn new() -> Self {
        Self { last_popped: None }
    }

    pub fn run(&mut self, stack: Stack, compiled: compiler::Output) -> Result<Stack, Error> {
        let compiler::Output {
            constants,
            instructions,
        } = compiled;

        instructions
            .into_iter()
            // TOOD make the stack a field.
            .fold(Ok(stack), |result, instruction| {
                // todo
                result.and_then(|mut stack| match instruction {
                    Instruction::OpConstant(i) => {
                        ith_object(&constants, i as usize).map(|constant| {
                            stack.push(constant);
                            stack
                        })
                    }
                    Instruction::OpPop => {
                        let top = last_object(&stack)?;

                        self.last_popped = Some(top);
                        Ok(stack)
                    }
                    Instruction::OpAdd => {
                        let (left, right) = top_pair_object(&mut stack)?;

                        let evaluated = left.apply_operator(Operator::Plus, right)?;

                        stack.push(evaluated);
                        Ok(stack)
                    }
                    Instruction::OpSub => {
                        let (left, right) = top_pair_object(&mut stack)?;

                        let evaluated = left.apply_operator(Operator::Minus, right)?;

                        stack.push(evaluated);
                        Ok(stack)
                    }
                    Instruction::OpMul => {
                        let (left, right) = top_pair_object(&mut stack)?;

                        let evaluated = left.apply_operator(Operator::Multiply, right)?;

                        stack.push(evaluated);
                        Ok(stack)
                    }
                    Instruction::OpDiv => {
                        let (left, right) = top_pair_object(&mut stack)?;

                        let evaluated = left.apply_operator(Operator::Divide, right)?;

                        stack.push(evaluated);
                        Ok(stack)
                    }
                    Instruction::OpTrue => {
                        stack.push(TRUE);
                        Ok(stack)
                    }
                    Instruction::OpFalse => {
                        stack.push(FALSE);
                        Ok(stack)
                    }
                })
            })
    }
}

// TODO extract stack operations into a struct.

fn ith_object(stack: &Vec<Object>, i: usize) -> Result<Object, Error> {
    stack.get(i).cloned().ok_or(Error::StackOutOfRange)
}

fn last_object(stack: &Vec<Object>) -> Result<Object, Error> {
    if stack.len() == 0 {
        Err(Error::StackOutOfRange)
    } else {
        let len = stack.len() - 1;
        ith_object(stack, len)
    }
}

fn top_pair_object(stack: &mut Vec<Object>) -> Result<(Object, Object), Error> {
    top_pair_object_option(stack).ok_or(Error::StackOutOfRange)
}

fn top_pair_object_option(stack: &mut Vec<Object>) -> Option<(Object, Object)> {
    let right = stack.pop()?;
    let left = stack.pop()?;

    Some((left, right))
}
