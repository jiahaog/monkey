use crate::ast::{Expression, Function, Operator, Statement, Statements};
use crate::lexer::Lexer;
use crate::parser::{Error, ErrorExpected, Parser};
use crate::token::Token;

#[test]
fn test_let_statements() {
    let inp = "let x = 5;
    let y = 10;";

    test_parser_success(
        vec![
            Statement::Let("x".to_string(), Expression::IntegerLiteral(5)),
            Statement::Let("y".to_string(), Expression::IntegerLiteral(10)),
        ],
        inp,
    );
}

#[test]
fn test_return_statements() {
    let inp = "return 5;
    return 10;";

    test_parser_success(
        vec![
            Statement::Return(Expression::IntegerLiteral(5)),
            Statement::Return(Expression::IntegerLiteral(10)),
        ],
        inp,
    );
}

#[test]
fn test_let_wrong_identifier() {
    let inp = "let 1";
    test_parser_error(
        vec![Error {
            expected: ErrorExpected::Identifier,
            received: Some(Token::Int(1)),
        }],
        inp,
    );
}

#[test]
fn test_let_no_identifier() {
    let inp = "let";
    test_parser_error(
        vec![Error {
            expected: ErrorExpected::Identifier,
            received: None,
        }],
        inp,
    );
}

#[test]
fn test_let_missing_assign() {
    let inp = "let x 5;";
    test_parser_error(
        vec![Error {
            expected: ErrorExpected::Assignment,
            received: Some(Token::Int(5)),
        }],
        inp,
    );
}

#[test]
fn test_let_missing_expression() {
    let inp = "let x = ;";
    test_parser_error(
        vec![Error {
            expected: ErrorExpected::PrefixTokenOrExpression,
            received: Some(Token::Semicolon),
        }],
        inp,
    );
}

#[test]
fn test_let_multiple_errors() {
    let inp = "let = 5;
    let y 10;
    let foobar = 838383;";
    test_parser_error(
        vec![
            Error {
                expected: ErrorExpected::Identifier,
                received: Some(Token::Assign),
            },
            Error {
                expected: ErrorExpected::Assignment,
                received: Some(Token::Int(10)),
            },
        ],
        inp,
    );
}

#[test]
fn test_identifier_expression() {
    let inp = "foo;
    bar;";
    test_parser_success(
        vec![
            Statement::Expression(Expression::Identifier("foo".to_string())),
            Statement::Expression(Expression::Identifier("bar".to_string())),
        ],
        inp,
    );
}

#[test]
fn test_identifier_expression_no_semicolon() {
    let inp = "foo
    bar";
    test_parser_success(
        vec![
            Statement::Expression(Expression::Identifier("foo".to_string())),
            Statement::Expression(Expression::Identifier("bar".to_string())),
        ],
        inp,
    );
}

#[test]
fn test_integer_literal_expression() {
    let inp = "1;
    2;";
    test_parser_success(
        vec![
            Statement::Expression(Expression::IntegerLiteral(1)),
            Statement::Expression(Expression::IntegerLiteral(2)),
        ],
        inp,
    );
}

#[test]
fn test_string_literal_expression() {
    let inp = r#""foo";
    "bar";"#;
    test_parser_success(
        vec![
            Statement::Expression(Expression::StringLiteral("foo".to_string())),
            Statement::Expression(Expression::StringLiteral("bar".to_string())),
        ],
        inp,
    );
}

#[test]
fn test_list_literal_expression() {
    let inp = r#"[1, 2];
    ["bar", 2];
    [2 + 1, 3]"#;
    test_parser_success(
        vec![
            Statement::Expression(Expression::ListLiteral(vec![
                Expression::IntegerLiteral(1),
                Expression::IntegerLiteral(2),
            ])),
            Statement::Expression(Expression::ListLiteral(vec![
                Expression::StringLiteral("bar".to_string()),
                Expression::IntegerLiteral(2),
            ])),
            Statement::Expression(Expression::ListLiteral(vec![
                Expression::Infix {
                    operator: Operator::Plus,
                    left: Expression::IntegerLiteral(2).into(),
                    right: Expression::IntegerLiteral(1).into(),
                },
                Expression::IntegerLiteral(3),
            ])),
        ],
        inp,
    );
}

#[test]
fn test_list_index_expression() {
    let cases = vec![
        (
            "[1, 2][3 * 4];",
            vec![Statement::Expression(Expression::Index {
                left: Expression::ListLiteral(vec![
                    Expression::IntegerLiteral(1),
                    Expression::IntegerLiteral(2),
                ])
                .into(),
                index: Expression::Infix {
                    operator: Operator::Multiply,
                    left: Expression::IntegerLiteral(3).into(),
                    right: Expression::IntegerLiteral(4).into(),
                }
                .into(),
            })],
        ),
        (
            "a * b[2];",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Multiply,
                left: Expression::Identifier("a".to_string()).into(),
                right: Expression::Index {
                    left: Expression::Identifier("b".to_string()).into(),
                    index: Expression::IntegerLiteral(2).into(),
                }
                .into(),
            })],
        ),
        (
            "2 * [1, 2][1];",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Multiply,
                left: Expression::IntegerLiteral(2).into(),
                right: Expression::Index {
                    left: Expression::ListLiteral(vec![
                        Expression::IntegerLiteral(1),
                        Expression::IntegerLiteral(2),
                    ])
                    .into(),
                    index: Expression::IntegerLiteral(1).into(),
                }
                .into(),
            })],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(expected, inp);
    }
}

#[test]
fn test_list_index_expression_error() {
    let cases = vec![
        (
            "[1][];",
            vec![Error {
                expected: ErrorExpected::SingleIndex,
                received: Some(Token::RBracket),
            }],
        ),
        (
            "[1][1,2];",
            vec![Error {
                expected: ErrorExpected::SingleIndex,
                received: Some(Token::Comma),
            }],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_error(expected, inp);
    }
}

#[test]
fn test_prefix_expressions() {
    let cases = vec![
        (
            "!5;",
            vec![Statement::Expression(Expression::Prefix {
                operator: Operator::Not,
                right: Box::new(Expression::IntegerLiteral(5)),
            })],
        ),
        (
            "-!5;",
            vec![Statement::Expression(Expression::Prefix {
                operator: Operator::Minus,
                right: Box::new(Expression::Prefix {
                    operator: Operator::Not,
                    right: Box::new(Expression::IntegerLiteral(5)),
                }),
            })],
        ),
        (
            "-15;",
            vec![Statement::Expression(Expression::Prefix {
                operator: Operator::Minus,
                right: Box::new(Expression::IntegerLiteral(15)),
            })],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(expected, inp);
    }
}

#[test]
fn test_prefix_expressions_error() {
    let cases = vec![
        (
            "-;",
            vec![Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Semicolon),
            }],
        ),
        (
            "-",
            vec![Error {
                expected: ErrorExpected::Expression,
                received: None,
            }],
        ),
        (
            "let a = return 1",
            vec![Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Return),
            }],
        ),
        (
            "123let",
            vec![Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Illegal("123let".to_string())),
            }],
        ),
        (
            "++",
            vec![Error {
                expected: ErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Plus),
            }],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_error(expected, inp);
    }
}

#[test]
fn test_infix_expressions() {
    let cases = vec![
        (
            "5 + 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Plus,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 - 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Minus,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 * 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Multiply,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 / 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Divide,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 < 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::LessThan,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 > 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::GreaterThan,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 == 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::Equal,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
        (
            "5 != 6;",
            vec![Statement::Expression(Expression::Infix {
                operator: Operator::NotEqual,
                left: Box::new(Expression::IntegerLiteral(5)),
                right: Box::new(Expression::IntegerLiteral(6)),
            })],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(expected, inp);
    }
}

#[test]
fn test_if_expression() {
    let cases = vec![
        (
            "if (x < y) { x } else { y }",
            vec![Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix {
                    operator: Operator::LessThan,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                }),
                consequence: vec![Statement::Expression(Expression::Identifier(
                    "x".to_string(),
                ))],
                alternative: vec![Statement::Expression(Expression::Identifier(
                    "y".to_string(),
                ))],
            })],
        ),
        (
            "if (x < y) { x; true; } else { y; false; }",
            vec![Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix {
                    operator: Operator::LessThan,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                }),
                consequence: vec![
                    Statement::Expression(Expression::Identifier("x".to_string())),
                    Statement::Expression(Expression::Boolean(true)),
                ],
                alternative: vec![
                    Statement::Expression(Expression::Identifier("y".to_string())),
                    Statement::Expression(Expression::Boolean(false)),
                ],
            })],
        ),
        (
            "if (x < y) { x; true; }",
            vec![Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix {
                    operator: Operator::LessThan,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                }),
                consequence: vec![
                    Statement::Expression(Expression::Identifier("x".to_string())),
                    Statement::Expression(Expression::Boolean(true)),
                ],
                alternative: vec![],
            })],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(expected, inp);
    }
}

#[test]
fn test_func_expression() {
    let cases = vec![
        (
            "fn() { x + y; }",
            vec![Statement::Expression(Expression::FunctionLiteral(
                Function {
                    params: vec![],
                    body: vec![Statement::Expression(Expression::Infix {
                        operator: Operator::Plus,
                        left: Box::new(Expression::Identifier("x".to_string())),
                        right: Box::new(Expression::Identifier("y".to_string())),
                    })],
                },
            ))],
        ),
        (
            "fn(x, y) { x + y; }",
            vec![Statement::Expression(Expression::FunctionLiteral(
                Function {
                    params: vec!["x".to_string(), "y".to_string()],
                    body: vec![Statement::Expression(Expression::Infix {
                        operator: Operator::Plus,
                        left: Box::new(Expression::Identifier("x".to_string())),
                        right: Box::new(Expression::Identifier("y".to_string())),
                    })],
                },
            ))],
        ),
        (
            "fn(x, y) { x + y; true; }",
            vec![Statement::Expression(Expression::FunctionLiteral(
                Function {
                    params: vec!["x".to_string(), "y".to_string()],
                    body: vec![
                        Statement::Expression(Expression::Infix {
                            operator: Operator::Plus,
                            left: Box::new(Expression::Identifier("x".to_string())),
                            right: Box::new(Expression::Identifier("y".to_string())),
                        }),
                        Statement::Expression(Expression::Boolean(true)),
                    ],
                },
            ))],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(expected, inp);
    }
}

#[test]
fn test_operator_precedence_expression() {
    let cases = vec![
        ("5 + 6 - 7;", "((5 + 6) - 7)"),
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("add(b + c)", "add((b + c))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        (
            "a * [1, 2, 3, 4][b * c] * d",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success_with_str(expected, inp);
    }
}

#[test]
fn test_boolean_expression() {
    let inp = "true;
    false;";
    test_parser_success(
        vec![
            Statement::Expression(Expression::Boolean(true)),
            Statement::Expression(Expression::Boolean(false)),
        ],
        inp,
    );
}

fn test_parser_success(expected: Statements, inp: &str) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    assert_eq!(expected, program.statements);
}

// Currently expect only one statement which is an expression statement
// Use this instead of having to manually write out the AST
fn test_parser_success_with_str(expected: &str, inp: &str) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    if program.statements.len() != 1 {
        panic!("expected only one statement");
    }

    let received = match program.statements[0] {
        Statement::Expression(ref expr) => format!("{}", expr),
        _ => panic!("Expected a expression statement"),
    };

    assert_eq!(expected, received);
}

fn test_parser_error(expected_err: Vec<Error>, inp: &str) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let err = parser.parse().expect_err("Expect parse errors");
    assert_eq!(expected_err, err.errors);
}
