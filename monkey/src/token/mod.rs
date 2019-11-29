use self::Token::*;
use std::convert::From;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal(String),
    Identifier(String),
    Int(usize),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Str(String),
}

impl From<String> for Token {
    fn from(literal: String) -> Self {
        match literal.as_ref() {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Identifier(literal),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Illegal(string) => format!("illegal: {}", &string),
                Identifier(string) => format!("identifier: {}", &string),
                Int(val) => format!("{}", &val),
                Str(string) => format!("\"{}\"", &string),
                Assign => "=".to_string(),
                Plus => "+".to_string(),
                Comma => ",".to_string(),
                Semicolon => ";".to_string(),
                LParen => "(".to_string(),
                RParen => ")".to_string(),
                LBrace => "{".to_string(),
                RBrace => "}".to_string(),
                LBracket => "[".to_string(),
                RBracket => "]".to_string(),
                Minus => "-".to_string(),
                Bang => "!".to_string(),
                Asterisk => "*".to_string(),
                Slash => "/".to_string(),
                LessThan => "<".to_string(),
                GreaterThan => ">".to_string(),
                Equal => "==".to_string(),
                NotEqual => "!=".to_string(),
                Function => "fn".to_string(),
                Let => "let".to_string(),
                True => "true".to_string(),
                False => "false".to_string(),
                If => "if".to_string(),
                Else => "else".to_string(),
                Return => "return".to_string(),
            }
        )
    }
}
