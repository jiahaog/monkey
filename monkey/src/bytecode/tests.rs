use super::*;

#[test]
fn test_make() {
  let tests = vec![(
    OP_CONSTANT,
    vec![65535],
    Bytes::new(vec![OP_CONSTANT, 255, 255]),
  )];

  for (opcode, operand, expected) in tests {
    let received = Instruction::new(opcode, operand).make();

    assert_eq!(expected, received);
  }
}

#[test]
fn test_disassemble_bytecode() {
  // TODO split this into fmt a Instruction, and reading bytes into a Instruction.
  let byte_vec = vec![
    Instruction::new(OP_CONSTANT, vec![1]).make(),
    Instruction::new(OP_CONSTANT, vec![2]).make(),
    Instruction::new(OP_CONSTANT, vec![65535]).make(),
  ];

  let expected = "0000 OpConstant 1
0003 OpConstant 2
0006 OpConstant 65535";

  let bytes: Bytes = byte_vec.into_iter().sum();

  assert_eq!(expected, format!("{}", bytes));
}

// #[test]
// fn test_read_operands() {
//   let tests = vec![(OP_CONSTANT, vec![65535], 2)];

//   for (op_code, operands, expected_bytes_read) in tests {
//     let bytecode = Instruction::new(op_code, operands).make();
//     todo!()
//   }
// }
