use crate::token::Token;

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
