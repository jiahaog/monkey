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
    Str(String),
    // Separate this out because it simplifies passing the specific enum variant around with helper
    // functions for function call evaluations
    Function(Function),
    BuiltIn(BuiltIn),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BuiltIn {
    Len,
}

impl BuiltIn {
    pub fn from_identifier(name: &str) -> Option<Self> {
        match name {
            "len" => Some(BuiltIn::Len),
            _ => None,
        }
    }
}
impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltIn::Len => write!(f, "<built-in function len>"),
        }
    }
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

    pub fn type_str(&self) -> String {
        match self {
            Object::Null => "NullType",
            Object::Boolean(_) => "bool",
            Object::Integer(_) => "int",
            Object::Function(_) => "function",
            Object::Str(_) => "string",
            Object::BuiltIn(_) => "BuiltIn",
        }
        .to_string()
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::Integer(val) => write!(f, "{}", val),
            Object::Str(val) => write!(f, "{}", val),
            Object::Function(func) => write!(f, "{}", func),
            Object::BuiltIn(built_in) => write!(f, "{}", built_in),
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

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn({}) {{{}}}",
            self.params.join(", "),
            self.body.iter().fold(String::from("\n"), |acc, line| acc
              // 4 spaces for indentation
                + &format!("    {};\n", line))
        )
    }
}
