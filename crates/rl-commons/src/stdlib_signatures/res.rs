//! Typed signatures for `std::res`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("res")
        .with_functions(&["result_unwrap_err", "result_map", "result_map_err"])
        .with_typed_function(is_ok())
        .with_typed_function(is_err())
        .with_typed_function(result_unwrap())
        .with_typed_function(result_unwrap_or())
}

fn t() -> T {
    T::Generic("T".into())
}

fn result_predicate(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![result(t())]), T::Bool)])
}
fn is_ok() -> StdFn {
    result_predicate("is_ok")
}
fn is_err() -> StdFn {
    result_predicate("is_err")
}

fn result_unwrap() -> StdFn {
    StdFn::typed("result_unwrap", vec![(params(vec![result(t())]), t())])
}

fn result_unwrap_or() -> StdFn {
    StdFn::typed(
        "result_unwrap_or",
        vec![(params(vec![result(t()), t()]), t())],
    )
}
