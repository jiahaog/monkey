mod collections;
mod error;
mod expr_call;
mod expr_func;
mod expr_if;
mod expr_prefix_infix;
mod precedence;
#[cfg(test)]
mod tests;

use self::error::ParseErrorExpected;
pub use self::error::{ParseError, ParseErrors};
use self::precedence::Precedence;
use crate::ast::{Program, Statement, Statements};
use crate::lexer::Lexer;
use crate::token::Token;
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(self) -> Result<Program, ParseErrors> {
        let (oks, fails): (Vec<_>, Vec<_>) = self.partition(Result::is_ok);
        let values = oks.into_iter().map(Result::unwrap).collect();
        let errors: Vec<ParseError> = fails.into_iter().map(Result::unwrap_err).collect();

        if errors.len() > 0 {
            Err(errors.into())
        } else {
            Ok(Program { statements: values })
        }
    }

    fn next_statement(&mut self) -> Option<Result<Statement, ParseError>> {
        (match self.lexer.peek() {
            None => None,
            Some(Token::Let) => {
                self.lexer.next();
                Some(self.next_let_statement())
            }
            Some(Token::Return) => {
                self.lexer.next();
                Some(self.next_return_statement())
            }
            _ => Some(self.next_expression_statement()),
        })
        .map(|result| {
            result.map_err(|err| {
                // Increment the iterator until the semicolon, so that the next call to next_statement will continue with the next line
                self.skip_tokens();
                err
            })
        })
    }

    fn next_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_let_statement_identifier()
            .and_then(|name| self.next_let_statement_assign().map(|_| name))
            .and_then(|name| {
                let result = self
                    .next_expression(Precedence::Lowest)
                    .map(|expression| Statement::Let(name, expression));

                if let Some(Token::Semicolon) = self.lexer.peek() {
                    self.lexer.next();
                }
                result
            })
    }

    fn next_let_statement_identifier(&mut self) -> Result<String, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Identifier,
                received: None,
            })
            .and_then(|token| match token {
                Token::Identifier(name) => Ok(name),
                unexpected => Err(ParseError {
                    expected: ParseErrorExpected::Identifier,
                    received: Some(unexpected),
                }),
            })
    }

    fn next_let_statement_assign(&mut self) -> Result<Token, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Assignment,
                received: None,
            })
            .and_then(|token| match token {
                Token::Assign => Ok(token),
                unexpected => Err(ParseError {
                    expected: ParseErrorExpected::Assignment,
                    received: Some(unexpected),
                }),
            })
    }

    fn next_return_statement(&mut self) -> Result<Statement, ParseError> {
        let result = self
            .next_expression(Precedence::Lowest)
            .map(|x| Statement::Return(x));

        if let Some(Token::Semicolon) = self.lexer.peek() {
            self.lexer.next();
        }

        result
    }

    fn next_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let result = self
            .next_expression(Precedence::Lowest)
            .map(|x| Statement::Expression(x));

        if let Some(Token::Semicolon) = self.lexer.peek() {
            self.lexer.next();
        }

        result
    }

    fn parse_block_statements(&mut self, mut prev: Statements) -> Result<Statements, ParseError> {
        match self.lexer.peek() {
            None => Ok(prev),
            Some(Token::RBrace) => {
                self.lexer.next();
                Ok(prev)
            }
            _ => match self.next_statement().unwrap() {
                // can unwrap because next token is guaranteed to not be none
                Ok(statement) => {
                    prev.push(statement);
                    self.parse_block_statements(prev)
                }
                Err(x) => Err(x),
            },
        }
    }

    // Attempts to skip tokens until a semicolon, which is useful in case we want to proceed to
    // parse the next statement even when there are errors
    fn skip_tokens(&mut self) {
        match self.lexer.next() {
            Some(Token::Semicolon) | None => (),
            _ => {
                self.skip_tokens();
            }
        };
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
