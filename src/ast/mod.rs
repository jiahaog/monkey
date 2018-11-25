#![allow(dead_code)]

#[derive(PartialEq, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
    Boolean(bool),
    IntegerLiteral(usize),
    PrefixExpression(Operator, Box<Expression>),
    InfixExpression(Box<Expression>, Operator, Box<Expression>),
    // IfExpression,
    // FunctionLiteral,
    // CallExpression,
    DummyExpression, // TODO remove me
}

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
