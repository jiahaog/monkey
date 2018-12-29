use std::collections::HashMap;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Object {
    Null,
    Boolean(bool),
    Integer(isize),
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

// Environment for doing ast evaluations. Perhaps it might be better if we move this to another
// module
pub struct Env {
    store: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> Option<&Object> {
        // We will have more problems if our Object struct doesn't implement copy
        self.store.get(name)
    }

    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}
