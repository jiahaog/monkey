use crate::bytecode::Instruction;
use crate::compiler;
use crate::object::Object;
use error::Error;

use super::error;

const STACK_SIZE: usize = 2048;

pub struct Vm {}

impl Vm {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self, compiled: compiler::Output) -> Result<Object, Error> {
        let compiler::Output {
            constants,
            instructions,
        } = compiled;

        let result = instructions
            .into_iter()
            // TOOD make the stack a field.
            .fold(Ok(Vec::new()), |result, instruction| {
                // todo
                result.and_then(|mut stack| match instruction {
                    Instruction::OpConstant(i) => {
                        ith_object(&constants, i as usize).map(|constant| {
                            stack.push(constant);
                            stack
                        })
                    }
                    Instruction::OpAdd => {
                        let (left, right) = top_pair_object(&mut stack)?;

                        // TODO this should be shared with eval module
                        let evaluated = match (left, right) {
                            (Object::Integer(x), Object::Integer(y)) => Ok(Object::Integer(x + y)),
                            _ => unimplemented!(),
                        }?;

                        stack.push(evaluated);
                        Ok(stack)
                    }
                })
            });

        result.and_then(|stack| last_object(&stack))
    }
}

// TODO extract stack operations into a struct.

fn ith_object(stack: &Vec<Object>, i: usize) -> Result<Object, Error> {
    stack.get(i).cloned().ok_or(Error::StackOutOfRange)
}

fn last_object(stack: &Vec<Object>) -> Result<Object, Error> {
    let len = stack.len() - 1;
    ith_object(stack, len)
}

fn top_pair_object(stack: &mut Vec<Object>) -> Result<(Object, Object), Error> {
    top_pair_object_option(stack).ok_or(Error::StackOutOfRange)
}

fn top_pair_object_option(stack: &mut Vec<Object>) -> Option<(Object, Object)> {
    let left = stack.pop()?;
    let right = stack.pop()?;

    Some((left, right))
}
