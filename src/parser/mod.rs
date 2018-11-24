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

struct Parser<'a> {
    iter: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            iter: lexer.peekable(),
        }
    }

    fn parse(self) -> Program {
        Program::new(self.collect())
    }

    fn next_statement(&mut self) -> Option<Statement> {
        // Ugly hack because peek doesn't work well with match
        // https://stackoverflow.com/a/26927642/5076225
        if let Some(true) = self.iter.peek().map(|val| match val {
            Let => true,
            _ => false,
        }) {
            self.iter.next();
            self.next_let_statement()
        } else {
            None
        }
    }

    fn next_let_statement(&mut self) -> Option<Statement> {
        self.iter
            .next()
            .and_then(|token| match token {
                Identifier(name) => Some(name),
                _ => {
                    println!("Expected identifier");
                    None
                }
            }).and_then(|name| {
                self.iter
                    .next()
                    .filter(|token| match token {
                        Assign => true,
                        _ => false,
                    }).and_then(|_| self.next_expression())
                    .map(|expression| LetStatement(name, expression))
            })
    }

    fn next_expression(&mut self) -> Option<Expression> {
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
        Some(DummyExpression)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
