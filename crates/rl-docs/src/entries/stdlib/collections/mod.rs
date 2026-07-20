use crate::entry::{FnEntry, StdEntry};

mod set_add;
mod set_contains;
mod set_is_empty;
mod set_len;
mod set_remove;
mod set_to_array;

use set_add::SET_ADD;
use set_contains::SET_CONTAINS;
use set_is_empty::SET_IS_EMPTY;
use set_len::SET_LEN;
use set_remove::SET_REMOVE;
use set_to_array::SET_TO_ARRAY;

pub static COLLECTIONS: StdEntry = StdEntry {
    name: "collections",
    description: "functions for working with set[T] collections - add, remove, membership, size, and conversion to an array",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &SET_ADD,
    &SET_REMOVE,
    &SET_CONTAINS,
    &SET_LEN,
    &SET_IS_EMPTY,
    &SET_TO_ARRAY,
];
