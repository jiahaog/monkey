use self::Error::*;
use crate::ast::{Expression, Operator};
use crate::object::{Object, TypeMismatchError};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    TypeMismatch(TypeMismatchError),
    TypeError {
        message: String,
    },
    UnknownOperation {
        operator: Operator,
        right: Object,
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
                TypeMismatch(err) => format!("{}", err),
                TypeError { message } => format!("{}", message),
                UnknownOperation { operator, right } => format!(
                    "TypeError: unsupported operand type(s) for {}: '{}'",
                    operator,
                    right.type_str(),
                ),
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

impl From<TypeMismatchError> for Error {
    fn from(err: TypeMismatchError) -> Self {
        Error::TypeMismatch(err)
    }
}
