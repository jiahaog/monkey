use crate::ast::{Expression, Operator, Statement};
use crate::eval::Eval;
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

fn test_eval(inp: &str, expected: Object) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    let received = program.eval();

    assert_eq!(expected, received);
}
