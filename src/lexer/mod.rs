#![allow(dead_code)]
use std::iter::Peekable;
use std::str::Chars;
use token::*;

#[cfg(test)]
mod tests;

struct Lexer<'a> {
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

    pub fn next_token(&mut self) -> Token<'a> {
        match self.iter.peek() {
            Some(&ch) => match ch {
                ch if is_whitespace(ch) => {
                    self.iter.next();
                    return self.next_token();
                }
                ch if is_symbol(ch) => self.next_symbol(),
                ch if ch.is_alphabetic() => self.next_identifier(),
                ch if ch.is_numeric() => self.next_int(),
                _ => Token::new(ILLEGAL, String::from("")),
            },
            None => Token::new(EOF, String::from("")),
        }
    }

    fn next_symbol(&mut self) -> Token<'a> {
        match self.iter.next() {
            Some('=') => Token::new(ASSIGN, String::from("=")),
            Some('+') => Token::new(PLUS, String::from("+")),
            Some('(') => Token::new(LPAREN, String::from("(")),
            Some(')') => Token::new(RPAREN, String::from(")")),
            Some('{') => Token::new(LBRACE, String::from("{")),
            Some('}') => Token::new(RBRACE, String::from("}")),
            Some(',') => Token::new(COMMA, String::from(",")),
            Some(';') => Token::new(SEMICOLON, String::from(";")),
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

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n'
}

fn is_symbol(ch: char) -> bool {
    match ch {
        '=' | '+' | '(' | ')' | '{' | '}' | ',' | ';' => true,
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
