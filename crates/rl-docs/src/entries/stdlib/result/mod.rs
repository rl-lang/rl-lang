use crate::entry::{FnEntry, StdEntry};

mod is_err;
mod is_ok;
mod result_map;
mod result_map_err;
mod result_unwrap;
mod result_unwrap_err;
mod result_unwrap_or;

pub static RES: StdEntry = StdEntry {
    name: "res",
    description: "functions for working with result[T] values (ok / err)",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &is_ok::IS_OK,
    &is_err::IS_ERR,
    &result_unwrap::RESULT_UNWRAP,
    &result_unwrap_err::RESULT_UNWRAP_ERR,
    &result_unwrap_or::RESULT_UNWRAP_OR,
    &result_map::RESULT_MAP,
    &result_map_err::RESULT_MAP_ERR,
];
