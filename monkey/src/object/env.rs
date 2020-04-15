use super::{BuiltIn, Object};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

type EnvRef = Rc<RefCell<_Env>>;

// Wrapper type to deal with RCs and RefCells, so that Env is cheap to clone
#[derive(Debug)]
pub struct Env(EnvRef);

fn register_built_ins(env: Env) -> Env {
    env.set("len".to_string(), Object::BuiltIn(BuiltIn::Len));
    env.set("push".to_string(), Object::BuiltIn(BuiltIn::Push));
    env.set("rest".to_string(), Object::BuiltIn(BuiltIn::Rest));
    env.set("print".to_string(), Object::BuiltIn(BuiltIn::Print));
    // Not all built-ins are here such as `Index` because it can be called using `[$index]`.
    // This allows us to reuse the apply logic of the built-ins for operators.
    env
}

impl Env {
    pub fn new() -> Self {
        let env = Env(Rc::new(RefCell::new(_Env::new())));
        register_built_ins(env)
    }

    pub(crate) fn new_extending(parent: Self) -> Self {
        let new_env = _Env::new_extending(parent.clone());
        Env(Rc::new(RefCell::new(new_env)))
    }

    pub(crate) fn get(&self, key: &String) -> Option<Object> {
        self.0.borrow().get(key)
    }

    pub(crate) fn set(&self, key: String, val: Object) {
        self.0.borrow_mut().set(key, val);
    }

    pub(crate) fn write_stdout(&self, msg: String) {
        self.0.borrow_mut().write_stdout(msg);
    }

    pub fn pop_stdout(&self) -> Vec<String> {
        self.0.borrow_mut().pop_stdout()
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
struct _Env {
    store: HashMap<String, Object>,
    parent: Option<Env>,
    stdout: Vec<String>,
}

impl _Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            parent: None,
            stdout: Vec::new(),
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

    pub(super) fn set(&mut self, key: String, val: Object) {
        self.store.insert(key, val);
    }

    // TODO: More performant way to always write and read the stdout instead of going up the tree.

    pub(super) fn write_stdout(&mut self, msg: String) {
        match &self.parent {
            None => self.stdout.push(msg),
            Some(parent) => parent.write_stdout(msg),
        }
    }

    pub(super) fn pop_stdout(&mut self) -> Vec<String> {
        match &self.parent {
            None => mem::replace(&mut self.stdout, Vec::new()),
            Some(parent) => parent.pop_stdout(),
        }
    }
}
