use super::object::Object;
use std::collections::HashMap;

// TODO RC instead of clone

#[derive(Debug, Clone)]
pub struct Env {
    store: HashMap<String, Object>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            parent: None,
        }
    }

    pub(super) fn get(&self, key: &String) -> Option<Object> {
        match (self.store.get(key).cloned(), &self.parent) {
            (Some(x), _) => Some(x),
            (None, Some(parent)) => parent.get(key),
            (None, None) => None,
        }
    }

    pub(super) fn new_extending(parent: Self) -> Self {
        let mut result = Self::new();
        result.parent = Some(Box::new(parent));
        result
    }

    pub(super) fn set(&mut self, key: &String, val: Object) {
        self.store.insert(key.to_string(), val);
    }

    pub(super) fn set_parent_env(&mut self, parent: Self) {
        self.parent = Some(Box::new(parent));
    }
}

// This is so that we can nest a Env inside a object for functions
impl PartialEq for Env {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
