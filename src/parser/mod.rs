#![allow(dead_code)]

mod error;
mod expr_if;
mod expr_prefix_infix;
mod precedence;
#[cfg(test)]
mod tests;

use self::error::{ParseError, ParseErrorExpected};
use self::precedence::Precedence;
use ast::{Expression, Program, Statement};
use lexer::Lexer;
use std::iter::Peekable;
use token::Token;

pub struct Parser<'a> {
    // TODO remove the peekable type if it's really unnecessary
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    fn parse(self) -> Result<Program, Vec<ParseError>> {
        let (oks, fails): (Vec<_>, Vec<_>) = self.partition(Result::is_ok);
        let values = oks.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = fails.into_iter().map(Result::unwrap_err).collect();

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(Program::new(values))
        }
    }

    fn next_statement(&mut self) -> Option<Result<Statement, ParseError>> {
        match self.lexer.peek() {
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
        }
    }

    fn next_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_let_statement_identifier()
            .and_then(|name| self.next_let_statement_assign().map(|_| name))
            .map_err(|err| {
                // Increment the iterator until the semicolon, so that the next call to
                // next_let_statement will continue with the next line.
                // We do this before the success case, because we don't want to call
                // next_expression() twice

                let _ = self.next_expression_dummy();
                err
            }).and_then(|name| {
                self.next_expression_dummy()
                    .map(|expression| Statement::Let(name, expression))
            })
    }

    fn next_let_statement_identifier(&mut self) -> Result<String, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: ParseErrorExpected::Identifier,
                received: None,
            }).and_then(|token| match token {
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
            }).and_then(|token| match token {
                Token::Assign => Ok(token),
                unexpected => Err(ParseError {
                    expected: ParseErrorExpected::Assignment,
                    received: Some(unexpected),
                }),
            })
    }

    fn next_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_expression_dummy().map(|x| Statement::Return(x))
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

    fn next_expression_dummy(&mut self) -> Result<Expression, ParseError> {
        // TODO temporary hack to chomp up stuff until the semicolon to make
        // tests pass
        loop {
            let current = self.lexer.next();
            if let Some(token) = current {
                if let Token::Semicolon = token {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(Expression::DummyExpression)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
