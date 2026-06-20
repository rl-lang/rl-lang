pub mod xoshiro;

mod rand_float;
mod rand_float_range;
mod rand_int_range;
//mod rand_bool;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "rand_int_range",
    "rand_float",
    "rand_float_range",
    "rand_bool",
];

pub fn module() -> Module {
    Module::new("random")
        .with_function("rand_int_range", rand_int_range::func)
        .with_function("rand_float", rand_float::func)
        .with_function("rand_float_range", rand_float_range::func)
    //.with_function("rand_bool", rand_bool::func)
}
