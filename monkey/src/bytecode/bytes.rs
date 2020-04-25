use super::*;
use std::{fmt, iter, ops};

#[derive(Debug, PartialEq, Clone)]
pub struct Bytes(pub Vec<u8>);

impl Bytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

impl ops::Add<Bytes> for Bytes {
    type Output = Bytes;
    fn add(self, mut other: Bytes) -> Bytes {
        let mut bytes = self.0;

        bytes.append(&mut other.0);

        Self(bytes)
    }
}

impl iter::Sum for Bytes {
    fn sum<I: Iterator<Item = Bytes>>(iter: I) -> Self {
        iter.fold(Bytes::empty(), |acc, x| acc + x)
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = self
            .clone()
            .into_iter()
            .collect::<Result<Vec<Instruction>, Error>>();

        let display: String = match result {
            Ok(instructions) => {
                instructions
                    .into_iter()
                    .fold(
                        ("".into(), 0),
                        |(
                            acc, // The string to concatenate.
                            i,   // The starting index of the current instruction.
                        ),
                         instruction| {
                            (
                                format!("{}{:04} {}\n", acc, i, instruction),
                                i + Definition::from(&instruction).size,
                            )
                        },
                    )
                    .0
            }
            Err(error) => format!("{:?}", error),
        };
        write!(f, "{}", display.trim())
    }
}

impl From<Instruction> for Bytes {
    fn from(instruction: Instruction) -> Self {
        use Instruction::*;

        let definition: Definition = (&instruction).into();

        let mut bytes = vec![definition.code];

        match instruction {
            OpConstant(pointer) => {
                let byte_slice = pointer.to_be_bytes();
                bytes.extend_from_slice(&byte_slice);

                Bytes::new(bytes)
            }
            OpAdd => Bytes::new(bytes),
        }
    }
}

impl IntoIterator for Bytes {
    type Item = Result<Instruction, Error>;
    type IntoIter = BytesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        BytesIntoIter(self.0.into_iter())
    }
}

pub struct BytesIntoIter(std::vec::IntoIter<u8>);

impl Iterator for BytesIntoIter {
    type Item = Result<Instruction, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let iter_mut = &mut self.0;
        let mut byte_iter = iter_mut.map(|byte| Byte(byte));

        byte_iter.next().map(|opcode| match opcode {
            Byte(OP_CONSTANT) => byte_iter
                .collect::<Result<u16, Error>>()
                .map(|pointer| OpConstant(pointer)),
            Byte(OP_ADD) => Ok(OpAdd),
            _ => Err(Error {}),
        })
    }
}

/// Wrapper for a byte.
///
/// We cannot `collect()` a `u8` into a `Result<u16, Error>`, because of E0117.
/// So wrap it for educational purposes.
///
/// The collect pattern also allows us to decouple the opcode from the expected
/// operand size – when this is called we only know about the expected result,
/// So we try to collect the iterator of bytes into the expected result, e.g.
/// `u16` or fail trying.
#[derive(Debug)]
struct Byte(u8);

impl iter::FromIterator<Byte> for Result<u16, Error> {
    fn from_iter<I: IntoIterator<Item = Byte>>(iter: I) -> Self {
        let bytes: Vec<u8> = iter.into_iter().take(2).map(|byte| byte.0).collect();

        if bytes.len() != 2 {
            return Err(Error {});
        }

        let mut array = [0; 2];
        array.copy_from_slice(bytes.as_slice());

        Ok(u16::from_be_bytes(array))
    }
}
