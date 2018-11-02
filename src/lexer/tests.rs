use lexer::Lexer;
use token::Token;
use token::Token::*;

fn test_lexer(input: &str, expected: Vec<Token>) {
    let mut lexer = Lexer::new(input);

    for exp_token in expected {
        if let Some(token) = lexer.next_token() {
            assert_eq!(token, exp_token);
        }
    }
}

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
        EOF,
    ];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_keyword() {
    let input = "let";
    let expected = vec![Let, EOF];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_identifier() {
    let input = "five";
    let expected = vec![Identifier("five".to_string())];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_int() {
    let input = "123";
    let expected = vec![Int("123".to_string())];

    test_lexer(input, expected);
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
        Int("5".to_string()),
        Semicolon,
        Let,
        Identifier("ten".to_string()),
        Assign,
        Int("10".to_string()),
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

    test_lexer(input, expected);
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
";

    let expected = vec![
        Bang,
        Minus,
        Slash,
        Asterisk,
        Int("5".to_string()),
        Semicolon,
        Int("5".to_string()),
        LessThan,
        Int("10".to_string()),
        GreaterThan,
        Int("5".to_string()),
        Semicolon,
        If,
        LParen,
        Int("5".to_string()),
        LessThan,
        Int("10".to_string()),
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
        Int("10".to_string()),
        Equal,
        Int("10".to_string()),
        Semicolon,
        Int("10".to_string()),
        NotEqual,
        Int("9".to_string()),
        Semicolon,
        EOF,
    ];

    test_lexer(input, expected);
}
