use self::EvalResult::*;
use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::{Env, Object, NULL};

#[cfg(test)]
mod tests;

type Result = std::result::Result<Object, Error>;

#[derive(Debug, PartialEq)]
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
        let (_, result) = self.eval(env);

        match result {
            Return(object) => Ok(object),
            Raw(object) => Ok(object),
            RuntimeError(err) => Err(err),
        }
    }
}

trait Eval {
    fn eval(&self, env: Env) -> (Env, EvalResult);
}

// Internal evaluation result for short circuit of return statements and errors
#[derive(Debug)]
enum EvalResult {
    Raw(Object),
    Return(Object),
    RuntimeError(Error),
}

impl Eval for Program {
    fn eval(&self, env: Env) -> (Env, EvalResult) {
        let (new_env, result) = self.statements.eval(env);

        (
            new_env,
            match result {
                // Unwrap return statement
                Return(x) => Raw(x),
                x => x,
            },
        )
    }
}

impl Eval for Statement {
    fn eval(&self, env: Env) -> (Env, EvalResult) {
        match self {
            Statement::Let(identifier, expr) => {
                let (mut new_env, result) = expr.eval(env);
                match result {
                    RuntimeError(err) => (new_env, RuntimeError(err)),
                    Raw(result) => {
                        new_env.set(identifier.to_string(), result);
                        (new_env, Raw(NULL))
                    }
                    Return(_) => panic!(
                        "Return not allowed here: This should have been disallowed by the parser"
                    ),
                }
            }
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => {
                let (new_env, result) = expr.eval(env);
                (
                    new_env,
                    match result {
                        Raw(x) => Return(x),
                        x => x,
                    },
                )
            }
        }
    }
}

impl Eval for Statements {
    fn eval(&self, env: Env) -> (Env, EvalResult) {
        // short circuit fold (kinda inefficient)
        self.iter()
            .fold((env, Raw(NULL)), |(prev_env, acc), statement| match acc {
                Return(_) => (prev_env, acc),
                RuntimeError(_) => (prev_env, acc),
                _ => statement.eval(prev_env),
            })
    }
}

impl Eval for Expression {
    fn eval(&self, env: Env) -> (Env, EvalResult) {
        // TODO there are some unimplemented cases here
        match self {
            Expression::Identifier(name) => match env.get(name) {
                Some(&val) => (env, Raw(val)),
                None => (
                    env,
                    RuntimeError(Error::IdentifierNotFound {
                        name: name.to_string(),
                    }),
                ),
            },
            // TODO check if this is safe
            Expression::IntegerLiteral(val) => (env, Raw(Object::Integer(*val as isize))),
            Expression::Boolean(val) => (env, Raw(Object::from_bool_val(*val))),
            Expression::Prefix { operator, right } => {
                let (new_env, result) = right.eval(env);
                (new_env, eval_prefix_expr(*operator, result))
            }
            Expression::Infix {
                operator,
                left,
                right,
            } => {
                let (new_env, left_result) = left.eval(env);
                let (new_env2, right_result) = right.eval(new_env);

                (
                    new_env2,
                    eval_infix_expr(*operator, left_result, right_result),
                )
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

fn eval_infix_expr(operator: Operator, left: EvalResult, right: EvalResult) -> EvalResult {
    match (operator, left, right) {
        (_, RuntimeError(x), _) => RuntimeError(x),
        (_, _, RuntimeError(x)) => RuntimeError(x),
        (_, Return(x), _) => Return(x),
        (_, _, Return(x)) => Return(x),
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
) -> (Env, EvalResult) {
    let (new_env, result) = condition.eval(env);

    match result {
        Raw(x) => {
            if x.is_truthy() {
                consequence.eval(new_env)
            } else {
                alternative.eval(new_env)
            }
        }
        x => (new_env, x),
    }
}
