use crate::ast::*;
use crate::bytecode::*;

// TODO this should be from object
struct Object {}

// TODO implement this
trait Node {}

// TODO implement this
struct Error {}

struct Compiler {
  // TODO these fields could be moved to compile as a parameter and result.
  instructions: Instructions,
  constants: Vec<Object>,
}

impl Compiler {
  fn new() -> Self {
    Self {
      instructions: Instructions::new(),
      constants: Vec::new(),
    }
  }

  fn compile(node: impl Node) -> Result<(), Error> {
    unimplemented!()
  }

  fn bytecode(self) -> Bytecode {
    Bytecode {
      instructions: self.instructions,
      constants: self.constants,
    }
  }
}

struct Bytecode {
  instructions: Instructions,
  constants: Vec<Object>,
}
