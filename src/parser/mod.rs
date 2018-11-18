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
        None

        // TODO this is broken, need to implement Copy on Token
        // match self.iter.peek() {
        //     Some(&token) => match token {
        //         // Let => self.next_let_statement(),
        //         Let => {
        //             self.iter.next();
        //             return None;
        //         }
        //         // TODO: Return statements, expression statements
        //         _ => None,
        //     },
        //     None => None,
        // }
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
