use self::ReturnState::*;
use super::error::Error;
use super::Result;
use crate::object::{Object, NULL};
use std::collections::HashMap;

// TODO reduce number of panics

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum EnvKey {
    Identifier(String),
    Anonymous,
}

#[derive(Debug)]
enum ReturnState<'a> {
    Nothing,
    PlainObject(EnvKey),
    ReturningObject(EnvKey),
    RuntimeError(Error),
    // TODO remove this once we are sure we don't need liftimes in Env
    LifetimeHack(&'a str),
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
#[derive(Debug)]
pub struct Env<'a> {
    store: HashMap<EnvKey, Object>,
    return_state: ReturnState<'a>,
    parent: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            return_state: Nothing,
            parent: None,
        }
    }

    pub(super) fn new_extending<'b>(parent: &'b Env<'b>) -> Env<'a>
    where
        'b: 'a,
    {
        let mut result = Self::new();
        result.parent = Some(parent);
        result
    }

    // TODO fix this
    pub fn get_return_hack(mut self) -> std::result::Result<Object, Error> {
        match self.return_state {
            Nothing => Ok(NULL),
            ReturningObject(key) | PlainObject(key) => Ok(self
                .store
                .remove(&key)
                .expect("Return state should always be a valid key to an object")),
            RuntimeError(err) => panic!("{:?} err", err),
            LifetimeHack(_) => unimplemented!(),
        }
    }

    pub fn get_result(&self) -> Result {
        match &self.return_state {
            Nothing => Ok(&NULL),
            ReturningObject(key) | PlainObject(key) => Ok(self
                .store_get(key)
                .expect("Return state should always be a valid key to an object")),
            RuntimeError(err) => Err(err),
            LifetimeHack(_) => unimplemented!(),
        }
    }

    fn store_get(&self, key: &EnvKey) -> Option<&Object> {
        match (self.store.get(key), self.parent) {
            (Some(x), _) => Some(x),
            (None, Some(parent)) => parent.store_get(key),
            (None, None) => None,
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
        self.map(|env| match env.return_state {
            PlainObject(key) => match env.store.get(&key).cloned() {
                Some(obj) => f(
                    Self {
                        store: env.store,
                        return_state: Nothing,
                        parent: env.parent,
                    },
                    obj,
                ),
                _ => panic!("return state should always be valid"),
            },
            _ => env,
        })
    }

    pub(super) fn map_return_obj<F: FnOnce(Object) -> std::result::Result<Object, Error>>(
        self,
        f: F,
    ) -> Self {
        self.map(|env| {
            println!("aaaa {:?}", env);
            let mut store = env.store;

            match env.return_state {
                PlainObject(key) => {
                    let obj = store.remove(&key).expect("State should be valid key");
                    match f(obj) {
                        Ok(new_obj) => {
                            // Not sure if we can avoid cloning here, need to think this through
                            store.insert(key.clone(), new_obj);

                            Self {
                                store: store,
                                return_state: PlainObject(key),
                                parent: env.parent,
                            }
                        }
                        Err(err) => Self {
                            store: store,
                            return_state: RuntimeError(err),
                            parent: env.parent,
                        },
                    }
                }
                _ => panic!("should be handled by map"),
            }
        })
    }

    // Stores the anonymous return val as the named string
    pub(super) fn bind_return_value_to_store(self, name: String) -> Self {
        self.map(|mut env| match &env.return_state {
            PlainObject(key) => match key {
                EnvKey::Anonymous => {
                    let obj = env.store.remove(&EnvKey::Anonymous);

                    env.set_key_val(
                        name,
                        obj.expect("Return state should always be a key to a valid object"),
                    )
                }
                EnvKey::Identifier(_) => {
                    // TODO Fix this, this duplicates the object instead of using a reference to
                    // the original identifier
                    // '''
                    // let a = 5;
                    // let b = a;
                    // b
                    // '''
                    // This should be fixable by storing our objects in the hashmap using RC
                    let obj = env
                        .store_get(key)
                        .expect("Return state should be a key to a valid object")
                        .clone();

                    env.set_key_val(name, obj)
                }
            },
            _ => panic!("This should have been handled by map"),
        })
    }

    fn store_contains_key(&self, key: &EnvKey) -> bool {
        match (self.store.contains_key(key), self.parent) {
            (true, _) => true,
            (false, Some(parent)) => parent.store_contains_key(key),
            (false, None) => false,
        }
    }

    // Sets the object named as name as the return val
    pub(super) fn set_return_val_from_name(self, name: String) -> Self {
        self.map(|env| {
            let key = EnvKey::Identifier(name);

            if env.store_contains_key(&key) {
                Self {
                    store: env.store,
                    return_state: PlainObject(key),
                    parent: env.parent,
                }
            } else {
                Self {
                    store: env.store,
                    return_state: RuntimeError(Error::IdentifierNotFound {
                        name: match key {
                            EnvKey::Identifier(name) => name,
                            _ => panic!("Expected a identifier key type"),
                        },
                    }),
                    parent: env.parent,
                }
            }
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
        self.map(|env| {
            let mut store = env.store;

            store.insert(EnvKey::Anonymous, obj);

            Self {
                store: store,
                return_state: PlainObject(EnvKey::Anonymous),
                parent: env.parent,
            }
        })
    }

    fn set_key_val(self, name: String, obj: Object) -> Self {
        self.map(|env| {
            let mut store = env.store;

            store.insert(EnvKey::Identifier(name), obj);

            Self {
                store: store,
                return_state: env.return_state,
                parent: env.parent,
            }
        })
    }
}
