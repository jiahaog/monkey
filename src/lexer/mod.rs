#![allow(dead_code)]
use std::str::Chars;
use token::*;

struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input,
            chars: input.chars(),
        }
    }

    fn next_token(&mut self) -> Token<'a> {
        match self.chars.next() {
            Some('=') => new_token(ASSIGN, "="),
            Some('+') => new_token(PLUS, "+"),
            Some('(') => new_token(LPAREN, "("),
            Some(')') => new_token(RPAREN, ")"),
            Some('{') => new_token(LBRACE, "{"),
            Some('}') => new_token(RBRACE, "}"),
            Some(',') => new_token(COMMA, ","),
            Some(';') => new_token(SEMICOLON, ";"),
            _ => new_token(EOF, ""),
        }
    }
}

#[cfg(test)]
mod tests;
