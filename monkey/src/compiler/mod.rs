use crate::ast;
use crate::bytecode;
use crate::object::Object;
use std::ops;

#[cfg(test)]
mod tests;

fn compile(program: ast::Program) -> Result<Bytecode, Error> {
  program.compile(Bytecode::new())
}

// TODO implement this
trait Node {
  fn compile(self, bytecode: Bytecode) -> Result<Bytecode, Error>;
}

impl Node for ast::Program {
  fn compile(self, bytecode: Bytecode) -> Result<Bytecode, Error> {
    self
      .statements
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
        println!("HELLO?");
        let object = Object::Integer(value as isize);

        Ok(bytecode.op_constant(object))
      }
      _ => unimplemented!(),
    }
  }
}

// TODO implement this
#[derive(Debug)]
struct Error {}

struct Bytecode {
  bytes: bytecode::Bytes,
  constants: Vec<Object>,
}

impl Bytecode {
  fn new() -> Self {
    Self {
      bytes: bytecode::Bytes::empty(),
      constants: Vec::new(),
    }
  }

  fn op_constant(self, object: Object) -> Self {
    let mut constants = self.constants;
    constants.push(object);

    let prev_bytes = self.bytes;
    let bytes =
      bytecode::Instruction::new(bytecode::OP_CONSTANT, vec![(constants.len() - 1) as u16]).make();

    Self {
      bytes: prev_bytes + bytes,
      constants,
    }
  }
}

impl ops::Add<Bytecode> for Bytecode {
  type Output = Bytecode;
  fn add(self, mut other: Bytecode) -> Bytecode {
    let bytes = self.bytes + other.bytes;

    let mut constants = self.constants;
    constants.append(&mut other.constants);

    Self { bytes, constants }
  }
}
