//! Typed signatures for `std::io`. Mirrors
//! `rl-interpreter/src/stdlib/io/*.rs`.
//!
//! `print`/`println` stay untyped: they accept a variadic list of
//! arguments of any type (`args.iter().map(|s| s.to_string())`), which the
//! checker's fixed-arity `TypeAnnotation` signatures can't express yet.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("io")
        .with_functions(&["print", "println"])
        .with_typed_function(read())
        .with_typed_function(read_int())
        .with_typed_function(read_float())
        .with_typed_function(read_file())
        .with_typed_function(read_lines())
        .with_typed_function(delete_file())
        .with_typed_function(write_file())
        .with_typed_function(append_file())
        .with_typed_function(eprint())
        .with_typed_function(read_bytes())
}

/// `read()` and `read(prompt)` both read a line from stdin (printing
/// `prompt` first, if given) and return it as-is - `prompt` is stringified
/// via `Value::to_string`, so any scalar type is accepted.
/// See `rl-interpreter/src/stdlib/io/input.rs::std_read`.
fn optional_prompt_overloads(return_inner: T) -> Vec<(T, T)> {
    let ret = result(return_inner);
    vec![
        (params(vec![]), ret.clone()),
        (params(vec![T::Int]), ret.clone()),
        (params(vec![T::Float]), ret.clone()),
        (params(vec![T::String]), ret.clone()),
        (params(vec![T::Bool]), ret.clone()),
        (params(vec![T::Char]), ret),
    ]
}

fn read() -> StdFn {
    StdFn::typed("read", optional_prompt_overloads(T::String))
}

/// Same optional-prompt overloads as `read`, but the line read from stdin
/// is then parsed as an `int` (or `Result::Err` if parsing fails).
fn read_int() -> StdFn {
    StdFn::typed("read_int", optional_prompt_overloads(T::Int))
}

/// Same optional-prompt overloads as `read`, but the line read from stdin
/// is then parsed as a `float` (or `Result::Err` if parsing fails).
fn read_float() -> StdFn {
    StdFn::typed("read_float", optional_prompt_overloads(T::Float))
}

fn read_file() -> StdFn {
    StdFn::typed(
        "read_file",
        vec![(params(vec![T::String]), result(T::String))],
    )
}

fn read_lines() -> StdFn {
    StdFn::typed(
        "read_lines",
        vec![(
            params(vec![T::String]),
            result(T::Array(Box::new(T::String))),
        )],
    )
}

fn delete_file() -> StdFn {
    StdFn::typed(
        "delete_file",
        vec![(params(vec![T::String]), result(T::Null))],
    )
}

fn write_file() -> StdFn {
    StdFn::typed(
        "write_file",
        vec![(params(vec![T::String, T::String]), result(T::Null))],
    )
}

fn append_file() -> StdFn {
    StdFn::typed(
        "append_file",
        vec![(params(vec![T::String, T::String]), result(T::Null))],
    )
}

/// `eprint` always raises a runtime error (it never yields a value back to
/// the caller) - see `rl-interpreter/src/stdlib/io/eprint.rs::std_eprint`.
/// `Null` is used as a placeholder return type since no expression can
/// meaningfully observe it.
fn eprint() -> StdFn {
    StdFn::typed("eprint", vec![(params(vec![T::String]), T::Null)])
}

fn read_bytes() -> StdFn {
    StdFn::typed(
        "read_bytes",
        vec![(params(vec![T::String]), result(T::Array(Box::new(T::Byte))))],
    )
}
