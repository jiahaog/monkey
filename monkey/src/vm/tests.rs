use super::*;
use crate::compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        ("1", Object::Integer(1)),
        ("2", Object::Integer(2)),
        ("1 + 2", Object::Integer(3)),
    ];

    for (inp, expected) in tests {
        let compiler::Output {
            instructions,
            constants,
        } = compile(inp);

        let vm = Vm::new(instructions, constants);
        let object = vm.run().unwrap();

        assert_eq!(object, expected);
    }
}

fn compile(inp: &str) -> compiler::Output {
    let lexer = Lexer::new(inp);
    let parser = Parser::new(lexer);

    let program = parser.parse().unwrap();

    compiler::compile(program).unwrap()
}
