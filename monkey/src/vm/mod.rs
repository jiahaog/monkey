use crate::compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

pub use self::core::Stack;
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

    pub fn run(&mut self, stack: Stack, inp: &str) -> Result<Stack, Error> {
        let lexer = Lexer::new(inp);
        let parser = Parser::new(lexer);

        let program = parser.parse()?;

        let compiled = compiler::compile(program)?;

        self.vm.run(stack, compiled).map_err(|e| e.into())
    }

    pub fn last_popped(&self) -> Option<&Object> {
        self.vm.last_popped.as_ref()
    }
}
