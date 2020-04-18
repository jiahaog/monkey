use super::Instruction::*;
use super::*;

#[test]
fn test_instruction_to_bytes() {
    let tests = vec![
        (OpConstant(65535), Bytes::new(vec![OP_CONSTANT, 255, 255])),
        // Test big-endian.
        (OpConstant(65534), Bytes::new(vec![OP_CONSTANT, 255, 254])),
        (OpAdd, Bytes::new(vec![OP_ADD])),
    ];

    for (instruction, expected) in tests {
        let bytes = instruction.into();

        assert_eq!(expected, bytes);
    }
}

#[test]
fn test_fmt_instruction() {
    let tests = vec![
        (OpConstant(1), "OpConstant 1"),
        (OpConstant(2), "OpConstant 2"),
        (OpConstant(65535), "OpConstant 65535"),
        (OpAdd, "OpAdd"),
    ];

    for (instruction, expected) in tests {
        assert_eq!(expected, format!("{}", instruction));
    }
}

#[test]
fn test_bytes_to_instruction() {
    let tests = vec![
        (
            Bytes::new(vec![OP_CONSTANT, 255, 255]),
            vec![OpConstant(65535)],
        ),
        (Bytes::new(vec![OP_ADD]), vec![OpAdd]),
    ];

    for (bytes, expected_instructions) in tests {
        let results: Result<Vec<Instruction>, Error> = bytes.into_iter().collect();
        assert_eq!(expected_instructions, results.unwrap());
    }
}

#[test]
fn test_bytes_disassembled_display() {
    let tests = vec![(
        vec![
            Bytes::new(vec![OP_CONSTANT, 0, 1]),
            Bytes::new(vec![OP_CONSTANT, 0, 2]),
            Bytes::new(vec![OP_CONSTANT, 255, 255]),
            Bytes::new(vec![OP_ADD]),
        ],
        "0000 OpConstant 1
0003 OpConstant 2
0006 OpConstant 65535
0009 OpAdd",
    )];

    for (bytes, expected) in tests {
        let all_bytes: Bytes = bytes.into_iter().sum();
        assert_eq!(expected, format!("{}", all_bytes))
    }
}
