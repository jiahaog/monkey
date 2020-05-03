use super::*;
use crate::ast::Program;
use crate::bytecode::Instruction::*;
use crate::lexer::Lexer;
use crate::object::Object::*;
use crate::parser;

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        (
            "1 + 2",
            vec![Integer(1), Integer(2)],
            vec![OpConstant(0), OpConstant(1), OpAdd, OpPop],
        ),
        (
            "1 - 2",
            vec![Integer(1), Integer(2)],
            vec![OpConstant(0), OpConstant(1), OpSub, OpPop],
        ),
        (
            "1 * 2",
            vec![Integer(1), Integer(2)],
            vec![OpConstant(0), OpConstant(1), OpMul, OpPop],
        ),
        (
            "2 / 1",
            vec![Integer(2), Integer(1)],
            vec![OpConstant(0), OpConstant(1), OpDiv, OpPop],
        ),
        ("true", vec![], vec![OpTrue, OpPop]),
        ("false", vec![], vec![OpFalse, OpPop]),
        (
            "1 > 2",
            vec![Integer(1), Integer(2)],
            vec![OpConstant(0), OpConstant(1), OpGreaterThan, OpPop],
        ),
        (
            "1 < 2",
            vec![Integer(2), Integer(1)],
            vec![OpConstant(0), OpConstant(1), OpGreaterThan, OpPop],
        ),
        (
            "1 == 2",
            vec![Integer(1), Integer(2)],
            vec![OpConstant(0), OpConstant(1), OpEqual, OpPop],
        ),
        (
            "1 != 2",
            vec![Integer(1), Integer(2)],
            vec![OpConstant(0), OpConstant(1), OpNotEqual, OpPop],
        ),
        (
            "true == false",
            vec![],
            vec![OpTrue, OpFalse, OpEqual, OpPop],
        ),
        (
            "true != false",
            vec![],
            vec![OpTrue, OpFalse, OpNotEqual, OpPop],
        ),
        ("-1", vec![Integer(1)], vec![OpConstant(0), OpNeg, OpPop]),
        ("!true", vec![], vec![OpTrue, OpNot, OpPop]),
        (
            "if (true) { 10 }; 3333;",
            vec![Integer(10), Integer(3333)],
            vec![
                // 0000
                OpTrue,
                // 0001
                OpJumpNotTruthy(10),
                // 0004
                OpConstant(0),
                // 0007
                OpJump(11),
                // 0010
                OpNull,
                // 0011
                OpPop,
                // 0012
                OpConstant(1),
                // 0015
                OpPop,
            ],
        ),
        (
            "if (true) { 10 } else { 20 }; 3333;",
            vec![Integer(10), Integer(20), Integer(3333)],
            vec![
                // 0000
                OpTrue,
                // 0001
                OpJumpNotTruthy(10),
                // 0004
                OpConstant(0),
                // 0007
                OpJump(13),
                // 0010
                OpConstant(1),
                // 0013
                OpPop,
                // 0014
                OpConstant(2),
                // 0017
                OpPop,
            ],
        ),
    ];

    for (input, expected_constants, expected_instructions) in tests {
        let program = parse(input).unwrap();

        let bytecode = compile(program).unwrap();

        assert_eq!(expected_instructions, bytecode.instructions);

        test_constants(expected_constants, bytecode.constants);
    }
}

// TODO testing of type mismatch.

fn test_constants(expected: Vec<Object>, received: Vec<Object>) {
    assert_eq!(expected, received);
}

fn parse(inp: &str) -> std::result::Result<Program, parser::Errors> {
    let lexer = Lexer::new(inp);
    let parser = parser::Parser::new(lexer);

    parser.parse()
}
