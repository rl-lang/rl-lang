use crate::entry::{FnEntry, StdEntry};

mod abs;
mod acos;
mod asin;
mod atan;
mod atan2;
mod ceil;
mod clamp;
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

pub static MATH: StdEntry = StdEntry {
    name: "math",
    description: "functions for math",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &abs::ABS,
    &acos::ACOS,
    &asin::ASIN,
    &atan::ATAN,
    &atan2::ATAN2,
    &ceil::CEIL,
    &clamp::CLAMP,
    &cos::COS,
    &degrees::DEGREES,
    &exp::EXP,
    &factorial::FACTORIAL,
    &fibonacci::FIBONACCI,
    &floor::FLOOR,
    &gcd::GCD,
    &hypot::HYPOT,
    &is_prime::IS_PRIME,
    &lcm::LCM,
    &lerp::LERP,
    &log::LOG,
    &log2::LOG2,
    &log10::LOG10,
    &map_range::MAP_RANGE,
    &max::MAX,
    &min::MIN,
    &modulo::MOD,
    &power::POW,
    &radians::RADIANS,
    &round::ROUND,
    &sign::SIGN,
    &sin::SIN,
    &sqrt::SQRT,
    &tan::TAN,
];
