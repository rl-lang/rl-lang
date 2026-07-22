//! Typed signatures for `std::time`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("time")
        .with_typed_function(time_now())
        .with_typed_function(time_now_ms())
        .with_typed_function(format_time())
        .with_typed_function(format_date_str())
        .with_typed_function(format_time_str())
        .with_typed_function(time_add())
        .with_typed_function(time_diff())
        .with_typed_function(time_parts())
}

fn time_now() -> StdFn {
    StdFn::typed("time_now", vec![(params(vec![]), T::Int)])
}
fn time_now_ms() -> StdFn {
    StdFn::typed("time_now_ms", vec![(params(vec![]), T::Int)])
}

fn format_time() -> StdFn {
    StdFn::typed(
        "format_time",
        vec![(params(vec![T::Int, T::String]), result(T::String))],
    )
}

fn timestamp_to_string(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Int]), result(T::String))])
}
fn format_date_str() -> StdFn {
    timestamp_to_string("format_date_str")
}
fn format_time_str() -> StdFn {
    timestamp_to_string("format_time_str")
}

fn int_int_to_int(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Int, T::Int]), T::Int)])
}
fn time_add() -> StdFn {
    int_int_to_int("time_add")
}
fn time_diff() -> StdFn {
    int_int_to_int("time_diff")
}

fn time_parts() -> StdFn {
    StdFn::typed(
        "time_parts",
        vec![(params(vec![T::Int]), result(T::Array(Box::new(T::Int))))],
    )
}
