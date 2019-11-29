use super::Env;
use crate::ast;
use crate::ast::{format_vec, Statements};
use std::convert::From;
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
    List(Vec<Object>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum BuiltIn {
    Len,
    Index,
}

impl BuiltIn {
    pub fn register(env: Env) -> Env {
        env.set("len".to_string(), Object::BuiltIn(BuiltIn::Len));
        // Not all built-ins are here such as `Index` because it can be called using `[$index]`.
        // This allows us to reuse the apply logic of the built-ins for operators.
        env
    }
}

impl fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuiltIn::Len => write!(f, "<built-in function len>"),
            BuiltIn::Index => write!(f, "<built-in function index>"),
        }
    }
}

// Cache the constants for performance. This might not be necessary.
const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
pub const NULL: Object = Object::Null;

impl From<bool> for Object {
    fn from(val: bool) -> Self {
        match val {
            true => TRUE,
            false => FALSE,
        }
    }
}

impl Object {
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
            Object::List(_) => "List",
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
            Object::List(values) => write!(f, "[{}]", format_vec(values)),
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
    pub fn new(env: Env, ast::Function { params, body }: ast::Function) -> Self {
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
