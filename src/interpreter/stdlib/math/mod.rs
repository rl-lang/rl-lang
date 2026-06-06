pub mod abs;
pub mod ceil;
pub mod clamp;
pub mod cos;
pub mod floor;
pub mod log;
pub mod log10;
pub mod log2;
pub mod max;
pub mod min;
pub mod modulo;
pub mod power;
pub mod round;
pub mod sin;
pub mod sqrt;
pub mod tan;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["sin", "cos", "tan", "pow", "mod"];

pub fn module() -> Module {
    Module::new("math")
        .with_function("sin", sin::std_sin)
        .with_function("cos", cos::std_cos)
        .with_function("tan", tan::std_tan)
        .with_function("pow", power::std_pow)
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
}
