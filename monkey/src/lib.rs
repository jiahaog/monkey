mod ast;
mod bytecode;
mod compiler;
mod eval;
mod lexer;
pub mod object;
mod parser;
mod token;
pub mod vm;

use crate::eval::Error as EvalError;
use crate::lexer::Lexer;
use crate::object::{Env, Object};
use crate::parser::{Errors, Parser};
use std::fmt::{Display, Formatter};

pub struct Interpreter {
    env: Env,
}

pub struct InterpreterResult {
    pub stdout: String,
    pub result: std::result::Result<Object, Error>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn evaluate(&mut self, s: String) -> InterpreterResult {
        // TODO cleanup output
        let result = match Parser::new(Lexer::new(&s)).parse() {
            Ok(program) => {
                let (env, eval_result) = program.evaluate(self.env.clone());

                // Update the existing env to preserve state.
                self.env = env;

                eval_result.map_err(|err| Error::Eval(err))
            }

            Err(e) => Err(Error::Parse(e)),
        };
        InterpreterResult {
            result,
            stdout: self.env.pop_stdout().join("\n"),
        }
    }
}

pub enum Error {
    Parse(Errors),
    Eval(EvalError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Parse(err) => write!(
                f,
                "{}",
                err.errors
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Error::Eval(error) => write!(f, "{}", error),
        }
    }
}
