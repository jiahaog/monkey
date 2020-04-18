mod ast;
mod bytecode;
mod compiler;
mod eval;
mod lexer;
mod object;
mod parser;
mod token;
pub mod vm;

use crate::eval::Error as EvalError;
use crate::lexer::Lexer;
use crate::object::{Env, Object};
use crate::parser::{ParseError, Parser};
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

    pub fn evaluate(&self, s: String) -> InterpreterResult {
        // TODO cleanup output
        InterpreterResult {
            result: match Parser::new(Lexer::new(&s)).parse() {
                Ok(program) => match program.evaluate(self.env.clone()) {
                    Ok(object) => Ok(object),
                    Err(e) => Err(Error::Eval(e)),
                },
                Err(e) => Err(Error::Parse(e)),
            },
            stdout: self.env.pop_stdout().join("\n"),
        }
    }
}

pub enum Error {
    Parse(Vec<ParseError>),
    Eval(EvalError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::Parse(errors) => write!(
                f,
                "{}",
                errors
                    .into_iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Error::Eval(error) => write!(f, "{}", error),
        }
    }
}
