use crate::ast::{Expression, Function, Statements};
use crate::parser::Parser;
use crate::parser::{Error, ErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub fn parse_function_expression(&mut self) -> Result<Expression, Error> {
        self.parse_function_params()
            .and_then(|params| self.parse_function_body().map(|body| (params, body)))
            .map(|(params, body)| {
                Expression::FunctionLiteral(Function {
                    params: params,
                    body: body,
                })
            })
    }

    fn parse_function_params(&mut self) -> Result<Vec<String>, Error> {
        self.lexer
            .next()
            .ok_or(Error {
                expected: ErrorExpected::ParenthesisForFunctionParams,
                received: None,
            })
            .and_then(|token| match token {
                Token::LParen => self.chomp_function_params(Vec::new()),
                x => Err(Error {
                    expected: ErrorExpected::ParenthesisForFunctionParams,
                    received: Some(x),
                }),
            })
            .and_then(|expr| match self.lexer.peek() {
                Some(Token::RParen) => {
                    self.lexer.next();
                    Ok(expr)
                }
                _ => Err(Error {
                    expected: ErrorExpected::ClosingParenthesis,
                    received: self.lexer.next(),
                }),
            })
    }

    fn chomp_function_params(&mut self, mut prev: Vec<String>) -> Result<Vec<String>, Error> {
        match self.lexer.peek() {
            Some(Token::RParen) => Ok(prev),
            Some(_) => match self.lexer.next() {
                Some(Token::Comma) => self.chomp_function_params(prev),
                Some(Token::Identifier(name)) => {
                    prev.push(name);
                    self.chomp_function_params(prev)
                }
                x => Err(Error {
                    expected: ErrorExpected::ParameterForFunction,
                    received: x,
                }),
            },
            None => Err(Error {
                expected: ErrorExpected::ClosingParenthesis,
                received: None,
            }),
        }
    }

    fn parse_function_body(&mut self) -> Result<Statements, Error> {
        self.lexer
            .next()
            .ok_or(Error {
                expected: ErrorExpected::ParenthesisForFunctionBody,
                received: None,
            })
            .and_then(|token| match token {
                Token::LBrace => self.parse_block_statements(Vec::new()),
                x => Err(Error {
                    expected: ErrorExpected::ParenthesisForIfCondition,
                    received: Some(x),
                }),
            })
    }
}
