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
      bytecode::Instruction::OpConstant(0),
      bytecode::Instruction::OpConstant(1),
    ],
  )];

  for (input, expected_constants, expected_instructions) in tests {
    let program = parse(input).unwrap();

    let bytecode = compile(program).unwrap();

    let expected_bytes: bytecode::Bytes = expected_instructions.into_iter().map(|x| x.into()).sum();

    assert_eq!(expected_bytes, bytecode.bytes);

    test_constants(expected_constants, bytecode.constants);
  }
}

fn test_constants(expected: Vec<Object>, received: Vec<Object>) {
  assert_eq!(expected, received);
}

fn parse(inp: &str) -> Result<Program, Vec<ParseError>> {
  let lexer = Lexer::new(inp);
  let parser = Parser::new(lexer);

  parser.parse()
}
