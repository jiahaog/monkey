use crate::ast::{CallFunctionExpression, Expression, Function, Operator, Statement};

#[test]
fn test_display() {
    let cases = vec![
        (
            Expression::Infix {
                operator: Operator::Minus,
                left: Box::new(Expression::Infix {
                    operator: Operator::Plus,
                    left: Box::new(Expression::IntegerLiteral(5)),
                    right: Box::new(Expression::IntegerLiteral(6)),
                }),
                right: Box::new(Expression::IntegerLiteral(7)),
            },
            "((5 + 6) - 7)",
        ),
        (
            Expression::Prefix {
                operator: Operator::Not,
                right: Box::new(Expression::Prefix {
                    operator: Operator::Minus,
                    right: Box::new(Expression::Identifier("a".to_string())),
                }),
            },
            "(!(-a))",
        ),
        (
            Expression::StringLiteral("hello world".to_string()),
            r#""hello world""#,
        ),
        (
            Expression::ListLiteral(vec![
                Expression::StringLiteral("bar".to_string()),
                Expression::IntegerLiteral(2),
            ]),
            r#"["bar", 2]"#,
        ),
        (
            Expression::FunctionLiteral(Function {
                params: vec!["x".to_string(), "y".to_string()],
                body: vec![
                    Statement::Expression(Expression::Infix {
                        operator: Operator::Plus,
                        left: Box::new(Expression::Identifier("x".to_string())),
                        right: Box::new(Expression::Identifier("y".to_string())),
                    }),
                    Statement::Expression(Expression::Boolean(true)),
                ],
            }),
            "fn(x, y) { (x + y), true }",
        ),
        (
            Expression::Call {
                function: CallFunctionExpression::Literal(Function {
                    params: vec!["x".to_string(), "y".to_string()],
                    body: vec![
                        Statement::Expression(Expression::Infix {
                            operator: Operator::Plus,
                            left: Box::new(Expression::Identifier("x".to_string())),
                            right: Box::new(Expression::Identifier("y".to_string())),
                        }),
                        Statement::Expression(Expression::Boolean(true)),
                    ],
                }),
                arguments: vec![Expression::Infix {
                    operator: Operator::Plus,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                }],
            },
            "fn(x, y) { (x + y), true }((x + y))",
        ),
        (
            Expression::Call {
                function: CallFunctionExpression::Identifier("something".to_string()),
                arguments: vec![Expression::Infix {
                    operator: Operator::Plus,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                }],
            },
            "something((x + y))",
        ),
        (
            Expression::Index {
                left: Expression::ListLiteral(vec![
                    Expression::StringLiteral("bar".to_string()),
                    Expression::IntegerLiteral(2),
                ])
                .into(),
                index: Expression::IntegerLiteral(0).into(),
            },
            r#"(["bar", 2][0])"#,
        ),
    ];

    for (inp, expected) in cases {
        let received = format!("{}", inp);
        assert_eq!(expected, received);
    }
}
