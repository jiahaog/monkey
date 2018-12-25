use crate::ast::{Expression, Operator, Program, Statement, Statements};
use crate::object::Object;

#[cfg(test)]
mod tests;

pub trait Eval {
    fn eval(&self) -> Object;
}

impl Eval for Program {
    fn eval(&self) -> Object {
        self.statements.eval()
    }
}

impl Eval for Statement {
    fn eval(&self) -> Object {
        match self {
            Statement::Let(identifier, expr) => unimplemented!(),
            Statement::Expression(expr) => expr.eval(),
            Statement::Return(expr) => Object::Return(Box::new(expr.eval())),
        }
    }
}

impl Eval for Statements {
    fn eval(&self) -> Object {
        // short circuit fold (kinda inefficient)
        let wrapped_return = self.iter().fold(Object::Null, |acc, statement| match acc {
            Object::Return(_) => acc,
            _ => statement.eval(),
        });

        match wrapped_return {
            Object::Return(x) => *x,
            x => x,
        }
    }
}

impl Eval for Expression {
    fn eval(&self) -> Object {
        // TODO there are some unimplemented cases here
        match self {
            // TODO check if this is safe
            Expression::IntegerLiteral(val) => Object::Integer(*val as isize),
            Expression::Boolean(val) => Object::from_bool_val(*val),
            Expression::Prefix { operator, right } => eval_prefix_expr(operator, &right.eval()),
            Expression::Infix {
                operator,
                left,
                right,
            } => eval_infix_expr(operator, &left.eval(), &right.eval()),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => eval_if_expr(condition, consequence, alternative),
            x => unimplemented!("{:?}", x),
        }
    }
}

fn eval_prefix_expr(operator: &Operator, right: &Object) -> Object {
    match (operator, right) {
        (Operator::Not, Object::Boolean(true)) => Object::from_bool_val(false),
        (Operator::Not, Object::Boolean(false)) => Object::from_bool_val(true),
        (Operator::Not, Object::Integer(_)) => Object::from_bool_val(false),
        (Operator::Minus, Object::Integer(val)) => Object::Integer(-val),
        // TODO return result instead of panicking on unsupported ops
        x => unimplemented!("{:?}", x),
    }
}

fn eval_infix_expr(operator: &Operator, left: &Object, right: &Object) -> Object {
    match (operator, left, right) {
        (Operator::Plus, Object::Integer(left_val), Object::Integer(right_val)) => {
            Object::Integer(left_val + right_val)
        }
        (Operator::Minus, Object::Integer(left_val), Object::Integer(right_val)) => {
            Object::Integer(left_val - right_val)
        }
        (Operator::Multiply, Object::Integer(left_val), Object::Integer(right_val)) => {
            Object::Integer(left_val * right_val)
        }
        (Operator::Divide, Object::Integer(left_val), Object::Integer(right_val)) => {
            Object::Integer(left_val / right_val)
        }
        // Relying on PartialOrd and PartialEq
        (Operator::LessThan, left_val, right_val) => Object::from_bool_val(left_val < right_val),
        (Operator::GreaterThan, left_val, right_val) => Object::from_bool_val(left_val > right_val),
        (Operator::Equal, left_val, right_val) => Object::from_bool_val(left_val == right_val),
        (Operator::NotEqual, left_val, right_val) => Object::from_bool_val(left_val != right_val),
        // TODO return result instead of panicking on unsupported ops
        x => unimplemented!("{:?}", x),
    }
}

fn eval_if_expr(
    condition: &Box<Expression>,
    consequence: &Statements,
    alternative: &Statements,
) -> Object {
    if condition.eval().is_truthy() {
        consequence.eval()
    } else {
        alternative.eval()
    }
}
