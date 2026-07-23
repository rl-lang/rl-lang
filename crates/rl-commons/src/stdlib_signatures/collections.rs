//! Typed signatures for `std::collections`.

use super::{params, result, t};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;
use std::rc::Rc;

pub fn module() -> ModuleNames {
    ModuleNames::new("collections")
        .with_typed_function(set_add())
        .with_typed_function(set_contains())
        .with_typed_function(set_is_empty())
        .with_typed_function(set_len())
        .with_typed_function(set_remove())
        .with_typed_function(set_to_array())
        .with_typed_function(map_contains())
        .with_typed_function(map_remove())
        .with_typed_function(map_len())
        .with_typed_function(map_is_empty())
        .with_typed_function(map_to_array())
        .with_typed_function(map_get())
        .with_typed_function(map_keys())
        .with_typed_function(map_values())
        .with_typed_function(map_clear())
        .with_typed_function(map_merge())
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

/// The generic key placeholder `K`, shared by every `map_*` signature.
fn k() -> T {
    T::Generic("K".into())
}

/// The generic value placeholder `V`, shared by every `map_*` signature.
fn v() -> T {
    T::Generic("V".into())
}

/// `map[K, V]` - shorthand for "map of the generic key/value pair".
fn map_t() -> T {
    T::Map(Box::new(k()), Box::new(v()))
}

fn map_contains() -> StdFn {
    StdFn::typed(
        "map_contains",
        vec![(params(vec![map_t(), k()]), result(T::Bool))],
    )
}

fn map_remove() -> StdFn {
    StdFn::typed(
        "map_remove",
        vec![(params(vec![map_t(), k()]), result(map_t()))],
    )
}

fn map_len() -> StdFn {
    StdFn::typed("map_len", vec![(params(vec![map_t()]), result(T::Int))])
}

fn map_is_empty() -> StdFn {
    StdFn::typed(
        "map_is_empty",
        vec![(params(vec![map_t()]), result(T::Bool))],
    )
}

fn map_to_array() -> StdFn {
    StdFn::typed(
        "map_to_array",
        vec![(
            params(vec![map_t()]),
            result(T::Array(Box::new(T::Tuple(Rc::new(vec![k(), v()]))))),
        )],
    )
}

fn map_get() -> StdFn {
    StdFn::typed("map_get", vec![(params(vec![map_t(), k()]), result(v()))])
}

fn map_keys() -> StdFn {
    StdFn::typed(
        "map_keys",
        vec![(params(vec![map_t()]), result(T::Array(Box::new(k()))))],
    )
}

fn map_values() -> StdFn {
    StdFn::typed(
        "map_values",
        vec![(params(vec![map_t()]), result(T::Array(Box::new(v()))))],
    )
}

fn map_clear() -> StdFn {
    StdFn::typed("map_clear", vec![(params(vec![map_t()]), result(map_t()))])
}

fn map_merge() -> StdFn {
    StdFn::typed(
        "map_merge",
        vec![(params(vec![map_t(), map_t()]), result(map_t()))],
    )
}
