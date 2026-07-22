//! Typed signatures for `std::bitwise`

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("bitwise")
        .with_typed_function(bit_and())
        .with_typed_function(bit_or())
        .with_typed_function(bit_xor())
        .with_typed_function(bit_not())
        .with_typed_function(bit_shift_left())
        .with_typed_function(bit_shift_right())
        .with_typed_function(count_bits())
        .with_typed_function(leading_zeros())
        .with_typed_function(trailing_zeros())
}

/// Mixed `byte`/`int` operands widen to `int`.
fn bit_and() -> StdFn {
    StdFn::typed(
        "bit_and",
        vec![
            (params(vec![T::Byte, T::Byte]), result(T::Byte)),
            (params(vec![T::Int, T::Int]), result(T::Int)),
            (params(vec![T::Byte, T::Int]), result(T::Int)),
            (params(vec![T::Int, T::Byte]), result(T::Int)),
        ],
    )
}

/// Mixed `byte`/`int` operands widen to `int`.
fn bit_or() -> StdFn {
    StdFn::typed(
        "bit_or",
        vec![
            (params(vec![T::Byte, T::Byte]), result(T::Byte)),
            (params(vec![T::Int, T::Int]), result(T::Int)),
            (params(vec![T::Byte, T::Int]), result(T::Int)),
            (params(vec![T::Int, T::Byte]), result(T::Int)),
        ],
    )
}

/// Unlike `bit_and`/`bit_or`, `bit_xor` requires matching operand types -
/// no mixed `byte`/`int` overload.
fn bit_xor() -> StdFn {
    StdFn::typed(
        "bit_xor",
        vec![
            (params(vec![T::Byte, T::Byte]), result(T::Byte)),
            (params(vec![T::Int, T::Int]), result(T::Int)),
        ],
    )
}

fn bit_not() -> StdFn {
    StdFn::typed(
        "bit_not",
        vec![
            (params(vec![T::Byte]), result(T::Byte)),
            (params(vec![T::Int]), result(T::Int)),
        ],
    )
}

/// The shift amount may independently be `byte` or `int`; the result type
/// tracks the shifted value, not the shift amount.
fn bit_shift_left() -> StdFn {
    StdFn::typed(
        "bit_shift_left",
        vec![
            (params(vec![T::Byte, T::Byte]), result(T::Byte)),
            (params(vec![T::Byte, T::Int]), result(T::Byte)),
            (params(vec![T::Int, T::Byte]), result(T::Int)),
            (params(vec![T::Int, T::Int]), result(T::Int)),
        ],
    )
}

/// The shift amount may independently be `byte` or `int`; the result type
/// tracks the shifted value, not the shift amount.
fn bit_shift_right() -> StdFn {
    StdFn::typed(
        "bit_shift_right",
        vec![
            (params(vec![T::Byte, T::Byte]), result(T::Byte)),
            (params(vec![T::Byte, T::Int]), result(T::Byte)),
            (params(vec![T::Int, T::Byte]), result(T::Int)),
            (params(vec![T::Int, T::Int]), result(T::Int)),
        ],
    )
}

fn count_bits() -> StdFn {
    StdFn::typed(
        "count_bits",
        vec![
            (params(vec![T::Byte]), result(T::Byte)),
            (params(vec![T::Int]), result(T::Int)),
        ],
    )
}

fn leading_zeros() -> StdFn {
    StdFn::typed(
        "leading_zeros",
        vec![
            (params(vec![T::Byte]), result(T::Byte)),
            (params(vec![T::Int]), result(T::Int)),
        ],
    )
}

fn trailing_zeros() -> StdFn {
    StdFn::typed(
        "trailing_zeros",
        vec![
            (params(vec![T::Byte]), result(T::Byte)),
            (params(vec![T::Int]), result(T::Int)),
        ],
    )
}
