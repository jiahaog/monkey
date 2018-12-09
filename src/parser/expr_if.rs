use crate::ast::{Expression, Statement};
use crate::parser::Parser;
use crate::parser::Precedence;
use crate::parser::{ParseError, ParseErrorExpected};
use crate::token::Token;

impl<'a> Parser<'a> {
    pub(super) fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
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

    fn parse_if_expression_conditional(&mut self) -> Result<Expression, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::ParenthesisForIfCondition,
                received: None,
            })
            .and_then(|token| match token {
                Token::LParen => self.next_expression(Precedence::Lowest),
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParenthesisForIfCondition,
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

    fn parse_if_expression_consequence(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::ParenthesisForIfCondition,
                received: None,
            })
            .and_then(|token| match token {
                Token::LBrace => self.parse_block_statement(),
                x => Err(ParseError {
                    expected: ParseErrorExpected::ParenthesisForIfCondition,
                    received: Some(x),
                }),
            })
    }

    fn parse_if_expression_alternative(&mut self) -> Result<Vec<Statement>, ParseError> {
        match self.lexer.peek() {
            Some(Token::Else) => {
                self.lexer.next(); // consume the else

                self.lexer
                    .next()
                    .ok_or(ParseError {
                        expected: ParseErrorExpected::ParenthesisForIfCondition,
                        received: None,
                    })
                    .and_then(|token| match token {
                        Token::LBrace => self.parse_block_statement(),
                        x => Err(ParseError {
                            expected: ParseErrorExpected::ParenthesisForIfCondition,
                            received: Some(x),
                        }),
                    })
            }
            _ => Ok(Vec::new()),
        }
    }

    fn parse_block_statement(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();

        loop {
            if let None = self.lexer.peek() {
                return Ok(statements);
            }

            if let Some(Token::RBrace) = self.lexer.peek() {
                self.lexer.next();
                return Ok(statements);
            }

            match self.next_statement().unwrap() {
                Ok(statement) => statements.push(statement),
                Err(x) => return Err(x),
            }
        }
    }
}
