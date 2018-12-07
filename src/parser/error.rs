use token::Token;

#[derive(Debug, PartialEq)]
pub enum ParseErrorExpected {
    Identifier,
    Expression,
    Assignment,
    PrefixTokenOrExpression,
    ClosingParenthesis,
    ParenthesisForIfCondition,
}

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub expected: ParseErrorExpected,
    pub received: Option<Token>,
}
