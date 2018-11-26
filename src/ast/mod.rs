#![allow(dead_code)]

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

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
    Boolean(bool),
    IntegerLiteral(usize),
    // TODO remove the `Expression` postfix in the names
    PrefixExpression {
        operator: Operator,
        right: Box<Expression>,
    },
    InfixExpression {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    // IfExpression,
    // FunctionLiteral,
    // CallExpression,
    DummyExpression, // TODO remove me
}

// TODO remove the `Statement` postfix in the names
#[derive(PartialEq, Debug)]
pub enum Statement {
    LetStatement(String, Expression),
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
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
