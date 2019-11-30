use super::env::Env;
use super::error::Error;
use super::eval::{eval_exprs, Eval, EvalResult};
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
        let objects = eval_exprs(env, args)?;

        match (self, objects.as_slice()) {
            (BuiltIn::Len, [Object::Str(val)]) => Ok(Object::Integer(val.len() as isize)),
            (BuiltIn::Len, [Object::List(vals)]) => Ok(Object::Integer(vals.len() as isize)),
            (BuiltIn::Len, [wrong_list_type]) => Err(Error::TypeError {
                message: format!(
                    "object of type '{}' has no len()",
                    wrong_list_type.type_str()
                ),
            }),
            (BuiltIn::Len, wrong_num_args) => Err(Error::TypeError {
                message: format!(
                    "len() takes exactly one argument ({} given)",
                    wrong_num_args.len()
                ),
            }),
            (BuiltIn::Index, [Object::List(vals), Object::Integer(index)]) => {
                if index < &0 {
                    return Error::TypeError {
                        message: "list indices must be positive".to_string(),
                    }
                    .into();
                }
                match vals.get(*index as usize) {
                    Some(val) => Ok(val.clone()),
                    None => Ok(Object::Null),
                }
            }
            (BuiltIn::Index, [Object::List(_), wrong_index_type]) => Err(Error::TypeError {
                message: format!(
                    "list indices must be integers, not {}",
                    wrong_index_type.type_str()
                ),
            }),
            (BuiltIn::Index, [wrong_list_type, _]) => Err(Error::TypeError {
                message: format!(
                    "object of type '{}' has no index",
                    wrong_list_type.type_str()
                ),
            }),
            (BuiltIn::Index, wrong_num_args) => Err(Error::TypeError {
                message: format!(
                    "index() takes exactly two arguments ({} given)",
                    wrong_num_args.len()
                ),
            }),
            (BuiltIn::Push, [Object::List(old_vals), new_element]) => {
                let mut new = old_vals.clone();
                new.push(new_element.clone());
                Ok(Object::List(new))
            }
            (BuiltIn::Push, [wrong_list_type, _]) => Err(Error::TypeError {
                message: format!(
                    "object of type '{}' has no push",
                    wrong_list_type.type_str()
                ),
            }),
            (BuiltIn::Push, wrong_num_args) => Err(Error::TypeError {
                message: format!(
                    "push() takes exactly two arguments ({} given)",
                    wrong_num_args.len()
                ),
            }),
            (BuiltIn::Rest, [Object::List(vals)]) => Ok(match vals.get(1..) {
                Some(x) => Object::List(x.into()),
                None => Object::Null,
            }),
            (BuiltIn::Rest, [wrong_list_type]) => Err(Error::TypeError {
                message: format!(
                    "object of type '{}' has no rest()",
                    wrong_list_type.type_str()
                ),
            }),
            (BuiltIn::Rest, wrong_num_args) => Err(Error::TypeError {
                message: format!(
                    "rest() takes exactly one argument ({} given)",
                    wrong_num_args.len()
                ),
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
        // Check params.
        if params.len() != arguments.len() {
            Error::CallExpressionWrongNumArgs {
                params: params.to_vec(), // not really sure what to_vec() does
                arguments: arguments,
            }
            .into()
        } else {
            // Evaluate arguments in the current env.
            let evaluated: Vec<Object> = eval_exprs(env.clone(), arguments)?;

            // bind argument results to a new env which extends the function env.
            let env_with_objects = params.iter().zip(evaluated).fold(
                Env::new_extending(func_env),
                |acc, (name, obj)| {
                    acc.set(name.to_string(), obj);
                    acc
                },
            );
            // Alternative to body.clone() here would be to put RC on all AST objects
            // which is a bit too much/
            body.as_ref().clone().eval(env_with_objects)
        }
    }
}
