use super::env::Env;
use super::error::Error;
use super::eval::{Eval, EvalMultiple, EvalResult, ToEvalResult, ToResult};
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
            .to_eval_result(),
        }
    }
}

impl Applicable for BuiltIn {
    // TODO find a more declarative way to write this.
    fn apply(self, env: Env, args: Vec<Expression>) -> EvalResult {
        let args_evaluated = args
            .into_iter()
            .map(|arg| arg.eval(env.clone()))
            .collect::<EvalMultiple>()
            .0;

        match args_evaluated {
            Ok(objects) => match (self, objects.as_slice()) {
                (BuiltIn::Len, [Object::Str(val)]) => Ok(Object::Integer(val.len() as isize)),
                (BuiltIn::Len, [obj]) => Err(Error::TypeError {
                    message: format!("object of type '{}' has no len()", obj.type_str()),
                }),
                (BuiltIn::Len, args) => Err(Error::TypeError {
                    message: format!("len() takes exactly one arguemnt ({} given)", args.len()),
                }),
            }
            .to_eval_result(),
            Err(err) => Err(err).to_eval_result(),
        }
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
            .to_eval_result()
        } else {
            params
                .iter()
                .zip(arguments)
                // evaluate arguments in the current env
                .map(|(name, expr)| {
                    expr.eval(env.clone())
                        .to_result()
                        .map(|object| (name, object))
                })
                .collect::<std::result::Result<Vec<(&String, Object)>, Error>>()
                // bind argument results to a new env which extends the function env
                .map(|name_and_objects| {
                    bind_objects_to_env(Env::new_extending(func_env), name_and_objects)
                })
                // alternative to body.clone() here would be to put RC on all AST objects
                // which is a bit too much
                .and_then(|child_env| body.as_ref().clone().eval(child_env).to_result())
                .to_eval_result()
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
