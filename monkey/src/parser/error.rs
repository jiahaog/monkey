use self::ParseErrorExpected::*;
use crate::token::Token;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseErrorExpected {
    Identifier,
    Expression,
    Assignment,
    PrefixTokenOrExpression,
    ClosingParenthesis,
    ParenthesisForIfCondition,
    ParenthesisForFunctionParams,
    ParenthesisForFunctionBody,
    ParameterForFunction,
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub expected: ParseErrorExpected,
    pub received: Option<Token>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expected = match self.expected {
            Identifier => "identifier",
            Expression => "expression",
            Assignment => "assignment",
            PrefixTokenOrExpression => "prefix token or expression",
            ClosingParenthesis => "closing parenthesis",
            ParenthesisForIfCondition => "parenthesis for if condition",
            ParenthesisForFunctionParams => "parenthesis for function parameters",
            ParenthesisForFunctionBody => "parenthesis for function body",
            ParameterForFunction => "parameter for function",
        };

        let received = match &self.received {
            Some(token) => format!("'{}'", token),
            None => String::from("nothing"),
        };
        write!(
            f,
            "ParseError: Expected {} but received {}",
            expected, received
        )
    }
}
