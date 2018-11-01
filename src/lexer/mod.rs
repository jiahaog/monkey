#![allow(dead_code)]
use std::iter::Peekable;
use std::str::Chars;
use token::*;

#[cfg(test)]
mod tests;

pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input,
            iter: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        match self.iter.peek() {
            Some(&ch) => match ch {
                ch if is_whitespace(ch) => {
                    self.iter.next();
                    return self.next_token();
                }
                ch if is_symbol(ch) => Some(self.next_symbol()),
                ch if ch.is_alphabetic() => Some(self.next_identifier()),
                ch if ch.is_numeric() => Some(self.next_int()),
                _ => Some(Token::new(ILLEGAL, String::from(""))),
            },
            None => None,
        }
    }

    fn next_symbol(&mut self) -> Token<'a> {
        match self.iter.next() {
            Some('=') => match self.iter.peek() {
                Some('=') => {
                    self.iter.next();
                    Token::new(EQ, String::from("=="))
                }
                _ => Token::new(ASSIGN, String::from("=")),
            },
            Some('+') => Token::new(PLUS, String::from("+")),
            Some('(') => Token::new(LPAREN, String::from("(")),
            Some(')') => Token::new(RPAREN, String::from(")")),
            Some('{') => Token::new(LBRACE, String::from("{")),
            Some('}') => Token::new(RBRACE, String::from("}")),
            Some(',') => Token::new(COMMA, String::from(",")),
            Some(';') => Token::new(SEMICOLON, String::from(";")),
            Some('-') => Token::new(MINUS, String::from("-")),
            Some('!') => match self.iter.peek() {
                Some('=') => {
                    self.iter.next();
                    Token::new(NOT_EQ, String::from("!="))
                }
                _ => Token::new(BANG, String::from("!")),
            },
            Some('*') => Token::new(ASTERISK, String::from("*")),
            Some('/') => Token::new(SLASH, String::from("/")),
            Some('<') => Token::new(LT, String::from("<")),
            Some('>') => Token::new(GT, String::from(">")),
            Some(ch) => Token::new(ILLEGAL, ch.to_string()),
            None => panic!("None matched for next_symbol"),
        }
    }

    fn next_identifier(&mut self) -> Token<'a> {
        let literal = consume_while(|ch| ch.is_alphabetic(), &mut self.iter);
        Token::from_literal(literal)
    }

    fn next_int(&mut self) -> Token<'a> {
        let literal = consume_while(|ch| ch.is_numeric(), &mut self.iter);
        Token::new(INT, literal)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

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
