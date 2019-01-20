use super::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// TODO RC instead of clone for objects, and maybe for objects used for errors as well

type EnvRef = Rc<RefCell<_Env>>;

// Wrapper type to deal with RCs and RefCells
#[derive(Debug)]
pub struct Env(EnvRef);

impl Env {
    pub fn new() -> Self {
        Env(Rc::new(RefCell::new(_Env::new())))
    }

    pub(super) fn new_extending(parent: Self) -> Self {
        let new_env = _Env::new_extending(parent.clone());
        Env(Rc::new(RefCell::new(new_env)))
    }

    pub(super) fn get(&self, key: &String) -> Option<Object> {
        self.0.borrow().get(key)
    }

    pub(super) fn set(&self, key: &String, val: Object) {
        self.0.borrow_mut().set(key, val);
    }
}

impl Clone for Env {
    fn clone(&self) -> Self {
        Env(Rc::clone(&self.0))
    }
}

// This is so that we can nest a Env inside a object for functions
impl PartialEq for Env {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

// This internal type doesn't know anything about RCs and RefCells
#[derive(Debug)]
pub struct _Env {
    store: HashMap<String, Object>,
    parent: Option<Env>,
}

impl _Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            parent: None,
        }
    }

    pub(super) fn new_extending(parent: Env) -> Self {
        let mut result = Self::new();
        result.parent = Some(parent);
        result
    }

    pub(super) fn get(&self, key: &String) -> Option<Object> {
        match (self.store.get(key).cloned(), &self.parent) {
            (Some(x), _) => Some(x),
            (None, Some(parent)) => parent.get(key),
            (None, None) => None,
        }
    }

    pub(super) fn set(&mut self, key: &String, val: Object) {
        self.store.insert(key.to_string(), val);
    }
}
