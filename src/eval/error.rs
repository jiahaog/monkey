use super::object::Object;
use crate::ast::Operator;

#[derive(Debug, PartialEq, Clone)]
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
    CallExpressionWrongNumArgs,
}
