use crate::compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

pub use error::Error;

#[cfg(test)]
mod tests;

mod core;
mod error;

pub struct Vm {
    vm: core::Vm,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            vm: core::Vm::new(),
        }
    }
    pub fn run(&self, inp: &str) -> Result<Object, Error> {
        let lexer = Lexer::new(inp);
        let parser = Parser::new(lexer);

        let program = parser.parse()?;

        let compiled = compiler::compile(program)?;

        let object = self.vm.run(compiled)?;
        Ok(object)
    }
}
