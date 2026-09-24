use crate::entry::{FnEntry, StdEntry};

mod is_err;
mod is_ok;
mod result_and_then;
mod result_map;
mod result_map_err;
mod result_unwrap;
mod result_unwrap_err;
mod result_unwrap_or;
mod result_unwrap_or_else;

pub static RES: StdEntry = StdEntry {
    name: "res",
    description: "functions for working with result[T] values (ok / err)",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &is_err::IS_ERR,
    &is_ok::IS_OK,
    &result_and_then::RESULT_AND_THEN,
    &result_map::RESULT_MAP,
    &result_map_err::RESULT_MAP_ERR,
    &result_unwrap::RESULT_UNWRAP,
    &result_unwrap_err::RESULT_UNWRAP_ERR,
    &result_unwrap_or::RESULT_UNWRAP_OR,
    &result_unwrap_or_else::RESULT_UNWRAP_OR_ELSE,
];
