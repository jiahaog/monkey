#![allow(dead_code)]

#[derive(PartialEq, Debug)]
pub enum Expression {
    // TODO fixme
    DummyExpression,
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    LetStatement(String, Expression),
    ReturnStatement(Expression),
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
