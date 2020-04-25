use super::Instruction::*;

use super::opcode::*;
use super::*;

macro_rules! bytes {
    ($( $x:expr ),* ) => {{
        let temp_vec = vec![
            $($x  ,)*
        ];

        Bytes::new(temp_vec)
    }};
}

#[test]
fn test_instruction_to_bytes() {
    let tests = vec![
        (OpConstant(65535), bytes![OP_CONSTANT, 255, 255]),
        // Test big-endian.
        (OpConstant(65534), bytes![OP_CONSTANT, 255, 254]),
        (OpAdd, bytes![OP_ADD]),
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
        (bytes![OP_CONSTANT, 255, 255], vec![OpConstant(65535)]),
        (bytes![OP_ADD], vec![OpAdd]),
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
            bytes![OP_CONSTANT, 0, 1],
            bytes![OP_CONSTANT, 0, 2],
            bytes![OP_CONSTANT, 255, 255],
            bytes![OP_ADD],
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
