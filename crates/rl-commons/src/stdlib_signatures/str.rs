//! Typed signatures for `std::string`.

use super::{params, result, string_to_string};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("str")
        .with_functions(&["concat", "format"])
        .with_typed_function(to_lower())
        .with_typed_function(to_upper())
        .with_typed_function(trim())
        .with_typed_function(trim_end())
        .with_typed_function(trim_start())
        .with_typed_function(reverse())
        .with_typed_function(repeat())
        .with_typed_function(is_empty())
        .with_typed_function(char_at())
        .with_typed_function(bytes())
        .with_typed_function(chars())
        .with_typed_function(slice())
        .with_typed_function(contains())
        .with_typed_function(starts_with())
        .with_typed_function(ends_with())
        .with_typed_function(replace())
        .with_typed_function(pad_left())
        .with_typed_function(pad_right())
        .with_typed_function(split())
        .with_typed_function(join())
        .with_typed_function(count())
        .with_typed_function(index_of())
}

fn to_lower() -> StdFn {
    string_to_string("to_lower")
}
fn to_upper() -> StdFn {
    string_to_string("to_upper")
}
fn trim() -> StdFn {
    string_to_string("trim")
}
fn trim_end() -> StdFn {
    string_to_string("trim_end")
}
fn trim_start() -> StdFn {
    string_to_string("trim_start")
}
fn reverse() -> StdFn {
    string_to_string("reverse")
}

fn repeat() -> StdFn {
    StdFn::typed("repeat", vec![(params(vec![T::String, T::Int]), T::String)])
}
fn is_empty() -> StdFn {
    StdFn::typed("is_empty", vec![(params(vec![T::String]), T::Bool)])
}

fn char_at() -> StdFn {
    StdFn::typed(
        "char_at",
        vec![(params(vec![T::String, T::Int]), result(T::Char))],
    )
}
fn bytes() -> StdFn {
    StdFn::typed(
        "bytes",
        vec![(params(vec![T::String]), T::Array(Box::new(T::Byte)))],
    )
}
fn chars() -> StdFn {
    StdFn::typed(
        "chars",
        vec![(params(vec![T::String]), T::Array(Box::new(T::Char)))],
    )
}
fn slice() -> StdFn {
    StdFn::typed(
        "slice",
        vec![(params(vec![T::String, T::Int, T::Int]), result(T::String))],
    )
}

fn string_predicate(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String, T::String]), T::Bool)])
}
fn contains() -> StdFn {
    string_predicate("contains")
}
fn starts_with() -> StdFn {
    string_predicate("starts_with")
}
fn ends_with() -> StdFn {
    string_predicate("ends_with")
}

fn replace() -> StdFn {
    StdFn::typed(
        "replace",
        vec![(params(vec![T::String, T::String, T::String]), T::String)],
    )
}

fn pad(name: &'static str) -> StdFn {
    StdFn::typed(
        name,
        vec![(params(vec![T::String, T::Int, T::Char]), T::String)],
    )
}
fn pad_left() -> StdFn {
    pad("pad_left")
}
fn pad_right() -> StdFn {
    pad("pad_right")
}

fn split() -> StdFn {
    StdFn::typed(
        "split",
        vec![(
            params(vec![T::String, T::String]),
            T::Array(Box::new(T::String)),
        )],
    )
}

fn join() -> StdFn {
    StdFn::typed(
        "join",
        vec![(
            params(vec![T::Array(Box::new(T::Generic("_".into()))), T::String]),
            result(T::String),
        )],
    )
}

fn string_to_int(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String, T::String]), T::Int)])
}
fn count() -> StdFn {
    string_to_int("count")
}
fn index_of() -> StdFn {
    string_to_int("index_of")
}
