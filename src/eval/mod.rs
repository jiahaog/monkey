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
    pub fn evaluate(&self, env: &mut Env) -> Result {
        match self.eval(env) {
            Return(object) => Ok(object),
            Raw(object) => Ok(object),
            RuntimeError(err) => Err(err),
        }
    }
}

trait Eval {
    fn eval(&self, env: &mut Env) -> EvalResult;
}

// Internal evaluation result for short circuit of return statements and errors
#[derive(Debug)]
enum EvalResult {
    Raw(Object),
    Return(Object),
    RuntimeError(Error),
}

impl Eval for Program {
    fn eval(&self, env: &mut Env) -> EvalResult {
        match self.statements.eval(env) {
            // Unwrap return statement
            Return(x) => Raw(x),
            x => x,
        }
    }
}

impl Eval for Statement {
    fn eval(&self, env: &mut Env) -> EvalResult {
        match self {
            Statement::Let(identifier, expr) => match expr.eval(env) {
                RuntimeError(err) => RuntimeError(err),
                Raw(result) => {
                    env.set(identifier.to_string(), result);
                    Raw(NULL)
                }
                Return(_) => panic!(
                    "Return not allowed here: This should have been disallowed by the parser"
                ),
            },
            Statement::Expression(expr) => expr.eval(env),
            Statement::Return(expr) => match expr.eval(env) {
                Raw(x) => Return(x),
                x => x,
            },
        }
    }
}

impl Eval for Statements {
    fn eval(&self, env: &mut Env) -> EvalResult {
        // short circuit fold (kinda inefficient)
        self.iter().fold(Raw(NULL), |acc, statement| match acc {
            Return(_) => acc,
            RuntimeError(_) => acc,
            _ => statement.eval(env),
        })
    }
}

impl Eval for Expression {
    fn eval(&self, env: &mut Env) -> EvalResult {
        // TODO there are some unimplemented cases here
        match self {
            Expression::Identifier(name) => match env.get(name) {
                Some(&val) => Raw(val),
                None => RuntimeError(Error::IdentifierNotFound {
                    name: name.to_string(),
                }),
            },
            // TODO check if this is safe
            Expression::IntegerLiteral(val) => Raw(Object::Integer(*val as isize)),
            Expression::Boolean(val) => Raw(Object::from_bool_val(*val)),
            Expression::Prefix { operator, right } => eval_prefix_expr(*operator, right.eval(env)),
            Expression::Infix {
                operator,
                left,
                right,
            } => eval_infix_expr(*operator, left.eval(env), right.eval(env)),
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
        // Relying on PartialOrd and PartialEq
        (Operator::LessThan, Raw(left_val), Raw(right_val)) => {
            Raw(Object::from_bool_val(left_val < right_val))
        }
        (Operator::GreaterThan, Raw(left_val), Raw(right_val)) => {
            Raw(Object::from_bool_val(left_val > right_val))
        }
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
    env: &mut Env,
    condition: &Box<Expression>,
    consequence: &Statements,
    alternative: &Statements,
) -> EvalResult {
    match condition.eval(env) {
        Raw(x) => {
            if x.is_truthy() {
                consequence.eval(env)
            } else {
                alternative.eval(env)
            }
        }
        x => x,
    }
}
