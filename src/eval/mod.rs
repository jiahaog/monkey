use self::EvalResult::*;
use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::{Object, NULL};

use std::collections::HashMap;

#[cfg(test)]
mod tests;

// TODO try to get rid of all the .clone()

type Result = std::result::Result<Object, Error>;

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
    pub fn evaluate(&self, env: Env) -> Result {
        match self.eval(env).get_return_val() {
            // This should be the only place we use Object::NULL as we use optionals internally
            // within this module to handle missing values
            None => Ok(NULL),
            Some(Return(object)) => Ok(object.clone()),
            Some(Raw(object)) => Ok(object.clone()),
            Some(RuntimeError(err)) => Err(err.clone()),
        }
    }
}

trait Eval {
    fn eval(&self, env: Env) -> (Env);
}

// Internal evaluation result for short circuit of return statements and errors
#[derive(Debug, Clone)]
enum EvalResult {
    Raw(Object),
    Return(Object),
    RuntimeError(Error),
}

// Environment for doing ast evaluations. Perhaps it might be better if we move this to another
// module
pub struct Env {
    store: HashMap<String, Object>,
    // TODO return val should be a enum with either a raw object, or a reference pointing to an
    // object within the store
    return_val: Option<EvalResult>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            store: HashMap::new(),
            return_val: None,
        }
    }

    // fn get(&self, name: &String) -> Option<&Object> {
    //     self.store.get(name)
    // }

    // fn set(self, name: String, val: Object) -> Self {
    //     let mut store = self.store;

    //     store.insert(name, val);
    //     Env {
    //         store: store,
    //         return_val: self.return_val,
    //     }
    // }

    fn map<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        f(self)
    }

    fn map_return_val<F: FnOnce(EvalResult) -> EvalResult>(self, f: F) -> Self {
        Env {
            store: self.store,
            return_val: self.return_val.map(f),
        }
    }

    fn set_return_val(self, val: Option<EvalResult>) -> Self {
        Env {
            store: self.store,
            return_val: val,
        }
    }

    fn get_return_val(&self) -> Option<&EvalResult> {
        match &self.return_val {
            None => None,
            Some(x) => Some(&x),
        }
    }

    // stores the return value into the store, with the name parameter
    fn bind_return_value(self, name: String) -> Self {
        match self.return_val {
            Some(RuntimeError(_)) => self,
            Some(Raw(result)) => {
                // TODO This code duplicates stuff from self.set
                let mut store = self.store;

                store.insert(name, result);
                Env {
                    store: store,
                    return_val: None,
                }
            }
            Some(Return(_)) => {
                panic!("Return not allowed here: This should have been disallowed by the parser")
            }
            None => self,
        }
    }

    // Sets the identifier with the name parameter as the return value
    fn return_named_identifier(mut self, name: String) -> Self {
        match self.store.remove(&name) {
            Some(val) => {
                // TODO fix owing two of the same object in the env
                self.store.insert(name, val.clone());
                Env {
                    store: self.store,
                    return_val: Some(Raw(val)),
                }
            }
            None => self.set_return_val(Some(RuntimeError(Error::IdentifierNotFound {
                name: name.to_string(),
            }))),
        }
    }
}

impl Eval for Program {
    fn eval(&self, env: Env) -> (Env) {
        self.statements
            .eval(env)
            .map_return_val(|result| match result {
                Return(x) => Raw(x),
                result => result,
            })
    }
}

impl Eval for Statement {
    fn eval(&self, env: Env) -> (Env) {
        match self {
            Statement::Let(identifier_name, expr) => expr
                .eval(env)
                .bind_return_value(identifier_name.to_string()),
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => expr.eval(env).map_return_val(|result| match result {
                Raw(x) => Return(x),
                x => x,
            }),
        }
    }
}

impl Eval for Statements {
    fn eval(&self, env: Env) -> (Env) {
        self.iter().fold(
            env.set_return_val(None),
            |prev_env, statement| match prev_env.get_return_val() {
                // short circuit fold (kinda inefficient)
                Some(Return(_)) | Some(RuntimeError(_)) => prev_env,
                _ => statement.eval(prev_env),
            },
        )
    }
}

impl Eval for Expression {
    fn eval(&self, env: Env) -> (Env) {
        // TODO there are some unimplemented cases here
        match self {
            Expression::Identifier(name) => env.return_named_identifier(name.to_string()),
            // TODO check if this is safe
            Expression::IntegerLiteral(val) => {
                env.set_return_val(Some(Raw(Object::Integer(*val as isize))))
            }
            Expression::Boolean(val) => env.set_return_val(Some(Raw(Object::from_bool_val(*val)))),
            Expression::Prefix { operator, right } => right
                .eval(env)
                .map_return_val(|result| eval_prefix_expr(*operator, result)),
            Expression::Infix {
                operator,
                left,
                right,
            } => left.eval(env).map(|left_env| {
                let left_return_val = left_env.get_return_val().cloned();

                let right_env = right.eval(left_env);

                match (left_return_val, right_env.get_return_val()) {
                    (Some(left_result), Some(_)) => right_env.map_return_val(|right_result| {
                        eval_infix_expr(*operator, &left_result, &right_result)
                    }),
                    _ => panic!("This should have been caught by the parser"),
                }
            }),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(env, condition, consequence, alternative),
            x => unimplemented!("{:?}", x),
        }
    }
}

fn eval_prefix_expr(operator: Operator, right: EvalResult) -> EvalResult {
    match (operator, right) {
        (_, RuntimeError(x)) => RuntimeError(x),
        (_, Return(x)) => Return(x),
        (Operator::Not, Raw(Object::Boolean(true))) => Raw(Object::from_bool_val(false)),
        (Operator::Not, Raw(Object::Boolean(false))) => Raw(Object::from_bool_val(true)),
        (Operator::Not, Raw(Object::Integer(_))) => Raw(Object::from_bool_val(false)),
        (Operator::Minus, Raw(Object::Integer(val))) => Raw(Object::Integer(-val)),
        (operator, Raw(right)) => RuntimeError(Error::UnknownOperation {
            operator: operator,
            right: right,
        }),
    }
}

fn eval_infix_expr(operator: Operator, left: &EvalResult, right: &EvalResult) -> EvalResult {
    match (operator, left, right) {
        (_, RuntimeError(x), _) => RuntimeError(x.clone()),
        (_, _, RuntimeError(x)) => RuntimeError(x.clone()),
        (_, Return(x), _) => Return(x.clone()),
        (_, _, Return(x)) => Return(x.clone()),
        (Operator::Plus, Raw(Object::Integer(left_val)), Raw(Object::Integer(right_val))) => {
            Raw(Object::Integer(left_val + right_val))
        }
        (Operator::Minus, Raw(Object::Integer(left_val)), Raw(Object::Integer(right_val))) => {
            Raw(Object::Integer(left_val - right_val))
        }
        (Operator::Multiply, Raw(Object::Integer(left_val)), Raw(Object::Integer(right_val))) => {
            Raw(Object::Integer(left_val * right_val))
        }
        (Operator::Divide, Raw(Object::Integer(left_val)), Raw(Object::Integer(right_val))) => {
            Raw(Object::Integer(left_val / right_val))
        }
        (Operator::LessThan, Raw(Object::Integer(left_val)), Raw(Object::Integer(right_val))) => {
            Raw(Object::from_bool_val(left_val < right_val))
        }
        (
            Operator::GreaterThan,
            Raw(Object::Integer(left_val)),
            Raw(Object::Integer(right_val)),
        ) => Raw(Object::from_bool_val(left_val > right_val)),
        (Operator::Equal, Raw(left_val), Raw(right_val)) => {
            Raw(Object::from_bool_val(left_val == right_val))
        }
        (Operator::NotEqual, Raw(left_val), Raw(right_val)) => {
            Raw(Object::from_bool_val(left_val != right_val))
        }
        (operator, Raw(left), Raw(right)) => RuntimeError(Error::TypeMismatch {
            operator: operator,
            left: left.clone(),
            right: right.clone(),
        }),
    }
}

fn eval_if_expr(
    env: Env,
    condition: &Box<Expression>,
    consequence: &Statements,
    alternative: &Statements,
) -> (Env) {
    condition
        .eval(env)
        .map(|new_env| match new_env.get_return_val() {
            Some(Raw(x)) => {
                if x.is_truthy() {
                    consequence.eval(new_env)
                } else {
                    alternative.eval(new_env)
                }
            }
            Some(RuntimeError(_)) => new_env,
            _ => panic!("Conditionals doing weird things should have been caught by the parser"),
        })
}
