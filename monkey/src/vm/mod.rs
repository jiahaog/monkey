use crate::bytecode::Instruction;
use crate::object::Object;

#[cfg(test)]
mod tests;

const STACK_SIZE: usize = 2048;

pub struct Vm {
    constants: Vec<Object>,
    instructions: Vec<Instruction>,
    // stack: Vec<Object>,
    // sp: usize, // Points to next value on top of the stack.
}

// TODO
#[derive(Debug)]
pub struct Error {}

impl Vm {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Object>) -> Self {
        Self {
            constants,
            instructions,
            // stack: Vec::new(),
            // sp: 0, // TODO maybe we don't need this
        }
    }

    pub fn run(self) -> Result<Object, Error> {
        let Vm {
            constants,
            instructions,
        } = self;

        let result = instructions
            .into_iter()
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
    stack.get(i).cloned().ok_or(Error {})
}

fn last_object(stack: &Vec<Object>) -> Result<Object, Error> {
    let len = stack.len() - 1;
    ith_object(stack, len)
}

fn top_pair_object(stack: &mut Vec<Object>) -> Result<(Object, Object), Error> {
    top_pair_object_option(stack).ok_or(Error {})
}

fn top_pair_object_option(stack: &mut Vec<Object>) -> Option<(Object, Object)> {
    let left = stack.pop()?;
    let right = stack.pop()?;

    Some((left, right))
}
