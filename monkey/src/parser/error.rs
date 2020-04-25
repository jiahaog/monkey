use self::ErrorExpected::*;
use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct Errors {
    pub errors: Vec<Error>,
}

impl std::error::Error for Errors {}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Vec<Error>> for Errors {
    fn from(errs: Vec<Error>) -> Self {
        Self { errors: errs }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorExpected {
    Identifier,
    Expression,
    Assignment,
    PrefixTokenOrExpression,
    ClosingParenthesis,
    ClosingBracket,
    ParenthesisForIfCondition,
    ParenthesisForFunctionParams,
    ParenthesisForFunctionBody,
    ParameterForFunction,
    SingleIndex,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub expected: ErrorExpected,
    pub received: Option<Token>,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expected = match self.expected {
            Identifier => "identifier",
            Expression => "expression",
            Assignment => "assignment",
            PrefixTokenOrExpression => "prefix token or expression",
            ClosingParenthesis => "closing parenthesis",
            ClosingBracket => "closing bracket",
            ParenthesisForIfCondition => "parenthesis for if condition",
            ParenthesisForFunctionParams => "parenthesis for function parameters",
            ParenthesisForFunctionBody => "parenthesis for function body",
            ParameterForFunction => "parameter for function",
            SingleIndex => "list index must be a single integer",
        };

        let received = match &self.received {
            Some(token) => format!("'{}'", token),
            None => String::from("nothing"),
        };
        write!(f, "Error: Expected {} but received {}", expected, received)
    }
}
