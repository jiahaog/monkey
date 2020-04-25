use crate::compiler;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;
use error::Error;

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

        let program = parser.parse().unwrap();

        let compiled = compiler::compile(program).unwrap();

        self.vm.run(compiled)
    }
}
