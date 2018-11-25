use ast::{Expression, Expression::DummyExpression, Statement, Statement::*};
use lexer::Lexer;
use parser::{ParseError, Parser};
use token::Token;

#[test]
fn test_let_statements() {
    let inp = "let x = 5;
    let y = 10;
    let foobar = 838383;";

    test_parser_success(
        inp,
        vec![
            LetStatement("x".to_string(), DummyExpression),
            LetStatement("y".to_string(), DummyExpression),
            LetStatement("foobar".to_string(), DummyExpression),
        ],
    );
}

#[test]
fn test_return_statements() {
    let inp = "return 5;
    return 10;";

    test_parser_success(
        inp,
        vec![
            ReturnStatement(DummyExpression),
            ReturnStatement(DummyExpression),
        ],
    );
}

#[test]
fn test_let_wrong_identifier() {
    let inp = "let 1";
    test_parser_error(
        inp,
        vec![ParseError {
            expected: Token::Identifier("IDENTIFIER".to_string()),
            received: Some(Token::Int(1)),
        }],
    );
}

#[test]
fn test_let_no_identifier() {
    let inp = "let";
    test_parser_error(
        inp,
        vec![ParseError {
            expected: Token::Identifier("IDENTIFIER".to_string()),
            received: None,
        }],
    );
}

#[test]
fn test_let_missing_assign() {
    let inp = "let x 5;";
    test_parser_error(
        inp,
        vec![ParseError {
            expected: Token::Assign,
            received: Some(Token::Int(5)),
        }],
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
//
#[test]
fn test_let_multiple_errors() {
    let inp = "let = 5;
    let y 10;
    let foobar = 838383;";
    test_parser_error(
        inp,
        vec![
            ParseError {
                expected: Token::Identifier("IDENTIFIER".to_string()),
                received: Some(Token::Assign),
            },
            ParseError {
                expected: Token::Assign,
                received: Some(Token::Int(10)),
            },
        ],
    );
}

#[test]
fn test_identifier_expression() {
    let inp = "foo;
    bar;";
    test_parser_success(
        inp,
        vec![
            Statement::ExpressionStatement(Expression::Identifier("foo".to_string())),
            Statement::ExpressionStatement(Expression::Identifier("bar".to_string())),
        ],
    );
}

#[test]
fn test_identifier_expression_no_semicolon() {
    let inp = "foo
    bar";
    test_parser_success(
        inp,
        vec![
            Statement::ExpressionStatement(Expression::Identifier("foo".to_string())),
            Statement::ExpressionStatement(Expression::Identifier("bar".to_string())),
        ],
    );
}

fn test_parser_success(inp: &str, expected: Vec<Statement>) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    for (i, exp_statement) in expected.iter().enumerate() {
        assert_eq!(*exp_statement, program.statements[i]);
    }
}

fn test_parser_error(inp: &str, expected_err: Vec<ParseError>) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let err = parser.parse().expect_err("Expect parse errors");
    assert_eq!(err, expected_err);
}
