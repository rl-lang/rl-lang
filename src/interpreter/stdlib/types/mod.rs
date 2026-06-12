mod is_bool;
mod is_char;
mod is_float;
mod is_int;
mod is_null;
mod is_string;
mod to_bin;
mod to_bool;
mod to_char;
mod to_float;
mod to_hex;
mod to_int;
mod to_oct;
mod to_string;

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
];

pub fn module() -> Module {
    Module::new("types")
        .with_function("to_bin", to_bin::std_to_bin)
        .with_function("to_bool", to_bool::std_to_bool)
        .with_function("to_char", to_char::std_to_char)
        .with_function("to_float", to_float::std_to_float)
        .with_function("to_hex", to_hex::std_to_hex)
        .with_function("to_int", to_int::std_to_int)
        .with_function("to_oct", to_oct::std_to_oct)
        .with_function("to_string", to_string::std_to_string)
        .with_function("is_bool", is_bool::std_is_bool)
        .with_function("is_null", is_null::std_is_null)
        .with_function("is_char", is_char::std_is_char)
        .with_function("is_int", is_int::std_is_int)
        .with_function("is_float", is_float::std_is_float)
        .with_function("is_string", is_string::std_is_string)
}
