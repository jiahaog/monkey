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
use crate::object::Object;
use crate::parser::{ParseError, Parser};
use std::fmt::{Display, Formatter};

pub struct Interpreter;

pub type InterpreterResult = Result<(Object, String), Error>;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, s: String) -> InterpreterResult {
        Parser::new(Lexer::new(&s))
            .parse()
            .map_err(|err| err.into())
            .and_then(|program| {
                program
                    .evaluate()
                    .map_err(|(err, stdout)| Error::Eval(err, stdout))
            })
    }
}

pub enum Error {
    Parse(Vec<ParseError>),
    Eval(EvalError, String),
}

impl From<Vec<ParseError>> for Error {
    fn from(err: Vec<ParseError>) -> Self {
        Error::Parse(err)
    }
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
            Error::Eval(error, stdout) => {
                let stdout = if stdout.len() > 0 {
                    format!("{}\n", stdout)
                } else {
                    "".into()
                };

                write!(f, "{}{}", stdout, error)
            }
        }
    }
}
