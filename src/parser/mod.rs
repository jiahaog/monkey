#![allow(dead_code)]

use ast::Expression;
use ast::Expression::DummyExpression;
use ast::Program;
use ast::Statement;
use ast::Statement::{LetStatement, ReturnStatement};
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
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
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
        self.lexer.next().map(|token| match token {
            Let => self.next_let_statement(),
            Return => self.next_return_statement(),
            _ => {
                // TODO other kinds of statements
                unimplemented!()
            }
        })
    }

    fn next_let_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_let_statement_identifier()
            .and_then(|name| self.next_let_statement_assign().map(|_| name))
            .map_err(|err| {
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

    fn next_let_statement_identifier(&mut self) -> Result<String, ParseError> {
        self.lexer
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
            })
    }

    fn next_let_statement_assign(&mut self) -> Result<Token, ParseError> {
        self.lexer
            .next()
            .ok_or(ParseError {
                expected: Assign,
                received: None,
            }).and_then(|token| match token {
                Assign => Ok(token),
                unexpected => Err(ParseError {
                    expected: Assign,
                    received: Some(unexpected),
                }),
            })
    }

    fn next_return_statement(&mut self) -> Result<Statement, ParseError> {
        self.next_expression().map(|x| ReturnStatement(x))
    }

    fn next_expression(&mut self) -> Result<Expression, ParseError> {
        // TODO temporary hack to chomp up stuff until the semicolon to make
        // tests pass
        loop {
            let current = self.lexer.next();
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
