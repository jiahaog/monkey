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
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";
pub const LT: TokenType = "<";
pub const GT: TokenType = "k";

pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

// keywords
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

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
            "true" => TRUE,
            "false" => FALSE,
            "if" => IF,
            "else" => ELSE,
            "return" => RETURN,
            _ => IDENT,
        };

        Token {
            token_type,
            literal,
        }
    }
}
