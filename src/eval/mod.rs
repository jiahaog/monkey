use self::EvalResult::*;
use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::{Object, NULL};

use std::collections::HashMap;

#[cfg(test)]
mod tests;

// TODO cleanup the external api for this
// TODO try to get rid of all the .clone()

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

#[derive(Debug, Clone)]
enum EvalObject {
    Anonymous,
    Reference(String),
}

// Internal evaluation result for short circuit of return statements and errors
#[derive(Debug, Clone)]
enum EvalResult<'a> {
    Raw(Object),
    Return(Object),
    RuntimeError(Error),
    // TODO this can probably be deleted
    ReferenceResult(&'a Object),
}

// Environment for doing ast evaluations. Perhaps it might be better if we move this to another
// module
pub struct Env<'a> {
    store: HashMap<String, Object>,
    // TODO return val should be a enum with either a raw object, or a reference pointing to an
    // object within the store
    return_val: EvalResult<'a>,
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            return_val: Raw(NULL),
        }
    }

    pub fn get_result(&self) -> Result {
        match &self.return_val {
            // This should be the only place we use Object::NULL as we use optionals internally
            // within this module to handle missing values
            Return(object) => Ok(&object),
            Raw(object) => Ok(&object),
            RuntimeError(err) => Err(err.clone()),
            // TODO references
            ReferenceResult(_r) => unimplemented!(),
        }
    }

    fn map<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        match self.return_val {
            Raw(_) => f(self),
            // Return, RuntimeError
            x => Env {
                store: self.store,
                return_val: x,
            },
        }
    }

    fn set_return_val_state_true(self) -> Self {
        self.map(|env| Env {
            store: env.store,
            return_val: match env.return_val {
                Raw(x) => Return(x),
                x => x,
            },
        })
    }

    fn map_return_obj<F: FnOnce(Object) -> std::result::Result<Object, Error>>(self, f: F) -> Self {
        self.map(|env| Env {
            store: env.store,
            return_val: match env.return_val {
                Raw(object) => match f(object) {
                    Ok(x) => Raw(x),
                    Err(x) => RuntimeError(x),
                },
                _ => panic!("This should have been handled by env.map"),
            },
        })
    }

    fn set_return_val(self, val: std::result::Result<Object, Error>) -> Self {
        Env {
            store: self.store,
            return_val: match val {
                Ok(object) => Raw(object),
                Err(x) => RuntimeError(x),
            },
        }
    }

    // stores the return value into the store, with the name parameter
    fn bind_return_value(self, name: String) -> Self {
        self.map(|env| match env.return_val {
            Raw(result) => {
                // TODO This code duplicates stuff from self.set
                let mut store = env.store;

                store.insert(name, result);
                Env {
                    store: store,
                    return_val: Raw(NULL),
                }
            }
            _ => panic!("This should have been handled by env.map"),
        })
    }

    // Sets the identifier with the name parameter as the return value
    fn return_named_identifier(mut self, name: String) -> Self {
        match self.store.remove(&name) {
            Some(val) => {
                // TODO fix owing two of the same object in the env
                self.store.insert(name, val.clone());
                Env {
                    store: self.store,
                    return_val: Raw(val),
                }
            }
            None => self.set_return_val(Err(Error::IdentifierNotFound {
                name: name.to_string(),
            })),
        }
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
                .bind_return_value(identifier_name.to_string()),
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => expr.eval(env).set_return_val_state_true(),
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
            .fold(env.set_return_val(Ok(NULL)), |acc, statement| {
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
            Expression::Identifier(name) => env.return_named_identifier(name.to_string()),
            // // TODO check if this is safe
            Expression::IntegerLiteral(val) => {
                env.set_return_val(Ok(Object::Integer(*val as isize)))
            }
            Expression::Boolean(val) => env.set_return_val(Ok(Object::from_bool_val(*val))),
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
