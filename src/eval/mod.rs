use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::{Object, NULL};

use self::ReturnState::*;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

// TODO cleanup the external api for this
// TODO Avoid cloning objects in Errors

type Result<'a> = std::result::Result<&'a Object, Error>;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    TypeMismatch {
        operator: Operator,
        left: Object,
        right: Object,
    },
    UnknownOperation {
        operator: Operator,
        right: Object,
    },
    IdentifierNotFound {
        name: String,
    },
}

impl Program {
    pub fn evaluate<'a, 'b>(&'a self, env: Env<'b>) -> Env<'b> {
        self.eval(env)
    }
}

trait Eval {
    fn eval<'a, 'b>(&'a self, env: Env<'b>) -> Env<'b>
    where
        'b: 'a;
}

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
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            return_state: Nothing,
        }
    }

    pub fn get_result(&self) -> Result {
        match &self.return_state {
            Nothing => Ok(&NULL),
            ReturningObject(key) | PlainObject(key) => Ok(self
                .store
                .get(key)
                .expect("Return state should always be a valid key to an object")),
            RuntimeError(err) => Err(err.clone()),
            LifetimeHack(_) => unimplemented!(),
        }
    }

    // TODO refactor private functions to module

    fn map<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match self.return_state {
            ReturningObject(_) | RuntimeError(_) => self,
            _ => f(self),
        }
    }

    fn map_return_obj<F: FnOnce(Object) -> std::result::Result<Object, Error>>(self, f: F) -> Self {
        self.map(|env| {
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
                            }
                        }
                        Err(err) => Self {
                            store: store,
                            return_state: RuntimeError(err),
                        },
                    }
                }
                _ => panic!("should be handled by map"),
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
            }
        })
    }

    // TODO rename this
    fn set_return_val(self, obj: Object) -> Self {
        self.map(|env| {
            let mut store = env.store;

            store.insert(EnvKey::Anonymous, obj);

            Self {
                store: store,
                return_state: PlainObject(EnvKey::Anonymous),
            }
        })
    }

    // Stores the anonymous return val as the named string
    fn bind_return_value_to_store(self, name: String) -> Self {
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
                        .store
                        .get(key)
                        .expect("Return state should be a key to a valid object")
                        .clone();

                    env.set_key_val(name, obj)
                }
            },
            _ => panic!("This should have been handled by map"),
        })
    }

    // Sets the object named as name as the return val
    fn set_return_val_from_name(self, name: String) -> Self {
        self.map(|env| {
            let key = EnvKey::Identifier(name);

            if env.store.contains_key(&key) {
                Self {
                    store: env.store,
                    return_state: PlainObject(key),
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
                }
            }
        })
    }

    // This is to signal that subsequent changes to the state should be skipped, as the evaluation
    // is in a "retuning" state
    fn set_return_val_short_circuit(self) -> Self {
        self.map(|env| Self {
            store: env.store,
            return_state: match env.return_state {
                PlainObject(key) => ReturningObject(key),
                x => x,
            },
        })
    }
}

impl Eval for Program {
    fn eval<'a, 'b>(&'a self, env: Env<'b>) -> Env<'b>
    where
        'b: 'a,
    {
        self.statements.eval(env)
    }
}

impl Eval for Statement {
    fn eval<'a, 'b>(&'a self, env: Env<'b>) -> Env<'b>
    where
        'b: 'a,
    {
        match self {
            Statement::Let(identifier_name, expr) => expr
                .eval(env)
                .bind_return_value_to_store(identifier_name.to_string()),
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => expr.eval(env).set_return_val_short_circuit(),
        }
    }
}

impl Eval for Statements {
    fn eval<'a, 'b>(&'a self, env: Env<'b>) -> Env<'b>
    where
        'b: 'a,
    {
        self.iter()
            // short circuit fold (kinda inefficient)
            .fold(env.set_return_val(NULL), |acc, statement| {
                acc.map(|prev_env| statement.eval(prev_env))
            })
    }
}

impl Eval for Expression {
    fn eval<'a, 'b>(&'a self, env: Env<'b>) -> Env<'b>
    where
        'b: 'a,
    {
        // TODO there are some unimplemented cases here
        match self {
            Expression::Identifier(name) => env.set_return_val_from_name(name.to_string()),
            // // TODO check if this is safe
            Expression::IntegerLiteral(val) => env.set_return_val(Object::Integer(*val as isize)),
            Expression::Boolean(val) => env.set_return_val(Object::from_bool_val(*val)),
            Expression::Prefix { operator, right } => right
                .eval(env)
                .map_return_obj(|result| eval_prefix_expr(*operator, result)),
            Expression::Infix {
                operator,
                left,
                right,
            } => {
                // TODO Not sure if there's a better way to do this. Perhaps we should be using
                // env.eval(Expression) or something instead?
                let left_env = left.eval(env);

                if let Ok(left_obj_original) = left_env.get_result().clone() {
                    let left_obj = left_obj_original.clone();

                    let right_env = right.eval(left_env);

                    right_env
                        .map_return_obj(|right_obj| eval_infix_expr(operator, left_obj, right_obj))
                } else {
                    left_env
                }
            }

            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(env, condition, consequence, alternative),
            x => unimplemented!("{:?}", x),
        }
    }
}

fn eval_prefix_expr(operator: Operator, right: Object) -> std::result::Result<Object, Error> {
    match (operator, right) {
        (Operator::Not, Object::Boolean(true)) => Ok(Object::from_bool_val(false)),
        (Operator::Not, Object::Boolean(false)) => Ok(Object::from_bool_val(true)),
        (Operator::Not, Object::Integer(_)) => Ok(Object::from_bool_val(false)),
        (Operator::Minus, Object::Integer(val)) => Ok(Object::Integer(-val)),
        (operator, right) => Err(Error::UnknownOperation {
            operator: operator,
            right: right,
        }),
    }
}

fn eval_infix_expr<'a>(
    operator: &Operator,
    left: Object,
    right: Object,
) -> std::result::Result<Object, Error> {
    match (operator, left, right) {
        (Operator::Plus, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::Integer(left_val + right_val))
        }
        (Operator::Minus, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::Integer(left_val - right_val))
        }
        (Operator::Multiply, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::Integer(left_val * right_val))
        }
        (Operator::Divide, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::Integer(left_val / right_val))
        }
        (Operator::LessThan, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::from_bool_val(left_val < right_val))
        }
        (Operator::GreaterThan, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::from_bool_val(left_val > right_val))
        }
        (Operator::Equal, left_val, right_val) => Ok(Object::from_bool_val(left_val == right_val)),
        (Operator::NotEqual, left_val, right_val) => {
            Ok(Object::from_bool_val(left_val != right_val))
        }
        (operator, left, right) => Err(Error::TypeMismatch {
            operator: *operator,
            left: left.clone(),
            right: right.clone(),
        }),
    }
}

fn eval_if_expr<'a, 'b>(
    env: Env<'a>,
    condition: &'b Box<Expression>,
    consequence: &'b Statements,
    alternative: &'b Statements,
) -> Env<'a> {
    condition
        .eval(env)
        .map(|new_env| match new_env.get_result() {
            Ok(object) => {
                if object.is_truthy() {
                    consequence.eval(new_env)
                } else {
                    alternative.eval(new_env)
                }
            }
            _ => new_env,
        })
}
