#[derive(PartialEq, Debug, PartialOrd)]
pub enum Object {
    Null,
    Boolean(bool),
    Integer(isize),
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
}
