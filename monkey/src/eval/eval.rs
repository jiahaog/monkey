use super::env::Env;
use super::error::Error;
use super::object::Object;
use std::convert::From;
use std::iter::FromIterator;

pub trait Eval {
    fn eval(self, env: Env) -> EvalResult;
}

#[derive(Debug)]
pub enum ShortCircuit {
    ReturningObject(Object),
    RuntimeError(Error),
}

impl From<Object> for ShortCircuit {
    fn from(obj: Object) -> Self {
        ShortCircuit::ReturningObject(obj)
    }
}

impl From<Error> for ShortCircuit {
    fn from(err: Error) -> Self {
        ShortCircuit::RuntimeError(err)
    }
}

// EvalResult is an alias to manage control flow of evaluated expressions. We use the failure
// portion of the Result type to handle both errors and return statements.
pub type EvalResult = Result<Object, ShortCircuit>;

impl From<Object> for EvalResult {
    // There are other cases where Object is returned from a `Statement::Return`. Note that in those
    // cases we actually want return the right side (or `Err()`) side for EvalResult, and we should
    // use `ShortCircuit::from(object)` instead.
    fn from(obj: Object) -> Self {
        Ok(obj)
    }
}

impl From<Error> for EvalResult {
    fn from(err: Error) -> Self {
        Err(ShortCircuit::RuntimeError(err))
    }
}

// Avoid rust E0117: only traits defined for the current crate can be implemented for arbitrary
// types.
pub struct EvalMultiple(pub Result<Vec<Object>, ShortCircuit>);

// So that we can collect a Vec<EvalResult> into a Result<Vec<Object>, ShortCircuit>.
// This is probably too complicated for just one use of collect(). But it is a good learning
// exercise.
impl FromIterator<EvalResult> for EvalMultiple {
    fn from_iter<I: IntoIterator<Item = EvalResult>>(iter: I) -> Self {
        EvalMultiple(iter.into_iter().fold(Ok(Vec::new()), |acc, eval_result| {
            match (acc, eval_result) {
                (Ok(mut evaluated), Ok(object)) => {
                    evaluated.push(object);
                    Ok(evaluated)
                }
                (Err(err), _) => Err(err),
                (Ok(_), Err(short_circuit)) => Err(short_circuit),
            }
        }))
    }
}
