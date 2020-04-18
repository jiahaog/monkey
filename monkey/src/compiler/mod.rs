use crate::ast;
use crate::bytecode;
use crate::object::Object;
use std::ops;

#[cfg(test)]
mod tests;

pub fn compile(program: ast::Program) -> Result<Bytecode, Error> {
    program.compile(Bytecode::new())
}

// TODO implement this
trait Node {
    fn compile(self, bytecode: Bytecode) -> Result<Bytecode, Error>;
}

impl Node for ast::Program {
    fn compile(self, bytecode: Bytecode) -> Result<Bytecode, Error> {
        self.statements
            .into_iter()
            .fold(Ok(bytecode), |acc, statement| {
                acc.and_then(|prev| statement.compile(prev))
            })
    }
}

impl Node for ast::Statement {
    fn compile(self, bytecode: Bytecode) -> Result<Bytecode, Error> {
        match self {
            ast::Statement::Expression(expression) => expression.compile(bytecode),
            _ => unimplemented!(),
        }
    }
}

impl Node for ast::Expression {
    fn compile(self, bytecode: Bytecode) -> Result<Bytecode, Error> {
        match self {
            ast::Expression::Infix {
                operator: _operator,
                left,
                right,
            } => {
                println!("left:{:?} right:{:?}", &left, &right);
                let left_result = left.compile(bytecode)?;
                let right_result = right.compile(left_result)?;

                // TODO operator

                Ok(right_result)
            }
            ast::Expression::IntegerLiteral(value) => {
                let object = Object::Integer(value as isize);

                Ok(bytecode.op_constant(object))
            }
            _ => unimplemented!(),
        }
    }
}

// TODO implement this
#[derive(Debug)]
pub struct Error {}

pub struct Bytecode {
    pub instructions: Vec<bytecode::Instruction>,
    pub constants: Vec<Object>,
}

impl Bytecode {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn op_constant(mut self, object: Object) -> Self {
        let i = self.constants.len();
        self.constants.push(object);

        let instruction = bytecode::Instruction::OpConstant(i as u16);

        self.instructions.push(instruction);

        Self {
            instructions: self.instructions,
            constants: self.constants,
        }
    }
}

impl ops::Add<Bytecode> for Bytecode {
    type Output = Bytecode;
    fn add(mut self, mut other: Bytecode) -> Bytecode {
        self.instructions.append(&mut other.instructions);
        self.constants.append(&mut other.constants);

        Self {
            instructions: self.instructions,
            constants: self.constants,
        }
    }
}
