mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

use crate::eval::Error as EvalError;
use crate::eval::{Env, Object};
use crate::lexer::Lexer;
use crate::parser::{ParseError, Parser};
use std::fmt::{Display, Formatter};

pub struct Interpreter {
    env: Env,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    // TODO return objects and errors instead
    // Technically we don't need &mut for self because of env interior mutability
    pub fn evaluate(&mut self, s: String) -> std::result::Result<Object, Error> {
        // TODO cleanup output
        match Parser::new(Lexer::new(&s)).parse() {
            Ok(program) => match program.evaluate(self.env.clone()) {
                Ok(object) => Ok(object),
                Err(e) => Err(Error::Eval(e)),
            },
            Err(e) => Err(Error::Parse(e)),
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
