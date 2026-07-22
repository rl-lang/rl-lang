//! Typed signatures for `std::types`. Mirrors
//! `rl-interpreter/src/stdlib/types/*.rs`.
//!
//! `is_bool`/`is_null`/`is_char`/`is_int`/`is_float`/`is_string`/`is_byte`/
//! `is_error` all stay untyped until we have generic to track it

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("types")
        .with_functions(&[
            "is_bool",
            "is_null",
            "is_char",
            "is_int",
            "is_float",
            "is_string",
            "is_byte",
            "is_error",
            "error_unwrap",
        ])
        .with_typed_function(to_bin())
        .with_typed_function(to_bool())
        .with_typed_function(to_byte())
        .with_typed_function(to_char())
        .with_typed_function(to_float())
        .with_typed_function(to_hex())
        .with_typed_function(to_int())
        .with_typed_function(to_oct())
        .with_typed_function(to_string())
}

/// Builds one `(single_arg) -> Result[ret]` overload per accepted input
/// type.
fn unary_overloads(name: &'static str, accepted: &[T], ret: T) -> StdFn {
    StdFn::typed(
        name,
        accepted
            .iter()
            .map(|t| (params(vec![t.clone()]), result(ret.clone())))
            .collect(),
    )
}

/// `to_bin(v) -> Result[string]` - `byte`/`int`/`bool`/`char`/`string`
/// only; `float` falls through to the error branch. See
/// `types/bin.rs::func`.
fn to_bin() -> StdFn {
    unary_overloads(
        "to_bin",
        &[T::Byte, T::Int, T::Bool, T::Char, T::String],
        T::String,
    )
}

/// `to_bool(v) -> Result[bool]` - `bool`/`int`/`byte`/`float`/`null`/
/// `string`; `char` falls through to the error branch. See
/// `types/bool.rs::std_to_bool`.
fn to_bool() -> StdFn {
    unary_overloads(
        "to_bool",
        &[T::Bool, T::Int, T::Byte, T::Float, T::Null, T::String],
        T::Bool,
    )
}

/// `to_byte(v) -> Result[byte]` - `int`/`byte`/`float`/`bool`/`char`/
/// `string`; `null` falls through to the error branch. See
/// `types/byte.rs::std_to_byte`.
fn to_byte() -> StdFn {
    unary_overloads(
        "to_byte",
        &[T::Int, T::Byte, T::Float, T::Bool, T::Char, T::String],
        T::Byte,
    )
}

/// `to_char(v) -> Result[char]` - `char`/`int`/`byte`/`string` (a
/// one-character string only, checked at runtime); `bool`/`float`/`null`
/// fall through to the error branch. See `types/char.rs::std_to_char`.
fn to_char() -> StdFn {
    unary_overloads("to_char", &[T::Char, T::Int, T::Byte, T::String], T::Char)
}

/// `to_float(v) -> Result[float]` - `float`/`int`/`byte`/`bool`/`string`;
/// `char`/`null` fall through to the error branch. See
/// `types/float.rs::std_to_float`.
fn to_float() -> StdFn {
    unary_overloads(
        "to_float",
        &[T::Float, T::Int, T::Byte, T::Bool, T::String],
        T::Float,
    )
}

/// `to_hex(v) -> Result[string]` - `int`/`byte`/`char`/`string`;
/// `bool`/`float`/`null` fall through to the error branch. See
/// `types/hex.rs::func`.
fn to_hex() -> StdFn {
    unary_overloads("to_hex", &[T::Int, T::Byte, T::Char, T::String], T::String)
}

/// `to_int(v) -> Result[int]` - `int`/`byte`/`float`/`bool`/`char`/
/// `string` (hex strings prefixed `0x`/`0X` accepted); `null` falls
/// through to the error branch. See `types/int.rs::std_to_int`.
fn to_int() -> StdFn {
    unary_overloads(
        "to_int",
        &[T::Int, T::Byte, T::Float, T::Bool, T::Char, T::String],
        T::Int,
    )
}

/// `to_oct(v) -> Result[string]` - same accepted set as `to_hex`. See
/// `types/oct.rs::func`.
fn to_oct() -> StdFn {
    unary_overloads("to_oct", &[T::Int, T::Byte, T::Char, T::String], T::String)
}

/// `to_string(v) -> Result[string]` - `int`/`byte`/`float`/`bool`/`char`/
/// `string`; `null` falls through to the error branch (despite `Value`
/// having a general-purpose `to_string()` used elsewhere for
/// stringification, e.g. `print`). See `types/string.rs::std_to_string`.
fn to_string() -> StdFn {
    unary_overloads(
        "to_string",
        &[T::Int, T::Byte, T::Float, T::Bool, T::Char, T::String],
        T::String,
    )
}
