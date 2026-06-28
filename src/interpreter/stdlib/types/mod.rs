//! `std::types` - type inspection and conversion functions.
//!
//! `is_*` functions check the runtime type of a value without conversion.
//! `to_int` accepts hex strings prefixed with `0x`/`0X`.

mod bin;
mod bool;
mod char;
mod error;
mod float;
mod hex;
mod int;
mod null;
mod oct;
mod string;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "to_bin",
    "to_bool",
    "to_char",
    "to_float",
    "to_hex",
    "to_int",
    "to_oct",
    "to_string",
    "is_bool",
    "is_null",
    "is_char",
    "is_int",
    "is_string",
    "is_float",
    "is_error",
    "error_unwrap",
];

pub fn module() -> Module {
    Module::new("types")
        .with_function("to_bin", bin::func)
        .with_function("to_bool", bool::std_to_bool)
        .with_function("to_char", char::std_to_char)
        .with_function("to_float", float::std_to_float)
        .with_function("to_hex", hex::func)
        .with_function("to_int", int::std_to_int)
        .with_function("to_oct", oct::std_to_oct)
        .with_function("to_string", string::std_to_string)
        .with_function("is_bool", bool::std_is_bool)
        .with_function("is_null", null::std_is_null)
        .with_function("is_char", char::std_is_char)
        .with_function("is_int", int::std_is_int)
        .with_function("is_float", float::std_is_float)
        .with_function("is_string", string::std_is_string)
        .with_function("is_error", error::std_is_error)
        .with_function("error_unwrap", error::std_error_unwrap)
}
