#![allow(dead_code)]
use std::iter::Peekable;
use std::str::Chars;
use token::*;

struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input: input,
            iter: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token<'a> {
        while let Some(&ch) = self.iter.peek() {
            if ch != ' ' && ch != '\t' && ch != '\n' {
                break;
            }

            self.iter.next();
        }

        match self.iter.next() {
            Some('=') => Token::new(ASSIGN, String::from("=")),
            Some('+') => Token::new(PLUS, String::from("+")),
            Some('(') => Token::new(LPAREN, String::from("(")),
            Some(')') => Token::new(RPAREN, String::from(")")),
            Some('{') => Token::new(LBRACE, String::from("{")),
            Some('}') => Token::new(RBRACE, String::from("}")),
            Some(',') => Token::new(COMMA, String::from(",")),
            Some(';') => Token::new(SEMICOLON, String::from(";")),
            None => Token::new(EOF, String::from("")),
            Some(token) => if token.is_alphabetic() {
                let mut literal = String::new();

                literal.push(token);

                while let Some(&ch) = self.iter.peek() {
                    if !ch.is_alphabetic() {
                        break;
                    }

                    literal.push(ch);
                    self.iter.next();
                }

                Token::from_literal(literal)
            } else {
                Token::new(ILLEGAL, String::from(""))
            },
        }
    }

    fn next_identifier(&mut self) -> Token<'a> {
        Token::new(EOF, String::from(""))
    }
}

#[cfg(test)]
mod tests;
