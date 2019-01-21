use super::object::Object;
use crate::ast::{Expression, Operator};

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
    CallExpressionExpectedFunction {
        received: Object,
    },
    CallExpressionWrongNumArgs {
        params: Vec<String>,
        arguments: Vec<Expression>,
    },
}
