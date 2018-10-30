#![allow(dead_code)]

pub type TokenType<'a> = &'a str;

pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub literal: String,
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";
pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";
pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType<'a>, literal: String) -> Token<'a> {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }

    pub fn from_literal(literal: String) -> Token<'a> {
        let token_type = match literal.as_ref() {
            "fn" => FUNCTION,
            "let" => LET,
            _ => IDENT,
        };

        Token {
            token_type,
            literal,
        }
    }
}
