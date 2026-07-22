//! Typed signatures for `std::math` and `std::math::constants`. Mirrors
//! `rl-interpreter/src/stdlib/math/*.rs`.

use super::{constants, params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("math")
        .with_typed_function(pow())
        .with_typed_function(same_type_unary_result("abs"))
        .with_typed_function(same_type_unary_result("ceil"))
        .with_typed_function(same_type_unary_result("floor"))
        .with_typed_function(same_type_unary_result("round"))
        .with_typed_function(clamp())
        .with_typed_function(same_type_binary_result("max"))
        .with_typed_function(same_type_binary_result("min"))
        .with_typed_function(modulo())
        .with_typed_function(log())
        .with_typed_function(float_result_unary("log10"))
        .with_typed_function(float_result_unary("log2"))
        .with_typed_function(float_result_unary("sqrt"))
        .with_typed_function(plain_unary_float("sin"))
        .with_typed_function(plain_unary_float("cos"))
        .with_typed_function(plain_unary_float("tan"))
        .with_typed_function(plain_unary_float("atan"))
        .with_typed_function(plain_unary_float("acos"))
        .with_typed_function(plain_unary_float("asin"))
        .with_typed_function(plain_unary_float("degrees"))
        .with_typed_function(plain_unary_float("radians"))
        .with_typed_function(plain_unary_float("exp"))
        .with_typed_function(plain_unary_float("sign"))
        .with_typed_function(atan2())
        .with_typed_function(plain_unary_int("factorial"))
        .with_typed_function(plain_unary_int("fibonacci"))
        .with_typed_function(is_prime())
        .with_typed_function(plain_binary_int("gcd"))
        .with_typed_function(plain_binary_int("lcm"))
        .with_typed_function(plain_binary_float("hypot"))
        .with_typed_function(lerp())
        .with_typed_function(map_range())
        .with_module(constants::module())
}

/// `pow(base, exponent) -> Result[int | float]`
///
/// Unlike most `std::math` functions (which coerce everything to `float`),
/// `pow` is handled with `with_raw_function` in `rl-interpreter` so it can
/// return `int` when both operands are `int`, and `float` for any
/// combination involving a `float` - four distinct overloads in total.
/// See `rl-interpreter/src/stdlib/math/power.rs`.
fn pow() -> StdFn {
    StdFn::typed(
        "pow",
        vec![
            (params(vec![T::Int, T::Int]), result(T::Int)),
            (params(vec![T::Int, T::Float]), result(T::Float)),
            (params(vec![T::Float, T::Float]), result(T::Float)),
            (params(vec![T::Float, T::Int]), result(T::Float)),
        ],
    )
}

/// Shared shape for `abs`/`ceil`/`floor`/`round`: one `int` or `float`
/// argument, and the result keeps that same type (`ceil`/`floor`/`round`
/// pass `int` straight through unchanged; `abs` and the float ops apply
/// the actual operation). See e.g. `math/abs.rs::std_abs`.
fn same_type_unary_result(name: &'static str) -> StdFn {
    StdFn::typed(
        name,
        vec![
            (params(vec![T::Int]), result(T::Int)),
            (params(vec![T::Float]), result(T::Float)),
        ],
    )
}

/// Shared shape for `max`/`min`: both arguments must be the same type
/// (`int, int` or `float, float` - no mixed overload), and the result
/// keeps that type. See e.g. `math/max.rs::std_max`.
fn same_type_binary_result(name: &'static str) -> StdFn {
    StdFn::typed(
        name,
        vec![
            (params(vec![T::Int, T::Int]), result(T::Int)),
            (params(vec![T::Float, T::Float]), result(T::Float)),
        ],
    )
}

/// `clamp(value, min, max) -> Result[int | float]` - all three arguments
/// must share the same type; no mixed overload. See
/// `math/clamp.rs::std_clamp`.
fn clamp() -> StdFn {
    StdFn::typed(
        "clamp",
        vec![
            (params(vec![T::Int, T::Int, T::Int]), result(T::Int)),
            (params(vec![T::Float, T::Float, T::Float]), result(T::Float)),
        ],
    )
}

/// `mod(a, b) -> Result[int | float]` - like `log`, all four `int`/`float`
/// combinations are accepted; the result is `int` only when both operands
/// are `int`, `float` otherwise. See `math/modulo.rs::std_mod`.
fn modulo() -> StdFn {
    StdFn::typed(
        "mod",
        vec![
            (params(vec![T::Int, T::Int]), result(T::Int)),
            (params(vec![T::Int, T::Float]), result(T::Float)),
            (params(vec![T::Float, T::Float]), result(T::Float)),
            (params(vec![T::Float, T::Int]), result(T::Float)),
        ],
    )
}

/// `log(a, base) -> Result[float]` - all four `int`/`float` combinations
/// are accepted, but unlike `pow`/`mod` the result is always `float`
/// (everything is computed via `f64::log`). See `math/log.rs::std_log`.
fn log() -> StdFn {
    StdFn::typed(
        "log",
        vec![
            (params(vec![T::Int, T::Int]), result(T::Float)),
            (params(vec![T::Int, T::Float]), result(T::Float)),
            (params(vec![T::Float, T::Float]), result(T::Float)),
            (params(vec![T::Float, T::Int]), result(T::Float)),
        ],
    )
}

/// Shared shape for `log10`/`log2`/`sqrt`: accepts `int` or `float`, but
/// always returns `float` (the `int` case is cast to `f64` before the
/// operation). See e.g. `math/sqrt.rs::std_sqrt`.
fn float_result_unary(name: &'static str) -> StdFn {
    StdFn::typed(
        name,
        vec![
            (params(vec![T::Int]), result(T::Float)),
            (params(vec![T::Float]), result(T::Float)),
        ],
    )
}

/// Shared shape for the trig/exp-family functions (`sin`, `cos`, `tan`,
/// `atan`, `acos`, `asin`, `degrees`, `radians`, `exp`, `sign`): a single
/// `float` argument via `FromValue for f64` (no `int` coercion), returning
/// `float` directly - not wrapped in `Result`, since there's no failure
/// path once the argument type has already been checked. See e.g.
/// `math/sin.rs::std_sin`.
fn plain_unary_float(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Float]), T::Float)])
}

/// `atan2(x, y) -> float` - both `float`, no `Result` wrapper (same
/// reasoning as `plain_unary_float`). See `math/atan2.rs::std_atan2`.
fn atan2() -> StdFn {
    StdFn::typed("atan2", vec![(params(vec![T::Float, T::Float]), T::Float)])
}

/// Shared shape for `factorial`/`fibonacci`: a single `int` argument,
/// returning `int` directly (no `Result` wrapper). See e.g.
/// `math/factorial.rs::std_factorial`.
fn plain_unary_int(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Int]), T::Int)])
}

/// `is_prime(x) -> bool` - no `Result` wrapper. See
/// `math/is_prime.rs::std_is_prime`.
fn is_prime() -> StdFn {
    StdFn::typed("is_prime", vec![(params(vec![T::Int]), T::Bool)])
}

/// Shared shape for `gcd`/`lcm`: two `int` arguments, returning `int`
/// directly (no `Result` wrapper). See e.g. `math/gcd.rs::std_gcd`.
fn plain_binary_int(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Int, T::Int]), T::Int)])
}

/// `hypot(x, y) -> float` - both `float`, no `Result` wrapper. See
/// `math/hypot.rs::std_hypot`.
fn plain_binary_float(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Float, T::Float]), T::Float)])
}

/// `lerp(x, y, t) -> float` - all `float`, no `Result` wrapper. See
/// `math/lerp.rs::std_lerp`.
fn lerp() -> StdFn {
    StdFn::typed(
        "lerp",
        vec![(params(vec![T::Float, T::Float, T::Float]), T::Float)],
    )
}

/// `map_range(value, in_min, out_min, in_max, out_max) -> float` - all
/// five arguments `float`, no `Result` wrapper (order matches the Rust
/// signature, not the more common `in_min, in_max, out_min, out_max`).
/// See `math/map_range.rs::std_map_range`.
fn map_range() -> StdFn {
    StdFn::typed(
        "map_range",
        vec![(
            params(vec![T::Float, T::Float, T::Float, T::Float, T::Float]),
            T::Float,
        )],
    )
}
