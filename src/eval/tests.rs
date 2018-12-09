use crate::ast::{Expression, Node, Operator, Statement};
use crate::eval::eval;
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

fn test_eval(inp: &str, expected: Object) {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().expect("No parse errors");

    let received = eval(program);

    assert_eq!(expected, received);
}
