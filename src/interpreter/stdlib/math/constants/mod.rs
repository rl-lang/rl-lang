pub mod e;
pub mod euler_gamma;
pub mod frac_1_pi;
pub mod frac_1_sqrt_2;
pub mod frac_2_pi;
pub mod frac_2_sqrt_pi;
pub mod frac_pi_2;
pub mod frac_pi_3;
pub mod frac_pi_4;
pub mod frac_pi_6;
pub mod frac_pi_8;
pub mod inf;
pub mod is_inf;
pub mod is_nan;
pub mod ln_10;
pub mod ln_2;
pub mod log10_2;
pub mod log10_e;
pub mod log2_10;
pub mod log2_e;
pub mod nan;
pub mod phi;
pub mod pi;
pub mod sqrt_2;
pub mod tau;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "E",
    "PI",
    "FRAC_1_PI",
    "FRAC_1_SQRT_2",
    "FRAC_2_PI",
    "FRAC_2_SQRT_PI",
    "FRAC_PI_2",
    "FRAC_PI_3",
    "FRAC_PI_4",
    "FRAC_PI_6",
    "FRAC_PI_8",
    "INF",
    "NAN",
    "is_inf",
    "is_nan",
    "LN_10",
    "LN_2",
    "LOG10_2",
    "LOG10_E",
    "LOG2_10",
    "LOG2_E",
    "SQRT_2",
    "EULER_GAMMA",
    "PHI",
    "TAU",
];

pub fn module() -> Module {
    Module::new("consts")
        .with_function("E", e::std_e)
        .with_function("PI", pi::std_pi)
        .with_function("PHI", phi::std_phi)
        .with_function("TAU", tau::std_tau)
        .with_function("INF", inf::std_inf)
        .with_function("NAN", nan::std_nan)
        .with_function("is_inf", is_inf::std_is_inf)
        .with_function("is_nan", is_nan::std_is_nan)
        .with_function("FRAC_1_PI", frac_1_pi::std_frac_1_pi)
        .with_function("FRAC_2_PI", frac_2_pi::std_frac_2_pi)
        .with_function("FRAC_1_SQRT_2", frac_1_sqrt_2::std_frac_1_sqrt_2)
        .with_function("FRAC_2_SQRT_PI", frac_2_sqrt_pi::std_frac_2_sqrt_pi)
        .with_function("EULER_GAMMA", euler_gamma::std_euler_gamma)
        .with_function("LN_10", ln_10::std_ln_10)
        .with_function("LN_2", ln_2::std_ln_2)
        .with_function("LOG2_E", log2_e::std_log2_e)
        .with_function("LOG2_10", log2_10::std_log2_10)
        .with_function("LOG10_2", log10_2::std_log10_2)
        .with_function("LOG10_E", log10_e::std_log10_e)
        .with_function("FRAC_PI_2", frac_pi_2::std_frac_pi_2)
        .with_function("FRAC_PI_3", frac_pi_3::std_frac_pi_3)
        .with_function("FRAC_PI_4", frac_pi_4::std_frac_pi_4)
        .with_function("FRAC_PI_6", frac_pi_6::std_frac_pi_6)
        .with_function("FRAC_PI_8", frac_pi_8::std_frac_pi_8)
        .with_function("SQRT_2", sqrt_2::std_sqrt_2)
}
