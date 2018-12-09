use std::fmt;

#[cfg(test)]
mod tests;

use self::Expression::*;
use self::Operator::*;
use self::Statement::*;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

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

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
    Prefix {
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
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
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
            FunctionLiteral {
                ref params,
                ref body,
            } => format!("fn({}) {{ {} }}", format_vec(params), format_vec(body)),
            Call {
                ref function,
                ref arguments,
            } => format!("{}({})", function, format_vec(arguments)),
            ref x => unimplemented!("Token: {:?}", x),
        };
        write!(f, "{}", string_val)
    }
}

fn format_vec<T: Display>(vec: &Vec<T>) -> String {
    let mut vec_str = vec
        .iter()
        .fold(String::new(), |acc, x| format!("{}{}, ", acc, &x));
    // remove the last ', ' from the string
    vec_str.truncate(vec_str.len() - 2);
    vec_str
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string_val: String = match *self {
            Let(ref name, ref expr) => format!("let {} = {}", name, expr.to_string()),
            Return(ref expr) => expr.to_string(),
            Expression(ref expr) => expr.to_string(),
        };
        write!(f, "{}", string_val)
    }
}
