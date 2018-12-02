use ast::{Expression, Operator};

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
    ];

    for (inp, expected) in cases {
        let received = format!("{}", inp);
        assert_eq!(expected, received);
    }
}
