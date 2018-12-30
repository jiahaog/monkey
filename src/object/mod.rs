use crate::ast::{Expression, Statements};

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Null,
    Boolean(bool),
    Integer(isize),
    // Function {
    //     // To be specific, this is a vec of Expression::Identifier
    //     parameters: Vec<String>,
    //     body: Statements,
    // },
}

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
pub const NULL: Object = Object::Null;

impl Object {
    pub fn from_bool_val(val: bool) -> Self {
        match val {
            true => TRUE,
            false => FALSE,
        }
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(false) | Object::Null => false,
            _ => true,
        }
    }
}
