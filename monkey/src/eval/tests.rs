use crate::ast::{Expression, Operator, Statement};
use crate::eval::{object, Env, Error, Object};
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::rc::Rc;

#[test]
fn test_eval_integer() {
    let cases = vec![("5", Object::Integer(5))];

    for (inp, expected) in cases {
        test_eval(expected, inp);
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
        test_eval(expected, inp);
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
        test_eval(expected, inp);
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
        test_eval(expected, inp);
    }
}

#[test]
fn test_eval_string_expr() {
    let cases = vec![
        (r#""foo""#, Object::Str("foo".to_string())),
        (r#""foo" + "bar"#, Object::Str("foobar".to_string())),
        (r#""foo" == "foo"#, Object::Boolean(true)),
        (r#""foo" == "bar"#, Object::Boolean(false)),
    ];
    for (inp, expected) in cases {
        test_eval(expected, inp);
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
        test_eval(expected, inp);
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
        test_eval(expected, inp);
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
        test_eval_error(expected, inp);
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
        test_eval(expected, inp);
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
        test_eval_error(expected, inp);
    }
}

#[test]
fn test_list_expr() {
    let cases = vec![
        (
            "[1, 2];",
            Object::List(vec![Object::Integer(1), Object::Integer(2)]),
        ),
        (
            r#"[1 + 3, "abc"];"#,
            Object::List(vec![Object::Integer(4), Object::Str("abc".to_string())]),
        ),
    ];

    for (inp, expected) in cases {
        test_eval(expected, inp);
    }
}

#[test]
fn test_list_index_expr() {
    let cases = vec![
        ("[1, 2, 3][0]", Object::Integer(1)),
        ("[1, 2, 3][1]", Object::Integer(2)),
        ("[1, 2, 3][2]", Object::Integer(3)),
        ("let i = 0; [1][i];", Object::Integer(1)),
        ("[1, 2, 3][1 + 1];", Object::Integer(3)),
        ("let myArray = [1, 2, 3]; myArray[2];", Object::Integer(3)),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            Object::Integer(6),
        ),
        (
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
            Object::Integer(2),
        ),
        ("[1, 2, 3][3]", Object::Null),
    ];

    for (inp, expected) in cases {
        test_eval(expected, inp);
    }
}
#[test]
fn test_fn_object() {
    let cases = vec![(
        "fn(x, y) { x + y }",
        Object::Function(object::Function {
            params: Rc::new(vec!["x".to_string(), "y".to_string()]),
            body: Rc::new(vec![Statement::Expression(Expression::Infix {
                operator: Operator::Plus,
                left: Box::new(Expression::Identifier("x".to_string())),
                right: Box::new(Expression::Identifier("y".to_string())),
            })]),
            env: Env::new(),
        }),
    )];

    for (inp, expected) in cases {
        test_eval(expected, inp);
    }
}

#[test]
fn test_fn_let_statment_returns_null() {
    let cases = vec![("let a = fn() { 1 }", object::NULL)];

    for (inp, expected) in cases {
        test_eval(expected, inp);
    }
}

#[test]
#[should_panic]
fn test_fn_stack_overflow_during_fmt_debug() {
    // NULL doesn't match the identifier for `a`, and the debug statement should be triggered by
    // assert equals. Since functions have a rc to the environment, the debug statement will
    // recursively call itself and stack overflow unless overriden. (Stack overflow != panic)
    let cases = vec![("let a = fn() { 1 }; a", object::NULL)];

    for (inp, expected) in cases {
        test_eval(expected, inp);
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
        test_eval(expected, inp);
    }
}

#[test]
fn test_enclosing_env() {
    let inp = "
        let first = 10;
        let second = 10;
        let third = 10;

        let ourFunction = fn(first) {
          let second = 20;

          first + second + third;
        };

        ourFunction(20) + first + second;";

    let expected = Object::Integer(70);
    test_eval(expected, inp);
}

#[test]
fn test_closures() {
    let inp = "
        let newAdder = fn(x) {
          fn(y) { x + y };
        };

        let addTwo = newAdder(2);
        addTwo(3);";

    let expected = Object::Integer(5);
    test_eval(expected, inp);
}

#[test]
fn test_object_function_display() {
    let inp = "let a = fn(x, y) {
        let b = x + y;
        c
    };

    a
    ";

    let expected = "fn(x, y) {
    let b = (x + y);
    c;
}";

    match eval(inp) {
        Err(received) => panic!("Received {:?} was not expected", received),
        Ok(received) => assert_eq!(expected, format!("{}", received)),
    }
}

#[test]
fn test_eval_builtin_expr() {
    let cases = vec![
        (r#"len("")"#, Object::Integer(0)),
        (r#"len("four")"#, Object::Integer(4)),
        (r#"len("hello world")"#, Object::Integer(11)),
        ("len([1,2])", Object::Integer(2)),
        (
            "push([0], 3)",
            Object::List(vec![Object::Integer(0), Object::Integer(3)]),
        ),
        ("rest([1,2])", Object::List(vec![Object::Integer(2)])),
        ("rest([])", Object::Null),
        ("rest(rest([1]))", Object::Null),
    ];

    for (inp, expected) in cases {
        test_eval(expected, inp);
    }
}

#[test]
fn test_eval_list_iter_expr() {
    let iter_helpers = "
let map = fn(arr, f) {
  let iter = fn(arr, accumulated) {
    len(arr)
    if (len(arr) == 0) {
      accumulated
    } else {
      iter(rest(arr), push(accumulated, f(arr[0])))
    }
  };

  iter(arr, [])
};

let reduce = fn(arr, initial, f) {
  let iter = fn(arr, result) {
    if (len(arr) == 0) {
      result
    } else {
      iter(rest(arr), f(result, arr[0]));
    }
  };

  iter(arr, initial);
};
    ";

    let cases = vec![
        (
            "
let a = [1, 2, 3, 4];
let double = fn(x) { x * 2 };
map(a, double);
        ",
            Object::List(vec![
                Object::Integer(2),
                Object::Integer(4),
                Object::Integer(6),
                Object::Integer(8),
            ]),
        ),
        (
            "
let sum = fn(arr) {
  reduce(arr, 0, fn(initial, el) { initial + el });
};
sum([1,2,3,4,5])
        ",
            Object::Integer(15),
        ),
    ];

    for (inp, expected) in cases {
        test_eval(expected, &(String::from(iter_helpers) + inp));
    }
}

#[test]
fn test_eval_builtin_expr_error() {
    let cases = vec![
        (
            r#"len(1)"#,
            Error::TypeError {
                message: "object of type 'int' has no len()".to_string(),
            },
        ),
        (
            r#"len("one", "two")"#,
            Error::TypeError {
                message: "len() takes exactly one argument (2 given)".to_string(),
            },
        ),
        (
            "let func = fn(x) { x }; len(func);",
            Error::TypeError {
                message: "object of type 'function' has no len()".to_string(),
            },
        ),
        (
            r#"[]["string index"]"#,
            Error::TypeError {
                message: "list indices must be integers, not string".to_string(),
            },
        ),
        (
            r#"[][-1]"#,
            Error::TypeError {
                message: "list indices must be positive".to_string(),
            },
        ),
        (
            r#""some string"[1]"#,
            Error::TypeError {
                message: "object of type 'string' has no index".to_string(),
            },
        ),
    ];

    for (inp, expected) in cases {
        test_eval_error(expected, inp);
    }
}

fn test_eval(expected: Object, inp: &str) {
    match eval(inp) {
        Ok(received) => assert_eq!(expected, received),
        Err(received) => panic!("Received {:?} was not expected", received),
    }
}

fn test_eval_error(expected: Error, inp: &str) {
    match eval(inp) {
        Err(received) => assert_eq!(expected, received),
        Ok(received) => panic!(
            "Expected error {:?}, received result {:?}",
            expected, received
        ),
    }
}

fn eval(inp: &str) -> Result<Object, Error> {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    let env = Env::new();

    program.evaluate(env)
}
