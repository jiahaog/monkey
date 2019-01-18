mod env;
mod error;
mod object;

#[cfg(test)]
mod tests;

use self::object::{Object, NULL};
use crate::ast::{
    CallFunctionExpression, Expression, Function, Operator, Program, Statement, Statements,
};

pub use self::env::Env;
use self::error::Error;

// TODO Avoid cloning objects in Errors

type Result<'a> = std::result::Result<Object, Error>;

impl Program {
    pub fn evaluate(&self, env: Env) -> Env {
        self.eval(env)
    }
}

trait Eval {
    fn eval(&self, env: Env) -> Env;
}

impl Eval for Program {
    fn eval(&self, env: Env) -> Env {
        self.statements.eval(env)
    }
}

impl Eval for Statement {
    fn eval(&self, env: Env) -> Env {
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
    fn eval(&self, env: Env) -> Env {
        self.iter()
            .fold(env.set_return_val(NULL), |acc, statement| {
                // Calling map will do nothing if the acc is already in a returning or error state.
                // There are possibly ways to make this exit immediately
                acc.map(|prev_env| statement.eval(prev_env))
            })
    }
}

impl Eval for Expression {
    fn eval(&self, env: Env) -> Env {
        // println!("env {:#?}\nexpr {:#?}\n", env, self);

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
            } => left.eval(env).map(|left_env| {
                let left_obj = left_env.get_result().expect("no errors after map").clone();

                right
                    .eval(left_env)
                    .map_return_obj(|right_obj| eval_infix_expr(operator, left_obj, right_obj))
            }),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(env, condition, consequence, alternative),
            Expression::FunctionLiteral(Function { params, body }) => {
                let func_env = env.clone();

                env.set_return_val(Object::Function {
                    params: params.clone(),
                    body: body.clone(),
                    env: Box::new(func_env),
                })
            }
            Expression::Call {
                function,
                arguments,
            } => {
                // 1. Convert the function to an object (and check if it exists)
                // 2. Create a new child env
                // 3. Evaluate each zip(parameter, argument) in the new child env
                // 4. get the result of body.eval(child_env) and put it in the parent env

                // Translate identifier or function literal to common function
                match function {
                    CallFunctionExpression::Identifier(name) => env
                        .set_return_val_from_name(name.to_string())
                        // check if idenntifier points to a function
                        .map_return_obj(|obj| match obj {
                            Object::Function {
                                params: _,
                                body: _,
                                env: _,
                            } => Ok(obj),
                            unexpected_obj => Err(Error::CallExpressionExpectedFunction {
                                received: unexpected_obj.clone(),
                            }),
                        }),

                    CallFunctionExpression::Literal(Function { params, body }) => {
                        let func_env = env.clone();

                        env.set_return_val(Object::Function {
                            params: params.clone(),
                            body: body.clone(),
                            env: Box::new(func_env),
                        })
                    }
                }
                .map(|env| eval_multiple(env, arguments))
            }
        }
    }
}

// TODO clean this up
fn eval_multiple(env: Env, arguments: &Vec<Expression>) -> Env {
    env.map_return_obj(|object| {
        // Check parameters
        match &object {
            Object::Function {
                params,
                body: _,
                env: _,
            } => {
                if arguments.len() != params.len() {
                    // TODO more information in error
                    Err(Error::CallExpressionWrongNumArgs)
                } else {
                    Ok(object)
                }
            }
            _ => panic!("Checks have been done earlier"),
        }
    })
    .map_separated(|env, object| match object {
        Object::Function {
            params,
            body,
            env: func_env,
        } => {
            let child_env = Env::new_extending(env.clone());

            // evaluate arguments in child env
            //
            // TODO handle errors from this env
            let env_with_args =
                eval_multiple_args(child_env, arguments, params).with_parent(*func_env);

            // println!("=========");

            // evalute body with arguments
            let return_result = body.eval(env_with_args).get_result_owned();

            env.set_return_result(return_result)
        }
        _ => panic!(),
    })
}

fn eval_multiple_args(env: Env, args: &Vec<Expression>, params: Vec<String>) -> Env {
    let zipped = args.iter().zip(params);
    zipped.fold(env, |acc, (expr, param_name)| {
        expr.eval(acc).bind_return_value_to_store(param_name)
    })
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
            left: left,
            right: right,
        }),
    }
}

fn eval_if_expr<'a, 'b>(
    env: Env,
    condition: &'b Box<Expression>,
    consequence: &'b Statements,
    alternative: &'b Statements,
) -> Env {
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
