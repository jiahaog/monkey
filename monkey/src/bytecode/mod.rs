use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub type Instructions = Vec<u8>;

type OpCode = u8;

const OP_CONSTANT: OpCode = 1;

#[derive(Clone)]
struct Definition {
  name: &'static str,
  // TODO: Can we remove this?
  // length is the number of operands, value is the number of bytes for the operand.
  operand_widths: Vec<isize>,
}

fn lookup(opcode: OpCode) -> Definition {
  let mut definitions = HashMap::new();
  definitions.insert(
    OP_CONSTANT,
    Definition {
      name: "OpConstant",
      // 16 bits
      operand_widths: vec![2],
    },
  );

  definitions[&opcode].clone()
}

// TODO change to signed integers
fn make(opcode: OpCode, operands: &[u16]) -> Vec<u8> {
  let definition = lookup(opcode);

  let instruction_len: isize = definition.operand_widths.iter().sum();

  let mut instruction = Vec::new();
  instruction.push(opcode);

  for (i, operand) in operands.iter().enumerate() {
    // We don't need to check operand_widths because of type safety.

    let bytes = operand.to_be_bytes();
    instruction.extend_from_slice(&bytes);
  }

  instruction
}
