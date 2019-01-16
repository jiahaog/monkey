use crate::ast::{Expression, Operator, Statement};
use crate::eval::{Env, Error};
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::Parser;

#[test]
fn test_eval_integer() {
    let cases = vec![("5", Object::Integer(5))];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_eval_boolean_expr() {
    let cases = vec![
        ("true", Object::Boolean(true)),
        ("false", Object::Boolean(false)),
        ("1 < 2", Object::Boolean(true)),
        ("1 > 2", Object::Boolean(false)),
        ("1 < 1", Object::Boolean(false)),
        ("1 > 1", Object::Boolean(false)),
        ("1 == 1", Object::Boolean(true)),
        ("1 != 1", Object::Boolean(false)),
        ("1 == 2", Object::Boolean(false)),
        ("1 != 2", Object::Boolean(true)),
        ("true == true", Object::Boolean(true)),
        ("false == false", Object::Boolean(true)),
        ("true == false", Object::Boolean(false)),
        ("true != false", Object::Boolean(true)),
        ("false != true", Object::Boolean(true)),
        ("(1 < 2) == true", Object::Boolean(true)),
        ("(1 < 2) == false", Object::Boolean(false)),
        ("(1 > 2) == true", Object::Boolean(false)),
        ("(1 > 2) == false", Object::Boolean(true)),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_eval_prefix_expr() {
    let cases = vec![
        ("!true", Object::Boolean(false)),
        ("!false", Object::Boolean(true)),
        ("!5", Object::Boolean(false)),
        ("!!true", Object::Boolean(true)),
        ("!!false", Object::Boolean(false)),
        ("!!5", Object::Boolean(true)),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_eval_integer_expr() {
    let cases = vec![
        ("5", Object::Integer(5)),
        ("-5", Object::Integer(-5)),
        ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
        ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
        ("-50 + 100 + -50", Object::Integer(0)),
        ("5 * 2 + 10", Object::Integer(20)),
        ("5 + 2 * 10", Object::Integer(25)),
        ("20 + 2 * -10", Object::Integer(0)),
        ("50 / 2 * 2 + 10", Object::Integer(60)),
        ("2 * (5 + 10)", Object::Integer(30)),
        ("3 * 3 * 3 + 10", Object::Integer(37)),
        ("3 * (3 * 3) + 10", Object::Integer(37)),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_eval_if_else_expr() {
    let cases = vec![
        ("if (true) { 10 }", Object::Integer(10)),
        ("if (false) { 10 }", Object::Null),
        ("if (1) { 10 }", Object::Integer(10)),
        ("if (1 < 2) { 10 }", Object::Integer(10)),
        ("if (1 > 2) { 10 }", Object::Null),
        ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
        ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_eval_return_expr() {
    let cases = vec![
        ("return 10;", Object::Integer(10)),
        ("return 10; 9;", Object::Integer(10)),
        ("return 2 * 5; 9;", Object::Integer(10)),
        ("9; return 2 * 5; 9;", Object::Integer(10)),
        ("if (10 > 1) { return 10; }", Object::Integer(10)),
        (
            "
            if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }

                return 1;
            }
           ",
            Object::Integer(10),
        ),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_error_expr() {
    let cases = vec![
        (
            "5 + true;",
            Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Integer(5),
                right: Object::Boolean(true),
            },
        ),
        (
            "5 + true; 5;",
            Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Integer(5),
                right: Object::Boolean(true),
            },
        ),
        (
            "-true;",
            Error::UnknownOperation {
                operator: Operator::Minus,
                right: Object::Boolean(true),
            },
        ),
        (
            "true + false;",
            Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
        (
            "5; true + false; 5;",
            Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
        (
            "if (10 > 1) { true + false; }",
            Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
        (
            "
            if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }

                return 1;
            }
           ",
            Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
    ];

    for (inp, expected) in cases {
        test_eval_error(inp, expected);
    }
}

#[test]
fn test_let_expr() {
    let cases = vec![
        ("let a = 5; a;", Object::Integer(5)),
        ("let a = 5 * 5; a;", Object::Integer(25)),
        ("let a = 1; let b = 2; a + b", Object::Integer(3)),
        ("let a = 1; let b = a + a; b;", Object::Integer(2)),
        ("let a = 1; let b = a; a + b", Object::Integer(2)),
        ("let a = 5; let b = a; b;", Object::Integer(5)),
        (
            "let a = 5; let b = a; let c = a + b + 5; c;",
            Object::Integer(15),
        ),
        // This tests for cloning of variables
        ("let a = 1; let b = a; let a = 2; b", Object::Integer(1)),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_let_expr_error() {
    let cases = vec![(
        "foobar;",
        Error::IdentifierNotFound {
            name: "foobar".to_string(),
        },
    )];

    for (inp, expected) in cases {
        test_eval_error(inp, expected);
    }
}

#[test]
fn test_fn_object() {
    let cases = vec![(
        "fn(x, y) { x + y }",
        Object::Function {
            params: vec!["x".to_string(), "y".to_string()],
            body: vec![Statement::Expression(Expression::Infix {
                operator: Operator::Plus,
                left: Box::new(Expression::Identifier("x".to_string())),
                right: Box::new(Expression::Identifier("y".to_string())),
            })],
        },
    )];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

#[test]
fn test_fn_expr() {
    let cases = vec![
        (
            "let identity = fn(x) { x; }; identity(5);",
            Object::Integer(5),
        ),
        (
            "let identity = fn(x) { return x; }; identity(5);",
            Object::Integer(5),
        ),
        (
            "let double = fn(x) { x * 2; }; double(5);",
            Object::Integer(10),
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5, 5);",
            Object::Integer(10),
        ),
        ("fn(x) { x; }(5)", Object::Integer(5)),
        (
            "let double = fn(x) { x * 2; }; double(double(2));",
            Object::Integer(8),
        ),
        (
            "let add = fn(x, y) { x + y; }; add(add(3, 4), add(1, 2));",
            Object::Integer(10),
        ),
    ];

    for (inp, expected) in cases {
        test_eval(inp, expected);
    }
}

fn test_eval(inp: &str, expected: Object) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    match program.evaluate(Env::new()).get_result() {
        Ok(received) => assert_eq!(&expected, received),
        Err(received) => panic!("Received {:?} was not expected", received),
    }
}

fn test_eval_error(inp: &str, expected: Error) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    match program.evaluate(Env::new()).get_result() {
        Err(received) => assert_eq!(&expected, received),
        Ok(received) => panic!(
            "Expected error {:?}, received result {:?}",
            expected, received
        ),
    }
}
