//! Typed signatures for `std::rl`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;
use std::rc::Rc;

pub fn module() -> ModuleNames {
    ModuleNames::new("rl")
        .with_typed_function(lex())
        .with_typed_function(check())
        .with_typed_function(eval())
        .with_typed_function(eval_isolated())
        .with_typed_function(rl_version())
        .with_typed_function(source_name())
}

fn lex() -> StdFn {
    let token = T::Tuple(Rc::new(vec![T::String, T::String, T::Int]));
    StdFn::typed(
        "lex",
        vec![(params(vec![T::String]), result(T::Array(Box::new(token))))],
    )
}

fn check() -> StdFn {
    StdFn::typed("check", vec![(params(vec![T::String]), result(T::Null))])
}

fn eval() -> StdFn {
    StdFn::typed("eval", vec![(params(vec![T::String]), result(T::String))])
}

fn eval_isolated() -> StdFn {
    StdFn::typed(
        "eval_isolated",
        vec![(params(vec![T::String]), result(T::String))],
    )
}

fn rl_version() -> StdFn {
    StdFn::typed("rl_version", vec![(params(vec![]), T::String)])
}

fn source_name() -> StdFn {
    StdFn::typed("source_name", vec![(params(vec![]), T::String)])
}
