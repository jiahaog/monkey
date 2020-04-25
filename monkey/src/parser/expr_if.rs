use crate::ast::{Expression, Statements};
use crate::parser::Parser;
use crate::parser::Precedence;
use crate::parser::{Error, ErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub fn parse_if_expression(&mut self) -> Result<Expression, Error> {
        self.parse_if_expression_conditional()
            .and_then(|conditional| {
                self.parse_if_expression_consequence()
                    .map(|consequence| (conditional, consequence))
            })
            .and_then(|(conditional, consequence)| {
                self.parse_if_expression_alternative()
                    .map(|alternative| (conditional, consequence, alternative))
            })
            .map(|(condition, consequence, alternative)| Expression::If {
                condition: Box::new(condition),
                consequence: consequence,
                alternative: alternative,
            })
    }

    fn parse_if_expression_conditional(&mut self) -> Result<Expression, Error> {
        self.lexer
            .next()
            .ok_or(Error {
                expected: ErrorExpected::ParenthesisForIfCondition,
                received: None,
            })
            .and_then(|token| match token {
                Token::LParen => self.next_expression(Precedence::Lowest),
                x => Err(Error {
                    expected: ErrorExpected::ParenthesisForIfCondition,
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

    fn parse_if_expression_consequence(&mut self) -> Result<Statements, Error> {
        self.lexer
            .next()
            .ok_or(Error {
                expected: ErrorExpected::ParenthesisForIfCondition,
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

    fn parse_if_expression_alternative(&mut self) -> Result<Statements, Error> {
        match self.lexer.peek() {
            Some(Token::Else) => {
                self.lexer.next(); // consume the else

                self.lexer
                    .next()
                    .ok_or(Error {
                        expected: ErrorExpected::ParenthesisForIfCondition,
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
            _ => Ok(Vec::new()),
        }
    }
}
