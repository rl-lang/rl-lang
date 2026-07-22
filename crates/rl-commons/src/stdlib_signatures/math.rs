use super::{params, result};
use crate::StdFn;
use rl_ast::statements::TypeAnnotation as T;

/// `pow(base, exponent) -> Result[int | float]`
///
/// Unlike most `std::math` functions (which coerce everything to `float`),
/// `pow` is handled with `with_raw_function` in `rl-interpreter` so it can
/// return `int` when both operands are `int`, and `float` for any
/// combination involving a `float` - four distinct overloads in total.
/// See `rl-interpreter/src/stdlib/math/power.rs`.
pub fn pow() -> StdFn {
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
