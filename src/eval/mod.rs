use crate::ast::{Expression, Program, Statement};
use crate::object::Object;

#[cfg(test)]
mod tests;

trait Eval {
    fn eval(&self) -> Object;
}

impl Eval for Program {
    fn eval(&self) -> Object {
        self.statements[0].eval()
    }
}

impl Eval for Statement {
    fn eval(&self) -> Object {
        match self {
            Statement::Let(identifier, expr) => unimplemented!(),
            Statement::Expression(expr) => expr.eval(),
            _ => unimplemented!(),
        }
    }
}

impl Eval for Expression {
    fn eval(&self) -> Object {
        match self {
            Expression::IntegerLiteral(val) => Object::Integer(*val),
            _ => unimplemented!(),
        }
    }
}
