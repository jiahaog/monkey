use super::error::Error;
use super::Env;
use crate::ast;
use crate::ast::Statements;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Null,
    Boolean(bool),
    Integer(isize),
    // Separate this out because it simplifies passing the specific enum variant around with helper
    // functions for function call evaluations
    Function(Function),
}

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
pub const NULL: Object = Object::Null;

impl Object {
    pub fn from_bool_val(val: bool) -> Self {
        match val {
            true => TRUE,
            false => FALSE,
        }
    }
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(false) | Object::Null => false,
            _ => true,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Statements,
    pub env: Env,
}

impl Function {
    pub fn from_object(object: Object) -> Result<Self, Error> {
        match object {
            Object::Function(func) => Ok(func),
            object => Err(Error::CallExpressionExpectedFunction {
                received: object.clone(),
            }),
        }
    }

    pub fn from_ast_fn(env: Env, func: ast::Function) -> Self {
        let ast::Function { params, body } = func;

        Self {
            params,
            body,
            env: env,
        }
    }
}
