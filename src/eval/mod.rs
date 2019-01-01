mod env;
mod error;

#[cfg(test)]
mod tests;

use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::{Object, NULL};

pub use self::env::Env;
use self::error::Error;

// TODO Avoid cloning objects in Errors

type Result<'a> = std::result::Result<&'a Object, &'a Error>;

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
            } => left.eval(env).map(|left_env| {
                let left_obj = left_env.get_result().unwrap().clone();

                right
                    .eval(left_env)
                    .map_return_obj(|right_obj| eval_infix_expr(operator, left_obj, right_obj))
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
