#![allow(dead_code)]

pub type TokenType<'a> = &'a str;

pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub literal: &'a str,
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

pub fn new_token<'a>(token_type: TokenType<'a>, literal: &'a str) -> Token<'a> {
    Token {
        token_type: token_type,
        literal: literal,
    }
}
