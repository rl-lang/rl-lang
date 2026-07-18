use crate::entry::{FnEntry, StdEntry};

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

pub static TYPES: StdEntry = StdEntry {
    name: "types",
    description: "functions for type checking and conversion",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &is_bool::IS_BOOL,
    &is_char::IS_CHAR,
    &is_float::IS_FLOAT,
    &is_int::IS_INT,
    &is_null::IS_NULL,
    &is_string::IS_STRING,
    &to_bin::TO_BIN,
    &to_bool::TO_BOOL,
    &to_char::TO_CHAR,
    &to_float::TO_FLOAT,
    &to_hex::TO_HEX,
    &to_int::TO_INT,
    &to_oct::TO_OCT,
    &to_string::TO_STRING,
];
