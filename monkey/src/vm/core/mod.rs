use crate::bytecode::Instruction;
use crate::compiler;
use crate::object::Object;

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
                    Instruction::OpPop => {
                        let top = last_object(&stack)?;

                        self.last_popped = Some(top);
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
    let left = stack.pop()?;
    let right = stack.pop()?;

    Some((left, right))
}
