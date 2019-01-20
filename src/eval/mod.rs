mod env;
mod error;
mod object;

#[cfg(test)]
mod tests;

use either::Either;

use self::object::{Object, NULL};
use crate::ast::{CallFunctionExpression, Expression, Operator, Program, Statement, Statements};

pub use self::env::Env;
use self::error::Error;

// TODO Avoid cloning objects in Errors
// TODO cleanup InternalResult -> std::result::Result conversions and vice versa

type Result = std::result::Result<Object, Error>;

#[derive(Debug, Clone)]
enum ShortCircuit {
    ReturningObject(Object),
    RuntimeError(Error),
}

type InternalResult = Either<Object, ShortCircuit>;

impl Program {
    pub fn evaluate(&self, env: &mut Env) -> Result {
        match self.eval(env) {
            Either::Left(object) => Ok(object),
            Either::Right(ShortCircuit::ReturningObject(object)) => Ok(object),
            Either::Right(ShortCircuit::RuntimeError(err)) => Err(err),
        }
    }
}

trait Eval {
    fn eval(&self, env: &mut Env) -> InternalResult;
}

impl Eval for Program {
    fn eval(&self, env: &mut Env) -> InternalResult {
        self.statements.eval(env)
    }
}

impl Eval for Statement {
    fn eval(&self, env: &mut Env) -> InternalResult {
        match self {
            Statement::Let(name, expr) => expr.eval(env).map_left(|object| {
                env.set(name, object.clone());
                object
            }),
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => expr
                .eval(env)
                .left_and_then(|object| Either::Right(ShortCircuit::ReturningObject(object))),
        }
    }
}

impl Eval for Statements {
    fn eval(&self, env: &mut Env) -> InternalResult {
        self.iter().fold(Either::Left(NULL), |acc, statement| {
            // Calling map will do nothing if the acc is already in a returning or error state.
            // There are possibly ways to make this exit immediately
            acc.left_and_then(|_| statement.eval(env))
        })
    }
}

impl Eval for Expression {
    fn eval(&self, env: &mut Env) -> InternalResult {
        // println!("env {:#?}\nexpr {:#?}\n", env, self);

        match self {
            Expression::Identifier(name) => match env.get(name) {
                Some(object) => Either::Left(object),
                None => Either::Right(ShortCircuit::RuntimeError(Error::IdentifierNotFound {
                    name: name.to_string(),
                })),
            },
            // // TODO check if this is safe
            Expression::IntegerLiteral(val) => Either::Left(Object::Integer(*val as isize)),
            Expression::Boolean(val) => Either::Left(Object::from_bool_val(*val)),
            Expression::Prefix { operator, right } => {
                right
                    .eval(env)
                    .left_and_then(|object| match eval_prefix_expr(*operator, object) {
                        Ok(object) => Either::Left(object),
                        Err(err) => Either::Right(ShortCircuit::RuntimeError(err)),
                    })
            }
            Expression::Infix {
                operator,
                left,
                right,
            } => left.eval(env).left_and_then(|left_obj| {
                right.eval(env).left_and_then(|right_obj| {
                    match eval_infix_expr(operator, left_obj, right_obj) {
                        Ok(object) => Either::Left(object),
                        Err(err) => Either::Right(ShortCircuit::RuntimeError(err)),
                    }
                })
            }),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(env, condition, consequence, alternative),
            Expression::FunctionLiteral(ast_func) => Either::Left(Object::Function(
                object::Function::from_ast_fn(env.clone(), ast_func.clone()),
            )),
            Expression::Call {
                function,
                arguments,
            } => {
                // 1. Convert the function to an object (and check if it exists)
                // 2. Create a new child env
                // 3. Evaluate each zip(parameter, argument) in the new child env
                // 4. get the result of body.eval(child_env) and put it in the parent env

                // Translate identifier or function literal to common function
                let func_result: std::result::Result<object::Function, Error> = match function {
                    CallFunctionExpression::Identifier(name) => match env.get(name) {
                        Some(object) => object::Function::from_object(object),
                        None => Err(Error::IdentifierNotFound {
                            name: name.to_string(),
                        }),
                    },
                    CallFunctionExpression::Literal(ast_func) => {
                        Ok(object::Function::from_ast_fn(env.clone(), ast_func.clone()))
                    }
                };

                match func_result {
                    Ok(func) => eval_multiple(env, func, arguments),
                    Err(err) => Either::Right(ShortCircuit::RuntimeError(err)),
                }
            }
        }
    }
}

// TODO clean this up
fn eval_multiple(
    env: &mut Env,
    func: object::Function,
    arguments: &Vec<Expression>,
) -> InternalResult {
    let object::Function {
        params,
        body,
        env: func_env,
    } = func;
    // check params
    if arguments.len() != params.len() {
        // TODO more information in error
        Either::Right(ShortCircuit::RuntimeError(
            Error::CallExpressionWrongNumArgs,
        ))
    } else {
        // function arguments should be evaluated with the current env
        match eval_multiple_args(env, arguments, params) {
            Ok(mut child_env) => {
                // function body should be evaluated with the function env
                child_env.set_parent_env(*func_env);

                body.eval(&mut child_env)
            }
            Err(err) => Either::Right(ShortCircuit::RuntimeError(err)),
        }
    }
}

// Creates a child env and evaluates arguments in the child env
fn eval_multiple_args(
    env: &Env,
    args: &Vec<Expression>,
    params: Vec<String>,
) -> std::result::Result<Env, Error> {
    let mut child_env = Env::new_extending(env.clone());

    let zipped = args.iter().zip(params);

    let eval_result = zipped.fold(Either::Left(NULL), |acc, (expr, param_name)| {
        acc.left_and_then(|_| {
            expr.eval(&mut child_env).map_left(|object| {
                child_env.set(&param_name, object.clone());
                object
            })
        })
    });

    match eval_result {
        Either::Right(ShortCircuit::RuntimeError(err)) => Err(err),
        _ => Ok(child_env),
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

fn eval_infix_expr(
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
            left: left,
            right: right,
        }),
    }
}

fn eval_if_expr(
    env: &mut Env,
    condition: &Box<Expression>,
    consequence: &Statements,
    alternative: &Statements,
) -> InternalResult {
    condition.eval(env).left_and_then(|object| {
        if object.is_truthy() {
            consequence.eval(env)
        } else {
            alternative.eval(env)
        }
    })
}
