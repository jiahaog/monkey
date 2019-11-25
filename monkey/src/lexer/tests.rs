use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::Token::*;

#[test]
fn test_next_token_symbols() {
    assert_eq!(2 + 2, 4);

    let input = "=+(){},;-!*/<>";

    let expected = vec![
        Assign,
        Plus,
        LParen,
        RParen,
        LBrace,
        RBrace,
        Comma,
        Semicolon,
        Minus,
        Bang,
        Asterisk,
        Slash,
        LessThan,
        GreaterThan,
    ];

    test_lexer(expected, input);
}

#[test]
fn test_next_token_keyword() {
    let input = "let";
    let expected = vec![Let];

    test_lexer(expected, input);
}

#[test]
fn test_next_token_identifier() {
    let input = "five";
    let expected = vec![Identifier("five".to_string())];

    test_lexer(expected, input);
}

#[test]
fn test_next_token_int() {
    let input = "123";
    let expected = vec![Int(123)];

    test_lexer(expected, input);
}

#[test]
fn test_next_token_int_invalid() {
    let input = "12a3";
    let expected = vec![Illegal("12a3".to_string())];

    test_lexer(expected, input);
}

#[test]
fn test_unknown_symbol() {
    let input = ".";
    let expected = vec![Illegal(".".to_string())];

    test_lexer(expected, input);
}

#[test]
fn test_next_token_simple() {
    assert_eq!(2 + 2, 4);

    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
";

    let expected = vec![
        Let,
        Identifier("five".to_string()),
        Assign,
        Int(5),
        Semicolon,
        Let,
        Identifier("ten".to_string()),
        Assign,
        Int(10),
        Semicolon,
        Let,
        Identifier("add".to_string()),
        Assign,
        Function,
        LParen,
        Identifier("x".to_string()),
        Comma,
        Identifier("y".to_string()),
        RParen,
        LBrace,
        Identifier("x".to_string()),
        Plus,
        Identifier("y".to_string()),
        Semicolon,
        RBrace,
        Semicolon,
        Let,
        Identifier("result".to_string()),
        Assign,
        Identifier("add".to_string()),
        LParen,
        Identifier("five".to_string()),
        Comma,
        Identifier("ten".to_string()),
        RParen,
        Semicolon,
    ];

    test_lexer(expected, input);
}

#[test]
fn test_next_token_extended() {
    assert_eq!(2 + 2, 4);

    let input = "!-/*5;
5 < 10 > 5;

if (5 < 10) {
        return true;
} else {
        return false;
}

10 == 10;
10 != 9;
\"foobar\";
\"foo space bar\";
";

    let expected = vec![
        Bang,
        Minus,
        Slash,
        Asterisk,
        Int(5),
        Semicolon,
        Int(5),
        LessThan,
        Int(10),
        GreaterThan,
        Int(5),
        Semicolon,
        If,
        LParen,
        Int(5),
        LessThan,
        Int(10),
        RParen,
        LBrace,
        Return,
        True,
        Semicolon,
        RBrace,
        Else,
        LBrace,
        Return,
        False,
        Semicolon,
        RBrace,
        Int(10),
        Equal,
        Int(10),
        Semicolon,
        Int(10),
        NotEqual,
        Int(9),
        Semicolon,
        Str("foobar".to_string()),
        Semicolon,
        Str("foo space bar".to_string()),
        Semicolon,
    ];

    test_lexer(expected, input);
}

fn test_lexer(expected: Vec<Token>, input: &str) {
    let lexer = Lexer::new(input);

    let received: Vec<Token> = lexer.collect();

    assert_eq!(expected, received);
}
