use super::*;

#[test]
fn test_make() {
  let tests = vec![(OP_CONSTANT, vec![65535], vec![OP_CONSTANT, 255, 255])];

  for (opcode, operand, expected) in tests {
    let received = make(opcode, operand);

    assert_eq!(expected, received);
  }
}
