use super::env::Env;
use super::error::Error;
use super::eval::{Eval, EvalMultiple, EvalResult, ShortCircuit};
use super::object::{BuiltIn, Function, Object};
use crate::ast::Expression;

pub trait Applicable {
    fn apply(self, env: Env, arguments: Vec<Expression>) -> EvalResult;
}

impl Applicable for Object {
    fn apply(self, env: Env, arguments: Vec<Expression>) -> EvalResult {
        match self {
            Object::Function(func) => func.apply(env, arguments),
            Object::BuiltIn(built_in) => built_in.apply(env, arguments),
            object => Error::CallExpressionExpectedFunction {
                received: object.clone(),
            }
            .into(),
        }
    }
}

impl Applicable for BuiltIn {
    fn apply(self, env: Env, args: Vec<Expression>) -> EvalResult {
        let objects = args
            .into_iter()
            .map(|arg| arg.eval(env.clone()))
            .collect::<EvalMultiple>()
            .0?;

        match (self, objects.as_slice()) {
            (BuiltIn::Len, [Object::Str(val)]) => Ok(Object::Integer(val.len() as isize)),
            (BuiltIn::Len, [obj]) => Err(Error::TypeError {
                message: format!("object of type '{}' has no len()", obj.type_str()),
            }),
            (BuiltIn::Len, args) => Err(Error::TypeError {
                message: format!("len() takes exactly one arguemnt ({} given)", args.len()),
            }),
        }
        .map_err(|err| err.into())
    }
}

impl Applicable for Function {
    fn apply(self, env: Env, arguments: Vec<Expression>) -> EvalResult {
        let Function {
            params,
            body,
            env: func_env,
        } = self;
        // check params
        if params.len() != arguments.len() {
            Error::CallExpressionWrongNumArgs {
                params: params.to_vec(), // not really sure what to_vec() does
                arguments: arguments,
            }
            .into()
        } else {
            params
                .iter()
                .zip(arguments)
                // evaluate arguments in the current env
                .map(|(name, expr)| expr.eval(env.clone()).map(|object| (name, object)))
                .collect::<std::result::Result<Vec<(&String, Object)>, ShortCircuit>>()
                // bind argument results to a new env which extends the function env
                .map(|name_and_objects| {
                    bind_objects_to_env(Env::new_extending(func_env), name_and_objects)
                })
                // alternative to body.clone() here would be to put RC on all AST objects
                // which is a bit too much
                .and_then(|child_env| body.as_ref().clone().eval(child_env))
        }
    }
}

fn bind_objects_to_env(env: Env, names_and_objects: Vec<(&String, Object)>) -> Env {
    names_and_objects
        .into_iter()
        .fold(env, |env, (name, object)| {
            env.set(name.to_string(), object);
            env
        })
}
