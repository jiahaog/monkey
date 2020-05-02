use crate::ast::{Expression, Operator, Statement};
use crate::eval::Error;
use crate::lexer::Lexer;
use crate::object;
use crate::object::{Env, Function, Object, NULL};
use crate::parser::Parser;
use std::rc::Rc;

#[test]
fn test_eval_integer() {
    let cases = vec![("5", 5)];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
    }
}

#[test]
fn test_eval_boolean_expr() {
    let cases = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
    }
}

#[test]
fn test_eval_prefix_expr() {
    let cases = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
    }
}

#[test]
fn test_eval_integer_expr() {
    let cases = vec![
        ("5", 5),
        ("-5", -5),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
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
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        ("if (10 > 1) { return 10; }", 10),
        (
            "
            if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }

                return 1;
            }
           ",
            10,
        ),
    ];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
    }
}

#[test]
fn test_error_expr() {
    let cases = vec![
        (
            "5 + true;",
            object::Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Integer(5),
                right: Object::Boolean(true),
            },
        ),
        (
            "5 + true; 5;",
            object::Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Integer(5),
                right: Object::Boolean(true),
            },
        ),
        (
            "-true;",
            object::Error::UnknownOperation {
                operator: Operator::Minus,
                right: Object::Boolean(true),
            },
        ),
        (
            "true + false;",
            object::Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
        (
            "5; true + false; 5;",
            object::Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
        (
            "if (10 > 1) { true + false; }",
            object::Error::TypeMismatch {
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
            object::Error::TypeMismatch {
                operator: Operator::Plus,
                left: Object::Boolean(true),
                right: Object::Boolean(false),
            },
        ),
    ];

    for (inp, expected) in cases {
        test_eval_error(expected.into(), inp);
    }
}

#[test]
fn test_let_expr() {
    let cases = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 1; let b = 2; a + b", 3),
        ("let a = 1; let b = a + a; b;", 2),
        ("let a = 1; let b = a; a + b", 2),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        // This tests for cloning of variables
        ("let a = 1; let b = a; let a = 2; b", 1),
    ];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
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
        Object::Function(Function {
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
    let cases = vec![("let a = fn() { 1 }", NULL)];

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
    let cases = vec![("let a = fn() { 1 }; a", NULL)];

    for (inp, expected) in cases {
        test_eval(expected, inp);
    }
}

#[test]
fn test_fn_expr() {
    let cases = vec![
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("fn(x) { x; }(5)", 5),
        ("let double = fn(x) { x * 2; }; double(double(2));", 8),
        (
            "let add = fn(x, y) { x + y; }; add(add(3, 4), add(1, 2));",
            10,
        ),
    ];

    for (inp, expected) in cases {
        test_eval(expected.into(), inp);
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

    let expected = 70.into();
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

    let expected = 5.into();
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
fn test_eval_builtin_print() {
    let cases = vec![
        ("print(1)", Ok(Object::Null), vec!["1"]),
        ("print(1, 2); print(3)", Ok(Object::Null), vec!["1 2", "3"]),
        (
            "
let test = fn(x) {
    print(1, 2);
    print(3);
    len(x)
};

test([])
        ",
            Ok(Object::Integer(0)),
            vec!["1 2", "3"],
        ),
        (
            "
let test = fn(x) {
    print(3);
    len(x)
};

test(1)
        ",
            Err(Error::TypeError {
                message: "object of type 'int' has no len()".to_string(),
            }),
            vec!["3"],
        ),
    ];

    for (inp, expected_result, expected_stdout) in cases {
        let lexer = Lexer::new(inp);
        let parser = Parser::new(lexer);

        let program = parser.parse().expect("No parse errors");

        let env = Env::new();

        let (env, result) = program.evaluate(env.clone());
        assert_eq!(expected_result, result);
        assert_eq!(expected_stdout, env.pop_stdout());
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

    let (_, result) = program.evaluate(env);
    result
}
