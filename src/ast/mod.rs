use std::fmt;

#[cfg(test)]
mod tests;

use self::Expression::*;
use self::Operator::*;

#[derive(PartialEq, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Not,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_val = match self {
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
            Not => "!",
            LessThan => "<",
            GreaterThan => ">",
            Equal => "==",
            NotEqual => "!=",
        };
        write!(f, "{}", str_val)
    }
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(usize),
    // TODO remove the `Expression` postfix in the names
    Prefix {
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // TODO
    Boolean(bool),
    If {
        condition: Box<Expression>,
        consequence: Vec<Statement>,
        alternative: Vec<Statement>,
    },
    FunctionLiteral {
        // This should be of the Identifier variant only
        // https://github.com/rust-lang/rfcs/pull/2593
        params: Vec<Expression>,
        body: Vec<Statement>,
    },
    Call {
        // This should be of the Identifier or FunctionLiteral variant only
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    DummyExpression, // TODO remove me
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_val: String = match *self {
            Identifier(ref name) => name.to_string(),
            IntegerLiteral(ref val) => val.to_string(),
            Prefix {
                ref operator,
                ref right,
            } => format!("({}{})", operator, right),
            Infix {
                ref operator,
                ref left,
                ref right,
            } => format!("({} {} {})", left, operator, right),
            Boolean(ref val) => val.to_string(),
            Call {
                ref function,
                ref arguments,
            } => format!("{}({})", function, {
                let mut vec_str = arguments
                    .iter()
                    .fold(String::new(), |acc, x| format!("{}{}, ", acc, &x));
                // remove the last ', ' from the string
                vec_str.truncate(vec_str.len() - 2);
                vec_str
            }),
            ref x => unimplemented!("Token: {:?}", x),
        };
        write!(f, "{}", string_val)
    }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(Debug)]
pub struct Program {
    // TODO Maybe statements can be an iterator instead...?
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Program { statements }
    }
}
