use crate::ast::{Expression, Operator, Statement};

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
            Expression::FunctionLiteral {
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
            "fn(x, y) { (x + y), true }",
        ),
        (
            Expression::Call {
                function: Box::new(Expression::Identifier("something".to_string())),
                arguments: vec![Expression::Infix {
                    operator: Operator::Plus,
                    left: Box::new(Expression::Identifier("x".to_string())),
                    right: Box::new(Expression::Identifier("y".to_string())),
                }],
            },
            "something((x + y))",
        ),
    ];

    for (inp, expected) in cases {
        let received = format!("{}", inp);
        assert_eq!(expected, received);
    }
}
