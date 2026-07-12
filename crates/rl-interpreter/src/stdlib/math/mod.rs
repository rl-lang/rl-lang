//! `std::math` - mathematical functions and the `std::math::consts` submodule.
//!
//! Most functions accept both `int` and `float`; mixed types are handled per-function.
//! `pow` uses `with_raw_function` to handle `(int, int)`, `(int, float)`, `(float, float)`,
//! and `(float, int)` combinations manually.

mod abs;
mod acos;
mod asin;
mod atan;
mod atan2;
mod ceil;
mod clamp;
pub mod constants;
mod cos;
mod degrees;
mod exp;
mod factorial;
mod fibonacci;
mod floor;
mod gcd;
mod hypot;
mod is_prime;
mod lcm;
mod lerp;
mod log;
mod log10;
mod log2;
mod map_range;
mod max;
mod min;
mod modulo;
mod power;
mod radians;
mod round;
mod sign;
mod sin;
mod sqrt;
mod tan;

use crate::native::Module;

pub use rl_commons::keywords::math::KEYWORDS;

pub fn module() -> Module {
    Module::new("math")
        .with_function("sin", sin::std_sin)
        .with_function("cos", cos::std_cos)
        .with_function("tan", tan::std_tan)
        .with_raw_function("pow", power::std_pow)
        .with_function("mod", modulo::std_mod)
        .with_function("abs", abs::std_abs)
        .with_function("ceil", ceil::std_ceil)
        .with_function("clamp", clamp::std_clamp)
        .with_function("floor", floor::std_floor)
        .with_function("round", round::std_round)
        .with_function("log", log::std_log)
        .with_function("log2", log2::std_log2)
        .with_function("log10", log10::std_log10)
        .with_function("max", max::std_max)
        .with_function("min", min::std_min)
        .with_function("sqrt", sqrt::std_sqrt)
        .with_function("atan", atan::std_atan)
        .with_function("atan2", atan2::std_atan2)
        .with_function("acos", acos::std_acos)
        .with_function("asin", asin::std_asin)
        .with_function("degrees", degrees::std_degrees)
        .with_function("radians", radians::std_radians)
        .with_function("exp", exp::std_exp)
        .with_function("factorial", factorial::std_factorial)
        .with_function("fibonacci", fibonacci::std_fibonacci)
        .with_function("gcd", gcd::std_gcd)
        .with_function("lcm", lcm::std_lcm)
        .with_function("lerp", lerp::std_lerp)
        .with_function("is_prime", is_prime::std_is_prime)
        .with_function("hypot", hypot::std_hypot)
        .with_function("sign", sign::std_sign)
        .with_function("map_range", map_range::std_map_range)
        .with_module(constants::module())
}
