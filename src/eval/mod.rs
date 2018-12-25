use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::Object;

#[cfg(test)]
mod tests;

type Result = std::result::Result<Object, Error>;

pub trait Eval {
    fn eval(&self) -> Result;
}

#[derive(Debug)]
pub enum Error {}

impl Eval for Program {
    fn eval(&self) -> Result {
        // Unwrap return statements
        match self.statements.eval() {
            Ok(Object::Return(x)) => Ok(*x),
            x => x,
        }
    }
}

impl Eval for Statement {
    fn eval(&self) -> Result {
        match self {
            Statement::Let(_identifier, _expr) => unimplemented!(),
            Statement::Expression(expr) => expr.eval(),
            Statement::Return(expr) => expr.eval().map(|result| Object::Return(Box::new(result))),
        }
    }
}

impl Eval for Statements {
    fn eval(&self) -> Result {
        // short circuit fold (kinda inefficient)
        self.iter()
            .fold(Ok(Object::Null), |acc, statement| match acc {
                Ok(Object::Return(_)) => acc,
                Err(_) => acc,
                _ => statement.eval(),
            })
    }
}

impl Eval for Expression {
    fn eval(&self) -> Result {
        // TODO there are some unimplemented cases here
        match self {
            // TODO check if this is safe
            Expression::IntegerLiteral(val) => Ok(Object::Integer(*val as isize)),
            Expression::Boolean(val) => Ok(Object::from_bool_val(*val)),
            Expression::Prefix { operator, right } => eval_prefix_expr(operator, right.eval()),
            Expression::Infix {
                operator,
                left,
                right,
            } => eval_infix_expr(operator, left.eval(), right.eval()),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(condition, consequence, alternative),
            x => unimplemented!("{:?}", x),
        }
    }
}

fn eval_prefix_expr(operator: &Operator, right: Result) -> Result {
    if let Err(_) = right {
        return right;
    }
    match (operator, right.unwrap()) {
        (Operator::Not, Object::Boolean(true)) => Ok(Object::from_bool_val(false)),
        (Operator::Not, Object::Boolean(false)) => Ok(Object::from_bool_val(true)),
        (Operator::Not, Object::Integer(_)) => Ok(Object::from_bool_val(false)),
        (Operator::Minus, Object::Integer(val)) => Ok(Object::Integer(-val)),
        // TODO return result instead of panicking on unsupported ops
        x => unimplemented!("{:?}", x),
    }
}

fn eval_infix_expr(operator: &Operator, left: Result, right: Result) -> Result {
    match (&left, &right) {
        (Err(_), _) => return left,
        (_, Err(_)) => return right,
        _ => (),
    };

    match (operator, left.unwrap(), right.unwrap()) {
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
        // Relying on PartialOrd and PartialEq
        (Operator::LessThan, left_val, right_val) => {
            Ok(Object::from_bool_val(left_val < right_val))
        }
        (Operator::GreaterThan, left_val, right_val) => {
            Ok(Object::from_bool_val(left_val > right_val))
        }
        (Operator::Equal, left_val, right_val) => Ok(Object::from_bool_val(left_val == right_val)),
        (Operator::NotEqual, left_val, right_val) => {
            Ok(Object::from_bool_val(left_val != right_val))
        }
        // TODO return result instead of panicking on unsupported ops
        x => unimplemented!("{:?}", x),
    }
}

fn eval_if_expr(
    condition: &Box<Expression>,
    consequence: &Statements,
    alternative: &Statements,
) -> Result {
    match condition.eval() {
        Ok(x) => {
            if x.is_truthy() {
                consequence.eval()
            } else {
                alternative.eval()
            }
        }
        err @ _ => err,
    }
}
