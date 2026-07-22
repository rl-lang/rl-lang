//! Typed signatures for `std::random`.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("random")
        .with_typed_function(rand_int())
        .with_typed_function(rand_int_range())
        .with_typed_function(rand_float())
        .with_typed_function(rand_float_range())
        .with_typed_function(rand_bool())
        .with_typed_function(rand_bool_weighted())
        .with_typed_function(rand_dice())
        .with_typed_function(rand_dices())
        .with_typed_function(rand_range())
        .with_typed_function(rand_range_step())
        .with_typed_function(rand_choice())
        .with_typed_function(rand_choices())
        .with_typed_function(rand_sample())
        .with_typed_function(rand_shuffle())
        .with_typed_function(rand_byte())
        .with_typed_function(rand_bytes())
        .with_typed_function(rand_char())
        .with_typed_function(rand_string())
}

fn t() -> T {
    T::Generic("T".into())
}
fn arr_t() -> T {
    T::Array(Box::new(t()))
}

fn rand_int() -> StdFn {
    StdFn::typed("rand_int", vec![(params(vec![]), T::Int)])
}

fn rand_int_range() -> StdFn {
    StdFn::typed(
        "rand_int_range",
        vec![(params(vec![T::Int, T::Int]), result(T::Int))],
    )
}

fn rand_float() -> StdFn {
    StdFn::typed("rand_float", vec![(params(vec![]), T::Float)])
}

fn rand_float_range() -> StdFn {
    StdFn::typed(
        "rand_float_range",
        vec![(params(vec![T::Float, T::Float]), result(T::Float))],
    )
}

fn rand_bool() -> StdFn {
    StdFn::typed("rand_bool", vec![(params(vec![]), T::Bool)])
}

fn rand_bool_weighted() -> StdFn {
    StdFn::typed(
        "rand_bool_weighted",
        vec![(params(vec![T::Float]), T::Bool)],
    )
}

fn rand_dice() -> StdFn {
    StdFn::typed("rand_dice", vec![(params(vec![T::Int]), result(T::Int))])
}

fn rand_dices() -> StdFn {
    StdFn::typed(
        "rand_dices",
        vec![(
            params(vec![T::Int, T::Int]),
            result(T::Array(Box::new(T::Int))),
        )],
    )
}

fn rand_range() -> StdFn {
    StdFn::typed("rand_range", vec![(params(vec![T::Int]), result(T::Int))])
}

fn rand_range_step() -> StdFn {
    StdFn::typed(
        "rand_range_step",
        vec![(params(vec![T::Int, T::Int, T::Int]), result(T::Int))],
    )
}

fn rand_choice() -> StdFn {
    StdFn::typed("rand_choice", vec![(params(vec![arr_t()]), result(t()))])
}

fn rand_choices() -> StdFn {
    StdFn::typed(
        "rand_choices",
        vec![(params(vec![arr_t(), T::Int]), result(arr_t()))],
    )
}

fn rand_sample() -> StdFn {
    StdFn::typed(
        "rand_sample",
        vec![(params(vec![arr_t(), T::Int]), result(arr_t()))],
    )
}

fn rand_shuffle() -> StdFn {
    StdFn::typed(
        "rand_shuffle",
        vec![(params(vec![arr_t()]), result(arr_t()))],
    )
}

fn rand_byte() -> StdFn {
    StdFn::typed("rand_byte", vec![(params(vec![]), T::Byte)])
}

fn rand_bytes() -> StdFn {
    StdFn::typed(
        "rand_bytes",
        vec![(params(vec![T::Int]), result(T::Array(Box::new(T::Byte))))],
    )
}

fn rand_char() -> StdFn {
    StdFn::typed("rand_char", vec![(params(vec![]), T::Char)])
}

fn rand_string() -> StdFn {
    StdFn::typed(
        "rand_string",
        vec![(params(vec![T::Int]), result(T::String))],
    )
}
