mod apply;
mod error;
mod eval;

#[cfg(test)]
mod tests;

use self::apply::Applicable;
pub use self::error::Error;
use self::eval::{eval_exprs, Eval, EvalResult, ShortCircuit};
use crate::ast::{CallFunctionExpression, Expression, Operator, Program, Statement, Statements};
use crate::object::{BuiltIn, Env, Function, Object, NULL};

impl Program {
    pub fn evaluate(self, env: Env) -> Result<Object, Error> {
        self.eval(env).or_else(|short_circuit| match short_circuit {
            ShortCircuit::ReturningObject(object) => Ok(object),
            ShortCircuit::RuntimeError(err) => Err(err),
        })
    }
}

impl Eval for Program {
    fn eval(self, env: Env) -> EvalResult {
        self.statements.eval(env)
    }
}

impl Eval for Statement {
    fn eval(self, env: Env) -> EvalResult {
        match self {
            Statement::Let(name, expr) => expr.eval(env.clone()).map(|object| {
                env.set(name, object);
                NULL
            }),
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => expr
                .eval(env)
                .and_then(|object| Err(ShortCircuit::from(object))),
        }
    }
}

impl Eval for Statements {
    fn eval(self, env: Env) -> EvalResult {
        self.into_iter().fold(NULL.into(), |acc, statement| {
            // Calling map will do nothing if the acc is already in a returning or error state.
            // There are possibly ways to make this exit immediately
            acc.and_then(|_| statement.eval(env.clone()))
        })
    }
}

impl Eval for Expression {
    fn eval(self, env: Env) -> EvalResult {
        // Useful for debugging:
        // println!("env {:#?}\nexpr {:#?}\n", env, self);

        match self {
            Expression::Identifier(name) => match env.get(&name) {
                Some(object) => object.into(),
                None => Error::IdentifierNotFound {
                    name: name.to_string(),
                }
                .into(),
            },
            // TODO: check if this is safe
            Expression::IntegerLiteral(val) => Object::Integer(val as isize).into(),
            Expression::StringLiteral(val) => Object::Str(val).into(),
            Expression::ListLiteral(vals) => {
                eval_exprs(env, vals).and_then(|objs| Object::List(objs).into())
            }
            Expression::Boolean(val) => Object::from(val).into(),
            Expression::Prefix { operator, right } => right
                .eval(env)
                .and_then(|object| eval_prefix_expr(operator, object)),
            Expression::Infix {
                operator,
                left,
                right,
            } => left.eval(env.clone()).and_then(|left_obj| {
                right
                    .eval(env)
                    .and_then(|right_obj| eval_infix_expr(operator, left_obj, right_obj))
            }),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(env, condition, consequence, alternative),
            Expression::FunctionLiteral(ast_func) => {
                Object::Function(Function::new(env.clone(), ast_func)).into()
            }
            Expression::Call {
                function,
                arguments,
            } => {
                // Normalize identifier or function literal to common function.
                let func_result: EvalResult = match function {
                    CallFunctionExpression::Identifier(name) => env.get(&name).ok_or(
                        Error::IdentifierNotFound {
                            name: name.to_string(),
                        }
                        .into(),
                    ),
                    CallFunctionExpression::Literal(ast_func) => {
                        Ok(Object::Function(Function::new(env.clone(), ast_func)))
                    }
                };

                func_result?.apply(env, arguments)
            }
            Expression::Index { left, index } => BuiltIn::Index.apply(env, vec![*left, *index]),
        }
    }
}

fn eval_prefix_expr(operator: Operator, right: Object) -> EvalResult {
    match (operator, right) {
        (Operator::Not, Object::Boolean(true)) => Ok(Object::from(false)),
        (Operator::Not, Object::Boolean(false)) => Ok(Object::from(true)),
        (Operator::Not, Object::Integer(_)) => Ok(Object::from(false)),
        (Operator::Minus, Object::Integer(val)) => Ok(Object::Integer(-val)),
        (operator, right) => Err(Error::UnknownOperation {
            operator: operator,
            right: right,
        }
        .into()),
    }
}

fn eval_infix_expr(operator: Operator, left: Object, right: Object) -> EvalResult {
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
            Ok(Object::from(left_val < right_val))
        }
        (Operator::GreaterThan, Object::Integer(left_val), Object::Integer(right_val)) => {
            Ok(Object::from(left_val > right_val))
        }
        (Operator::Plus, Object::Str(left_val), Object::Str(right_val)) => {
            Ok(Object::Str(left_val + &right_val))
        }
        (Operator::Equal, left_val, right_val) => Ok(Object::from(left_val == right_val)),
        (Operator::NotEqual, left_val, right_val) => Ok(Object::from(left_val != right_val)),
        (operator, left, right) => Err(Error::TypeMismatch {
            operator: operator,
            left: left,
            right: right,
        }
        .into()),
    }
}

fn eval_if_expr(
    env: Env,
    condition: Box<Expression>,
    consequence: Statements,
    alternative: Statements,
) -> EvalResult {
    condition.eval(env.clone()).and_then(|object| {
        if object.is_truthy() {
            consequence
        } else {
            alternative
        }
        .eval(env)
    })
}
