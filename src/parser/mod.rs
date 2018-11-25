#![allow(dead_code)]

use ast::Expression;
use ast::Expression::DummyExpression;
use ast::Program;
use ast::Statement;
use ast::Statement::{DummyStatement, LetStatement};
use lexer::Lexer;
use std::iter::Peekable;
use token::{Token, Token::*};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
struct ParseError {
    expected: Token,
    received: Option<Token>,
}

struct Parser<'a> {
    // TODO remove the peekable type if it's really unnecessary
    iter: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            iter: lexer.peekable(),
        }
    }

    fn parse(self) -> Result<Program, ParseError> {
        // TODO we should collect all errors into a vec, and not just the first one
        self.collect::<Result<Vec<Statement>, ParseError>>()
            .map(Program::new)
    }

    fn next_statement(&mut self) -> Option<Result<Statement, ParseError>> {
        self.iter.next().map(|token| match token {
            Let => self.next_let_statement(),
            _ => {
                // TODO other kinds of statements
                unimplemented!()
            }
        })
    }

    fn next_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.iter
            .next()
            .ok_or(ParseError {
                expected: Identifier("IDENTIFIER".to_string()),
                received: None,
            }).and_then(|token| match token {
                Identifier(name) => Ok(name),
                unexpected => Err(ParseError {
                    expected: Identifier("IDENTIFIER".to_string()),
                    received: Some(unexpected),
                }),
            }).and_then(|name| {
                self.iter
                    .next()
                    .ok_or(ParseError {
                        expected: Assign,
                        received: None,
                    }).and_then(|token| match token {
                        Assign => Ok(name),
                        unexpected => Err(ParseError {
                            expected: Assign,
                            received: Some(unexpected),
                        }),
                    })
            }).map_err(|err| {
                // Increment the iterator until the semicolon, so that the next call to
                // next_let_statement will continue with the next line.
                // We do this before the success case, because we don't want to call
                // next_expression() twice

                let _ = self.next_expression();
                err
            }).and_then(|name| {
                self.next_expression()
                    .map(|expression| LetStatement(name, expression))
            })
    }

    fn next_expression(&mut self) -> Result<Expression, ParseError> {
        // TODO temporary hack to chomp up stuff until the semicolon to make
        // tests pass
        while true {
            let current = self.iter.next();
            if let Some(token) = current {
                if let Semicolon = token {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(DummyExpression)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<Statement, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
