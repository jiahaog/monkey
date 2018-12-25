#[derive(PartialEq, Debug, PartialOrd)]
pub enum Object {
    Null,
    Boolean(bool),
    Integer(isize),
    // Marker for the evaluator to stop evaluating subsequent statements. Should not be
    // recursive
    Return(Box<Object>),
}

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

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
