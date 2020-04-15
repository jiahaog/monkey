use super::*;
use crate::ast::Program;
use crate::bytecode;
use crate::lexer::Lexer;
use crate::object::Object;
use crate::parser::{ParseError, Parser};

#[test]
fn test_integer_arithmetic() {
  let tests = vec![(
    "1 + 2",
    vec![Object::Integer(1), Object::Integer(2)],
    vec![
      bytecode::make(bytecode::OP_CONSTANT, vec![0]),
      bytecode::make(bytecode::OP_CONSTANT, vec![1]),
    ],
  )];

  for (input, expected_constants, expected_instructions) in tests {
    // let received = make(opcode, operand.as_slice());

    let program = parse(input).unwrap();

    let bytecode = compile(program).unwrap();
    // let compiler = Compiler::new();
    // compiler.compile(program);

    test_instructions(expected_instructions, bytecode.instructions);

    test_constants(expected_constants, bytecode.constants);
  }
}

fn test_instructions(expected: Vec<bytecode::Instructions>, received: bytecode::Instructions) {
  let concatenated: bytecode::Instructions = expected.into_iter().flatten().collect();
  assert_eq!(concatenated, received);
}

fn test_constants(expected: Vec<Object>, received: Vec<Object>) {
  assert_eq!(expected, received);
}

fn parse(inp: &str) -> Result<Program, Vec<ParseError>> {
  let lexer = Lexer::new(inp);
  let parser = Parser::new(lexer);

  parser.parse()
}
