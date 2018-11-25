use ast::{Expression, Expression::DummyExpression, Statement, Statement::LetStatement};
use lexer::Lexer;
use parser::{ParseError, Parser};
use token::Token;

#[test]
fn test_let_statements() {
    let inp = "let x = 5;
    let y = 10;
    let foobar = 838383;";

    let expected = vec![
        LetStatement("x".to_string(), DummyExpression),
        LetStatement("y".to_string(), DummyExpression),
        LetStatement("foobar".to_string(), DummyExpression),
    ];

    test_parser_success(inp, expected);
}

#[test]
fn test_let_wrong_identifier() {
    let inp = "let 1";
    test_parser_error(
        inp,
        ParseError {
            expected: Token::Identifier("IDENTIFIER".to_string()),
            received: Some(Token::Int("1".to_string())),
        },
    );
}

#[test]
fn test_let_no_identifier() {
    let inp = "let";
    test_parser_error(
        inp,
        ParseError {
            expected: Token::Identifier("IDENTIFIER".to_string()),
            received: None,
        },
    );
}

#[test]
fn test_let_missing_assign() {
    let inp = "let x 5;";
    test_parser_error(
        inp,
        ParseError {
            expected: Token::Assign,
            received: Some(Token::Int("5".to_string())),
        },
    );
}

// #[test]
// fn test_let_missing_expression() {
//     let inp = "let x = ;";
//     test_parser_error(
//         inp,
//         ParseError {
//             expected: Token::Assign, // ?? idk
//             received: Some(Token::Semicolon),
//         },
//     );
// }

fn test_parser_success(inp: &str, expected: Vec<Statement>) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    for (i, exp_statement) in expected.iter().enumerate() {
        assert_eq!(*exp_statement, program.statements[i]);
    }
}

fn test_parser_error(inp: &str, expected_err: ParseError) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let err = parser.parse().expect_err("Expect parse errors");
    assert_eq!(err, expected_err);
}
