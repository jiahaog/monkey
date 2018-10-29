use lexer::Lexer;
use token;

#[test]
fn test_next_token() {
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

    let mut lexer = Lexer::new(input);

    for (exp_token, exp_literal) in expected {
        let token = lexer.next_token();
        assert_eq!(token.token_type, exp_token);
        assert_eq!(token.literal, exp_literal);
    }
}
