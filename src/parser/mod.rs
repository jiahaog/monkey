#![allow(dead_code)]

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
            self.next_let_statement()
        } else {
            None
        }
    }

    fn next_let_statement(&mut self) -> Option<Statement> {
        None
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_statement()
    }
}
