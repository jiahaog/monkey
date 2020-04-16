use super::compile;
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
      bytecode::Instruction::new(bytecode::OP_CONSTANT, vec![0]).make(),
      bytecode::Instruction::new(bytecode::OP_CONSTANT, vec![1]).make(),
    ],
  )];

  for (input, expected_constants, expected_instructions) in tests {
    let program = parse(input).unwrap();

    let bytecode = compile(program).unwrap();

    test_bytes(expected_instructions, bytecode.bytes);

    test_constants(expected_constants, bytecode.constants);
  }
}

fn test_bytes(expected: Vec<bytecode::Bytes>, received: bytecode::Bytes) {
  let concatenated: bytecode::Bytes = expected.into_iter().sum();
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
