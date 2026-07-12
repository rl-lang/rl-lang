//! `std::result` - functions for working with `result[T]` values.

use crate::native::Module;

mod is_err;
mod is_ok;
mod result_map;
mod result_unwrap;

pub const KEYWORDS: &[&str] = &[
    "is_ok",
    "is_err",
    "result_unwrap",
    "result_unwrap_err",
    "result_unwrap_or",
    "result_map",
    "result_map_err",
];

pub fn module() -> Module {
    Module::new("res")
        .with_raw_function("is_ok", is_ok::func)
        .with_raw_function("is_err", is_err::func)
        .with_raw_function("result_unwrap", result_unwrap::std_unwrap)
        .with_raw_function("result_unwrap_err", result_unwrap::std_unwrap_err)
        .with_raw_function("result_unwrap_or", result_unwrap::std_unwrap_or)
        .with_raw_function("result_map", result_map::std_result_map)
        .with_raw_function("result_map_err", result_map::std_result_map_err)
}
