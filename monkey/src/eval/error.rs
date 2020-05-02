use self::Error::*;
use crate::ast::Expression;
use crate::object;
use crate::object::Object;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    ObjectError(object::Error),
    TypeError {
        message: String,
    },
    IdentifierNotFound {
        name: String,
    },
    CallExpressionExpectedFunction {
        received: Object,
    },
    CallExpressionWrongNumArgs {
        params: Vec<String>,
        arguments: Vec<Expression>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectError(err) => format!("{}", err),
                TypeError { message } => format!("{}", message),
                IdentifierNotFound { name } => format!("NameError: name '{}' is not defined", name),
                CallExpressionExpectedFunction { received } => format!(
                    "TypeError: '{}' object is not callable",
                    received.type_str(),
                ),
                CallExpressionWrongNumArgs { params, arguments } => format!(
                    "TypeError: function takes {} positional {} but {} {} given",
                    params.len(),
                    if params.len() == 1 {
                        "argument"
                    } else {
                        "arguments"
                    },
                    arguments.len(),
                    if arguments.len() == 1 { "was" } else { "were" },
                ),
            }
        )
    }
}

impl From<object::Error> for Error {
    fn from(err: object::Error) -> Self {
        Error::ObjectError(err)
    }
}
