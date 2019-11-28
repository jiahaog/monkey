use crate::token::{Token, Token::*};
use std::iter::Peekable;
use std::str::Chars;

#[cfg(test)]
mod tests;

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            iter: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.iter.peek() {
            Some(&ch) => match ch {
                ch if is_whitespace(ch) => {
                    self.iter.next();
                    return self.next_token();
                }
                ch if is_symbol(ch) => Some(self.next_symbol()),
                ch if ch.is_alphabetic() => Some(self.next_identifier()),
                ch if ch.is_digit(10) => Some(self.next_int()),
                '"' => {
                    self.iter.next();
                    Some(self.next_str())
                }
                _ => {
                    self.iter.next();
                    Some(Illegal(ch.to_string()))
                }
            },
            None => None,
        }
    }

    fn next_symbol(&mut self) -> Token {
        match self.iter.next() {
            Some('=') => match self.iter.peek() {
                Some('=') => {
                    self.iter.next();
                    Equal
                }
                _ => Assign,
            },
            Some('+') => Plus,
            Some('(') => LParen,
            Some(')') => RParen,
            Some('{') => LBrace,
            Some('}') => RBrace,
            Some(',') => Comma,
            Some(';') => Semicolon,
            Some('-') => Minus,
            Some('!') => match self.iter.peek() {
                Some('=') => {
                    self.iter.next();
                    NotEqual
                }
                _ => Bang,
            },
            Some('*') => Asterisk,
            Some('/') => Slash,
            Some('<') => LessThan,
            Some('>') => GreaterThan,
            Some(ch) => Illegal(ch.to_string()),
            None => panic!("None matched for next_symbol"),
        }
    }

    fn next_identifier(&mut self) -> Token {
        consume_while(|ch| ch.is_alphabetic(), &mut self.iter).into()
    }

    fn next_int(&mut self) -> Token {
        let literal = consume_while(|ch| ch.is_alphanumeric(), &mut self.iter);
        match literal.parse() {
            Ok(val) => Int(val),
            Err(_) => Illegal(literal),
        }
    }

    fn next_str(&mut self) -> Token {
        let literal = consume_while(|ch| ch != '"', &mut self.iter);
        self.iter.next();
        Str(literal)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n'
}

fn is_symbol(ch: char) -> bool {
    match ch {
        '=' | '+' | '(' | ')' | '{' | '}' | ',' | ';' | '-' | '!' | '*' | '/' | '<' | '>' => true,
        _ => false,
    }
}

fn consume_while<F>(condition: F, iter: &mut Peekable<Chars>) -> String
where
    F: Fn(char) -> bool,
{
    let mut literal = String::new();

    while let Some(&ch) = iter.peek() {
        if !condition(ch) {
            break;
        }

        literal.push(ch);
        iter.next();
    }
    literal
}
