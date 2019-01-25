use super::error::Error;
use super::Env;
use crate::ast;
use crate::ast::Statements;
use std::fmt;
use std::rc::Rc;

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

#[derive(PartialEq, Clone)]
pub struct Function {
    // Using Rc here makes Function cheap to clone
    pub params: Rc<Vec<String>>,
    pub body: Rc<Statements>,
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

    pub fn from_ast_fn(env: Env, ast::Function { params, body }: ast::Function) -> Self {
        // TODO cloning ast function fields is O(n), maybe we want to fix this
        Self {
            params: Rc::new(params),
            body: Rc::new(body),
            env: env,
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Exclude Env from output to avoid stack overflow
        write!(
            f,
            "Function {{ params: {:#?}, body: {:#?}, env: <omitted> }}",
            self.params, self.body
        )
    }
}
