//! Typed signatures for `std::collections`.

use super::{params, result, t};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("collections")
        .with_typed_function(set_add())
        .with_typed_function(set_contains())
        .with_typed_function(set_is_empty())
        .with_typed_function(set_len())
        .with_typed_function(set_remove())
        .with_typed_function(set_to_array())
}

fn set_t() -> T {
    T::Set(Box::new(t()))
}

fn set_and_value_to_set(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![set_t(), t()]), result(set_t()))])
}
fn set_add() -> StdFn {
    set_and_value_to_set("set_add")
}
fn set_remove() -> StdFn {
    set_and_value_to_set("set_remove")
}

fn set_contains() -> StdFn {
    StdFn::typed(
        "set_contains",
        vec![(params(vec![set_t(), t()]), result(T::Bool))],
    )
}

fn set_len() -> StdFn {
    StdFn::typed("set_len", vec![(params(vec![set_t()]), result(T::Int))])
}

fn set_is_empty() -> StdFn {
    StdFn::typed(
        "set_is_empty",
        vec![(params(vec![set_t()]), result(T::Bool))],
    )
}

fn set_to_array() -> StdFn {
    StdFn::typed(
        "set_to_array",
        vec![(params(vec![set_t()]), result(T::Array(Box::new(t()))))],
    )
}
