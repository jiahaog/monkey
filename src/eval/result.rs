use super::error::Error;
use super::object::Object;
use either::Either;

// This module contains a EvalResult as a wrapper around Either, to manage control flow of
// expressions. normal objects are on the left, and can be manipulated by map_left for either,
// while returning objects and errors are on the right. This allows us to keep calling
// map_left/left_and_then to transform our EvalResult, which does nothing the moment errors or a
// return object is encountered. I suppose this is some sort of monadic concept but I'm not a FP
// expert...
//
// I chose to use Either instead of std::result::Result because it doesn't feel right to put
// return objects in the error side of Result.
//
// The traits here are convenience methods to allow us to easily construct the EvalResult from
// different types with a simple interface, with the goal of owning how Either types are
// constructed.

// This type should not be used outside of this module, but I'm not sure how to make it private
// with the current type alias for EvalResult.
#[derive(Debug)]
pub enum ShortCircuit {
    ReturningObject(Object),
    RuntimeError(Error),
}

pub type EvalResult = Either<Object, ShortCircuit>;

pub trait ToEvalResult {
    fn to_eval_result(self) -> EvalResult;
    fn to_eval_result_return(self) -> EvalResult;
}

impl ToEvalResult for Result<Object, Error> {
    fn to_eval_result(self) -> EvalResult {
        match self {
            Ok(object) => Either::Left(object),
            Err(err) => Either::Right(ShortCircuit::RuntimeError(err)),
        }
    }

    fn to_eval_result_return(self) -> EvalResult {
        self.to_eval_result()
    }
}

impl ToEvalResult for Object {
    fn to_eval_result(self) -> EvalResult {
        // Something like `pure :: a -> f a`
        Either::Left(self)
    }

    fn to_eval_result_return(self) -> EvalResult {
        Either::Right(ShortCircuit::ReturningObject(self))
    }
}

impl ToEvalResult for Error {
    fn to_eval_result(self) -> EvalResult {
        Either::Right(ShortCircuit::RuntimeError(self))
    }

    fn to_eval_result_return(self) -> EvalResult {
        self.to_eval_result()
    }
}

pub trait ToResult {
    fn to_result(self) -> Result<Object, Error>;
}

impl ToResult for EvalResult {
    fn to_result(self) -> Result<Object, Error> {
        match self {
            Either::Left(object) => Ok(object),
            Either::Right(ShortCircuit::ReturningObject(object)) => Ok(object),
            Either::Right(ShortCircuit::RuntimeError(err)) => Err(err),
        }
    }
}
