use super::Instruction::*;
use super::*;

#[test]
fn test_instruction_to_bytes() {
  let tests = vec![
    (OpConstant(65535), Bytes::new(vec![OP_CONSTANT, 255, 255])),
    // Test big-endian.
    (OpConstant(65534), Bytes::new(vec![OP_CONSTANT, 255, 254])),
  ];

  for (instruction, expected) in tests {
    let bytes = instruction.into();

    assert_eq!(expected, bytes);
  }
}

// TODO
#[ignore]
#[test]
fn test_fmt_instruction() {
  let tests = vec![
    (OpConstant(1), "OpConstant 1"),
    (OpConstant(2), "OpConstant 2"),
    (OpConstant(65535), "OpConstant 65535"),
  ];

  for (instruction, expected) in tests {
    assert_eq!(expected, format!("{}", instruction));
  }
}

#[test]
fn test_bytes_to_instruction() {}

#[test]
fn test_bytes_disassembled_fmt() {
  //   let expected = "0000 OpConstant 1
  // 0003 OpConstant 2
  // 0006 OpConstant 65535";
}
