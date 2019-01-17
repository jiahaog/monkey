use self::ReturnState::*;
use super::error::Error;
use super::object::{Object, NULL};
use super::Result;
use std::collections::HashMap;

// TODO RC instead of clone

// This is so that we can nest a Env inside a object for functions
impl PartialEq for Env {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
enum ReturnState {
    Nothing,
    PlainObject(Object),
    ReturningObject(Object),
    RuntimeError(Error),
}

// Key design notes: a state variable is used to key the "last", or final object in the current
// evaluation for the environment to the store. This is done to avoid duplicating this object both
// inside and outside of the store, and also because I don't think it's possible to store a
// reference to a another struct field within the same struct in safe rust.
//
// This leaves us with having to maintain the invariant that return_state should always be a valid
// indicator of objects in the store, hence the use of panics in the code here.
//
// Rules:
// - Types used by this object should not be exposed to consumers even in the same module
// - Methods should preserve immutability
#[derive(Debug, Clone)]
pub struct Env {
    store: HashMap<String, Object>,
    return_state: ReturnState,
    parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            return_state: Nothing,
            parent: None,
        }
    }

    pub fn get_result(&self) -> Result {
        match &self.return_state {
            Nothing => Ok(NULL),
            ReturningObject(object) | PlainObject(object) => Ok(object.clone()),
            RuntimeError(err) => Err(err.clone()),
        }
    }

    pub(super) fn new_extending(parent: Env) -> Env {
        let mut result = Self::new();
        result.parent = Some(Box::new(parent));
        result
    }

    pub(super) fn with_parent(self, parent: Env) -> Env {
        Self {
            store: self.store,
            return_state: self.return_state,
            parent: Some(Box::new(parent)),
        }
    }

    pub fn clear_return_state(self) -> Self {
        Self {
            store: self.store,
            return_state: Nothing,
            parent: self.parent,
        }
    }

    pub(super) fn get_result_owned(self) -> std::result::Result<Object, Error> {
        match self.return_state {
            Nothing => Ok(NULL),
            ReturningObject(obj) | PlainObject(obj) => Ok(obj),
            RuntimeError(err) => Err(err),
        }
    }

    pub(super) fn map<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match self.return_state {
            ReturningObject(_) | RuntimeError(_) => self,
            _ => f(self),
        }
    }

    // TODO rename this, possibly look for reusing things
    pub(super) fn map_separated<F: FnOnce(Self, Object) -> Self>(self, f: F) -> Self {
        self.map(|env| match &env.return_state {
            PlainObject(obj) => f(
                Self {
                    store: env.store,
                    return_state: Nothing,
                    parent: env.parent,
                },
                obj.clone(),
            ),
            _ => env,
        })
    }

    pub(super) fn map_return_obj<F: FnOnce(Object) -> std::result::Result<Object, Error>>(
        self,
        f: F,
    ) -> Self {
        self.map(|env| Self {
            store: env.store,
            return_state: match env.return_state {
                PlainObject(object) => match f(object) {
                    Ok(new_obj) => PlainObject(new_obj),
                    Err(err) => RuntimeError(err),
                },
                x => x,
            },
            parent: env.parent,
        })
    }

    // Stores the anonymous return val as the named string
    pub(super) fn bind_return_value_to_store(self, name: String) -> Self {
        self.map(|mut env| Self {
            store: match env.return_state {
                PlainObject(obj) => {
                    env.store.insert(name, obj);
                    env.store
                }
                _ => env.store,
            },
            return_state: Nothing,
            parent: env.parent,
        })
    }

    // Sets the object named as name as the return val
    pub(super) fn set_return_val_from_name(self, name: String) -> Self {
        self.map(|env| match env.store_get(&name) {
            Some(obj) => Self {
                store: env.store,
                return_state: PlainObject(obj),
                parent: env.parent,
            },

            None => Self {
                store: env.store,
                return_state: RuntimeError(Error::IdentifierNotFound { name: name }),
                parent: env.parent,
            },
        })
    }

    // This is to signal that subsequent changes to the state should be skipped, as the evaluation
    // is in a "retuning" state
    pub(super) fn set_return_val_short_circuit(self) -> Self {
        self.map(|env| Self {
            store: env.store,
            return_state: match env.return_state {
                PlainObject(key) => ReturningObject(key),
                x => x,
            },
            parent: env.parent,
        })
    }

    pub(super) fn set_return_val(self, obj: Object) -> Self {
        self.map(|env| Self {
            store: env.store,
            return_state: PlainObject(obj),
            parent: env.parent,
        })
    }

    pub(super) fn set_return_result(self, result: std::result::Result<Object, Error>) -> Self {
        self.map(|env| Self {
            store: env.store,
            return_state: match result {
                Ok(obj) => PlainObject(obj),
                Err(err) => RuntimeError(err),
            },
            parent: env.parent,
        })
    }

    fn map_store<F: FnOnce(HashMap<String, Object>, Object) -> HashMap<String, Object>>(
        self,
        f: F,
    ) -> Self {
        self.map(|env| Self {
            store: match &env.return_state {
                PlainObject(obj) => f(env.store, obj.clone()),
                _ => env.store,
            },
            return_state: env.return_state,
            parent: env.parent,
        })
    }

    fn store_get(&self, key: &String) -> Option<Object> {
        match (self.store.get(key).cloned(), &self.parent) {
            (Some(x), _) => Some(x),
            (None, Some(parent)) => parent.store_get(key),
            (None, None) => None,
        }
    }
}
