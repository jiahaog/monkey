use lexer::Lexer;
use token;

fn test_lexer(input: &str, expected: Vec<(token::TokenType, &str)>) {
    let mut lexer = Lexer::new(input);

    for (exp_token, exp_literal) in expected {
        let token = lexer.next_token();
        assert_eq!(token.literal, exp_literal);
        assert_eq!(token.token_type, exp_token);
    }
}

#[test]
fn test_next_token_symbols() {
    assert_eq!(2 + 2, 4);

    let input = "=+(){},;";

    let expected = vec![
        (token::ASSIGN, "="),
        (token::PLUS, "+"),
        (token::LPAREN, "("),
        (token::RPAREN, ")"),
        (token::LBRACE, "{"),
        (token::RBRACE, "}"),
        (token::COMMA, ","),
        (token::SEMICOLON, ";"),
        (token::EOF, ""),
    ];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_keyword() {
    let input = "let";
    let expected = vec![(token::LET, "let"), (token::EOF, "")];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_identifier() {
    let input = "five";
    let expected = vec![(token::IDENT, "five")];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_int() {
    let input = "123";
    let expected = vec![(token::INT, "123")];

    test_lexer(input, expected);
}

#[test]
fn test_next_token_extended() {
    assert_eq!(2 + 2, 4);

    let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
";

    let expected = vec![
        (token::LET, "let"),
        (token::IDENT, "five"),
        (token::ASSIGN, "="),
        (token::INT, "5"),
        (token::SEMICOLON, ";"),
        (token::LET, "let"),
        (token::IDENT, "ten"),
        (token::ASSIGN, "="),
        (token::INT, "10"),
        (token::SEMICOLON, ";"),
        (token::LET, "let"),
        (token::IDENT, "add"),
        (token::ASSIGN, "="),
        (token::FUNCTION, "fn"),
        (token::LPAREN, "("),
        (token::IDENT, "x"),
        (token::COMMA, ","),
        (token::IDENT, "y"),
        (token::RPAREN, ")"),
        (token::LBRACE, "{"),
        (token::IDENT, "x"),
        (token::PLUS, "+"),
        (token::IDENT, "y"),
        (token::SEMICOLON, ";"),
        (token::RBRACE, "}"),
        (token::SEMICOLON, ";"),
        (token::LET, "let"),
        (token::IDENT, "result"),
        (token::ASSIGN, "="),
        (token::IDENT, "add"),
        (token::LPAREN, "("),
        (token::IDENT, "five"),
        (token::COMMA, ","),
        (token::IDENT, "ten"),
        (token::RPAREN, ")"),
        (token::SEMICOLON, ";"),
        //
        // EXTENDED
        // (token::BANG, "!"),
        // (token::MINUS, "-"),
        // (token::SLASH, "/"),
        // (token::ASTERISK, "*"),
        // (token::INT, "5"),
        // (token::SEMICOLON, ";"),
        // (token::INT, "5"),
        // (token::LT, "<"),
        // (token::INT, "10"),
        // (token::GT, ">"),
        // (token::INT, "5"),
        // (token::SEMICOLON, ";"),
        // (token::IF, "if"),
        // (token::LPAREN, "("),
        // (token::INT, "5"),
        // (token::LT, "<"),
        // (token::INT, "10"),
        // (token::RPAREN, ")"),
        // (token::LBRACE, "("),
        // (token::RETURN, "return"),
        // (token::TRUE, "true"),
        // (token::SEMICOLON, ";"),
        // (token::RBRACE, ")"),
        // (token::ELSE, "else"),
        // (token::LBRACE, "("),
        // (token::RETURN, "return"),
        // (token::FALSE, "false"),
        // (token::SEMICOLON, ";"),
        // (token::RBRACE, ")"),
        // (token::INT, "10"),
        // (token::EQ, "=="),
        // (token::INT, "10"),
        // (token::SEMICOLON, ";"),
        // (token::INT, "10"),
        // (token::NOT_EQ, "!="),
        // (token::INT, "9"),
        // (token::SEMICOLON, ";"),
        // (token::EOF, ""),
    ];

    test_lexer(input, expected);
}
