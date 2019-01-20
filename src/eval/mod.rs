mod env;
mod error;
mod object;
mod result;

#[cfg(test)]
mod tests;

use self::object::{Object, NULL};
use crate::ast::{CallFunctionExpression, Expression, Operator, Program, Statement, Statements};

pub use self::env::Env;
use self::error::Error;
use self::result::{EvalResult, ToEvalResult, ToResult};

impl Program {
    pub fn evaluate(&self, env: Env) -> Result<Object, Error> {
        self.eval(env).to_result()
    }
}

trait Eval {
    fn eval(&self, env: Env) -> EvalResult;
}

impl Eval for Program {
    fn eval(&self, env: Env) -> EvalResult {
        self.statements.eval(env)
    }
}

impl Eval for Statement {
    fn eval(&self, env: Env) -> EvalResult {
        match self {
            Statement::Let(name, expr) => expr.eval(env.clone()).map_left(|object| {
                env.set(name, object.clone());
                object
            }),
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => expr
                .eval(env)
                .left_and_then(|object| object.to_eval_result_return()),
        }
    }
}

impl Eval for Statements {
    fn eval(&self, env: Env) -> EvalResult {
        self.iter().fold(NULL.to_eval_result(), |acc, statement| {
            // Calling map will do nothing if the acc is already in a returning or error state.
            // There are possibly ways to make this exit immediately
            acc.left_and_then(|_| statement.eval(env.clone()))
        })
    }
}

impl Eval for Expression {
    fn eval(&self, env: Env) -> EvalResult {
        // println!("env {:#?}\nexpr {:#?}\n", env, self);

        match self {
            Expression::Identifier(name) => match env.get(name) {
                Some(object) => object.to_eval_result(),
                None => Error::IdentifierNotFound {
                    name: name.to_string(),
                }
                .to_eval_result(),
            },
            // // TODO check if this is safe
            Expression::IntegerLiteral(val) => Object::Integer(*val as isize).to_eval_result(),
            Expression::Boolean(val) => Object::from_bool_val(*val).to_eval_result(),
            Expression::Prefix { operator, right } => right
                .eval(env)
                .left_and_then(|object| eval_prefix_expr(*operator, object).to_eval_result()),
            Expression::Infix {
                operator,
                left,
                right,
            } => left.eval(env.clone()).left_and_then(|left_obj| {
                right.eval(env).left_and_then(|right_obj| {
                    eval_infix_expr(operator, left_obj, right_obj).to_eval_result()
                })
            }),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(env, condition, consequence, alternative),
            Expression::FunctionLiteral(ast_func) => {
                Object::Function(object::Function::from_ast_fn(env.clone(), ast_func.clone()))
                    .to_eval_result()
            }
            Expression::Call {
                function,
                arguments,
            } => {
                // Translate identifier or function literal to common function
                let func_result: std::result::Result<object::Function, Error> = match function {
                    CallFunctionExpression::Identifier(name) => env
                        .get(name)
                        .ok_or(Error::IdentifierNotFound {
                            name: name.to_string(),
                        })
                        .and_then(|object| object::Function::from_object(object)),
                    CallFunctionExpression::Literal(ast_func) => {
                        Ok(object::Function::from_ast_fn(env.clone(), ast_func.clone()))
                    }
                };

                match func_result {
                    Ok(func) => apply_func(env, func, arguments),
                    Err(err) => err.to_eval_result(),
                }
            }
        }
    }
}

fn eval_prefix_expr(operator: Operator, right: Object) -> Result<Object, Error> {
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

fn eval_infix_expr(operator: &Operator, left: Object, right: Object) -> Result<Object, Error> {
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
            left: left,
            right: right,
        }),
    }
}

fn eval_if_expr(
    env: Env,
    condition: &Box<Expression>,
    consequence: &Statements,
    alternative: &Statements,
) -> EvalResult {
    condition.eval(env.clone()).left_and_then(|object| {
        if object.is_truthy() {
            consequence.eval(env)
        } else {
            alternative.eval(env)
        }
    })
}

fn apply_func(
    env: Env,
    object::Function {
        params,
        body,
        env: func_env,
    }: object::Function,
    arguments: &Vec<Expression>,
) -> EvalResult {
    // check params
    if params.len() != arguments.len() {
        Error::CallExpressionWrongNumArgs {
            params: params,
            arguments: arguments.clone(),
        }
        .to_eval_result()
    } else {
        params
            .into_iter()
            .zip(arguments)
            // evaluate arguments in the current env
            .map(|(name, expr)| {
                expr.eval(env.clone())
                    .to_result()
                    .map(|object| (name, object))
            })
            .collect::<std::result::Result<Vec<(String, Object)>, Error>>()
            // bind argument results to a new env which extends the function env
            .map(|name_and_objects| {
                bind_objects_to_env(Env::new_extending(func_env), name_and_objects)
            })
            .and_then(|child_env| body.eval(child_env).to_result())
            .to_eval_result()
    }
}

fn bind_objects_to_env(env: Env, names_and_objects: Vec<(String, Object)>) -> Env {
    names_and_objects
        .into_iter()
        .fold(env, |env, (name, object)| {
            env.set(&name, object);
            env
        })
}
