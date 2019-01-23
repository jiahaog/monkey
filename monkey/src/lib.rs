mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::Env;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    // TODO return objects and errors instead
    // Technically we don't need &mut for self because of env interior mutability
    pub fn evaluate(&mut self, s: String) -> std::result::Result<String, String> {
        // TODO cleanup output
        match Parser::new(Lexer::new(&s)).parse() {
            Ok(program) => match program.evaluate(self.env.clone()) {
                Ok(result) => Ok(format!("{:?}\n", result)),
                Err(result) => Err(format!("{:?}\n", result)),
            },
            Err(e) => Err(format!("{:?}\n", e)),
        }
    }
}
