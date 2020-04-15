use crate::ast;
use crate::bytecode;
use crate::object::Object;
use std::ops;
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
        match acc {
          Ok(prev) => statement.compile(prev),
          Err(_) => acc,
        }
        // acc + statement.compile()?
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
  instructions: bytecode::Instructions,
  constants: Vec<Object>,
}

impl Bytecode {
  fn new() -> Self {
    Self {
      instructions: Vec::new(),
      constants: Vec::new(),
    }
  }

  fn op_constant(self, object: Object) -> Self {
    let mut constants = self.constants;
    constants.push(object);

    let mut instructions = self.instructions;
    let mut new_instruction =
      bytecode::make(bytecode::OP_CONSTANT, vec![(constants.len() - 1) as u16]);
    instructions.append(&mut new_instruction);

    Self {
      instructions,
      constants,
    }
  }
}

impl ops::Add<Bytecode> for Bytecode {
  type Output = Bytecode;
  fn add(self, mut other: Bytecode) -> Bytecode {
    let mut instructions = self.instructions;
    instructions.append(&mut other.instructions);

    let mut constants = self.constants;
    constants.append(&mut other.constants);

    Self {
      instructions,
      constants,
    }
  }
}
