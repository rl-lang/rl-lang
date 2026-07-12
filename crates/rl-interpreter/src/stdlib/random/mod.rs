//! `std::random` - random number generation using a custom Xoshiro256** PRNG.
//!
//! The PRNG state is stored on [`Evaluator::rng`] and seeded from the system clock
//! at startup. All random functions share this single instance.

mod rand_bool;
mod rand_bool_weighted;
mod rand_float;
mod rand_float_range;
mod rand_int;
mod rand_int_range;
mod random_general;
pub mod xoshiro;

use crate::native::Module;

pub use rl_commons::keywords::random::KEYWORDS;

pub fn module() -> Module {
    Module::new("random")
        .with_function("rand_int", rand_int::func)
        .with_function("rand_int_range", rand_int_range::func)
        .with_function("rand_float", rand_float::func)
        .with_function("rand_float_range", rand_float_range::func)
        .with_function("rand_bool_weighted", rand_bool_weighted::func)
        .with_function("rand_bool", rand_bool::func)
        .with_function("rand_dice", random_general::dice)
        .with_function("rand_range", random_general::range)
        .with_function("rand_range_step", random_general::range_step)
        .with_function("rand_choice", random_general::choice)
        .with_function("rand_choices", random_general::choices)
        .with_function("rand_sample", random_general::sample)
        .with_function("rand_shuffle", random_general::shuffle)
        .with_function("rand_byte", random_general::byte)
        .with_function("rand_bytes", random_general::bytes)
        .with_function("rand_char", random_general::char)
        .with_function("rand_string", random_general::string)
        .with_function("rand_dices", random_general::dices)
}
