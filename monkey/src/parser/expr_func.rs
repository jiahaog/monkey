use crate::ast::{Expression, Function, Statements};
use crate::parser::Parser;
use crate::parser::{ParseError, ParseErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub(super) fn parse_function_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_function_params()
            .and_then(|params| self.parse_function_body().map(|body| (params, body)))
            .map(|(params, body)| {
                Expression::FunctionLiteral(Function {
                    params: params,
                    body: body,
                })
            })
    }

    fn parse_function_params(&mut self) -> Result<Vec<String>, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::ParenthesisForFunctionParams,
                received: None,
            })
            .and_then(|token| match token {
                Token::LParen => self.chomp_function_params(Vec::new()),
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParenthesisForFunctionParams,
                    received: Some(x),
                }),
            })
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RParen) => {
                    self.lexer.next();
                    Ok(expr)
                }
                _ => Err(ParseError {
                    expected: ParseErrorExpected::ClosingParenthesis,
                    received: self.lexer.next(),
                }),
            })
    }

    fn chomp_function_params(&mut self, mut prev: Vec<String>) -> Result<Vec<String>, ParseError> {
        match self.lexer.peek() {
            Some(Token::RParen) => Ok(prev),
            Some(_) => match self.lexer.next() {
                Some(Token::Comma) => self.chomp_function_params(prev),
                Some(Token::Identifier(name)) => {
                    prev.push(name);
                    self.chomp_function_params(prev)
                }
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParameterForFunction,
                    received: x,
                }),
            },
            None => Err(ParseError {
                expected: ParseErrorExpected::ClosingParenthesis,
                received: None,
            }),
        }
    }

    fn parse_function_body(&mut self) -> Result<Statements, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::ParenthesisForFunctionBody,
                received: None,
            })
            .and_then(|token| match token {
                Token::LBrace => self.parse_block_statements(Vec::new()),
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParenthesisForIfCondition,
                    received: Some(x),
                }),
            })
    }
}
