use crate::ast::{Expression, Operator, Statement, Statements};
use crate::lexer::Lexer;
use crate::parser::{ParseError, ParseErrorExpected, Parser};
use crate::token::Token;

#[test]
fn test_let_statements() {
    let inp = "let x = 5;
    let y = 10;";

    test_parser_success(
        inp,
        vec![
            Statement::Let("x".to_string(), Expression::IntegerLiteral(5)),
            Statement::Let("y".to_string(), Expression::IntegerLiteral(10)),
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
            Statement::Return(Expression::IntegerLiteral(5)),
            Statement::Return(Expression::IntegerLiteral(10)),
        ],
    );
}

#[test]
fn test_let_wrong_identifier() {
    let inp = "let 1";
    test_parser_error(
        inp,
        vec![ParseError {
            expected: ParseErrorExpected::Identifier,
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
            expected: ParseErrorExpected::Identifier,
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
            expected: ParseErrorExpected::Assignment,
            received: Some(Token::Int(5)),
        }],
    );
}

#[test]
fn test_let_missing_expression() {
    let inp = "let x = ;";
    test_parser_error(
        inp,
        vec![ParseError {
            expected: ParseErrorExpected::PrefixTokenOrExpression,
            received: Some(Token::Semicolon),
        }],
    );
}

#[test]
fn test_let_multiple_errors() {
    let inp = "let = 5;
    let y 10;
    let foobar = 838383;";
    test_parser_error(
        inp,
        vec![
            ParseError {
                expected: ParseErrorExpected::Identifier,
                received: Some(Token::Assign),
            },
            ParseError {
                expected: ParseErrorExpected::Assignment,
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
            Statement::Expression(Expression::Identifier("foo".to_string())),
            Statement::Expression(Expression::Identifier("bar".to_string())),
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
            Statement::Expression(Expression::Identifier("foo".to_string())),
            Statement::Expression(Expression::Identifier("bar".to_string())),
        ],
    );
}

#[test]
fn test_integer_literal_expression() {
    let inp = "1;
    2;";
    test_parser_success(
        inp,
        vec![
            Statement::Expression(Expression::IntegerLiteral(1)),
            Statement::Expression(Expression::IntegerLiteral(2)),
        ],
    );
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
        test_parser_success(inp, expected);
    }
}

#[test]
fn test_prefix_expressions_error() {
    let cases = vec![
        (
            "-;",
            vec![ParseError {
                expected: ParseErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Semicolon),
            }],
        ),
        (
            "-",
            vec![ParseError {
                expected: ParseErrorExpected::Expression,
                received: None,
            }],
        ),
        (
            "let a = return 1",
            vec![ParseError {
                expected: ParseErrorExpected::PrefixTokenOrExpression,
                received: Some(Token::Return),
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
        test_parser_success(inp, expected);
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
        test_parser_success(inp, expected);
    }
}

#[test]
fn test_func_expression() {
    let cases = vec![
        (
            "fn() { x + y; }",
            vec![Statement::Expression(Expression::FunctionLiteral {
                params: vec![],
                body: vec![Statement::Expression(Expression::Infix {
                    operator: Operator::Plus,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                })],
            })],
        ),
        (
            "fn(x, y) { x + y; }",
            vec![Statement::Expression(Expression::FunctionLiteral {
                params: vec![
                    Expression::Identifier("x".to_string()),
                    Expression::Identifier("y".to_string()),
                ],
                body: vec![Statement::Expression(Expression::Infix {
                    operator: Operator::Plus,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                })],
            })],
        ),
        (
            "fn(x, y) { x + y; true; }",
            vec![Statement::Expression(Expression::FunctionLiteral {
                params: vec![
                    Expression::Identifier("x".to_string()),
                    Expression::Identifier("y".to_string()),
                ],
                body: vec![
                    Statement::Expression(Expression::Infix {
                        operator: Operator::Plus,
                        left: Box::new(Expression::Identifier("x".to_string())),
                        right: Box::new(Expression::Identifier("y".to_string())),
                    }),
                    Statement::Expression(Expression::Boolean(true)),
                ],
            })],
        ),
    ];

    for (inp, expected) in cases {
        test_parser_success(inp, expected);
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
    ];

    for (inp, expected) in cases {
        test_parser_success_with_str(inp, expected);
    }
}

#[test]
fn test_boolean_expression() {
    let inp = "true;
    false;";
    test_parser_success(
        inp,
        vec![
            Statement::Expression(Expression::Boolean(true)),
            Statement::Expression(Expression::Boolean(false)),
        ],
    );
}

fn test_parser_success(inp: &str, expected: Statements) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    for (i, exp_statement) in expected.iter().enumerate() {
        assert_eq!(*exp_statement, program.statements[i]);
    }
}

// Currently expect only one statement which is an expression statement
// Use this instead of having to manually write out the AST
fn test_parser_success_with_str(inp: &str, expected: &str) {
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

fn test_parser_error(inp: &str, expected_err: Vec<ParseError>) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let err = parser.parse().expect_err("Expect parse errors");
    assert_eq!(err, expected_err);
}
