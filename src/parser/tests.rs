use ast::{Expression, Expression::DummyExpression, Operator, Statement, Statement::*};
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

// #[test]
// fn test_identifier_expression_no_semicolon() {
//     let inp = "foo
//     bar";
//     test_parser_success(
//         inp,
//         vec![
//             Statement::ExpressionStatement(Expression::Identifier("foo".to_string())),
//             Statement::ExpressionStatement(Expression::Identifier("bar".to_string())),
//         ],
//     );
// }

// #[test]
// fn test_integer_literal_expression_no_semicolon() {
//     let inp = "1;
//     2";
//     test_parser_success(
//         inp,
//         vec![
//             Statement::ExpressionStatement(Expression::IntegerLiteral(1)),
//             Statement::ExpressionStatement(Expression::IntegerLiteral(2)),
//         ],
//     );
// }

#[test]
fn test_integer_literal_expression() {
    let inp = "1;
    2;";
    test_parser_success(
        inp,
        vec![
            Statement::ExpressionStatement(Expression::IntegerLiteral(1)),
            Statement::ExpressionStatement(Expression::IntegerLiteral(2)),
        ],
    );
}

#[test]
fn test_prefix_expressions() {
    let cases = vec![
        (
            "!5;",
            vec![Statement::ExpressionStatement(
                Expression::PrefixExpression {
                    operator: Operator::Not,
                    right: Box::new(Expression::IntegerLiteral(5)),
                },
            )],
        ),
        (
            "-15;",
            vec![Statement::ExpressionStatement(
                Expression::PrefixExpression {
                    operator: Operator::Minus,
                    right: Box::new(Expression::IntegerLiteral(15)),
                },
            )],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(inp, expected);
    }
}

#[test]
fn test_prefix_expressions_error() {
    let cases = vec![
        (
            "-;",
            vec![ParseError {
                expected: Token::Identifier("IDENTIFIER".to_string()),
                received: Some(Token::Semicolon),
            }],
        ),
        (
            "-",
            vec![ParseError {
                expected: Token::Identifier("rest of expression".to_string()),
                received: None,
            }],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_error(inp, expected);
    }
}

#[test]
fn test_infix_expressions() {
    let cases = vec![
        (
            "5 + 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::Plus,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 - 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::Minus,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 * 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::Multiply,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 / 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::Divide,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 < 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::LessThan,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 > 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::GreaterThan,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 == 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::Equal,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
        (
            "5 != 6;",
            vec![Statement::ExpressionStatement(
                Expression::InfixExpression {
                    operator: Operator::NotEqual,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                },
            )],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(inp, expected);
    }
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
