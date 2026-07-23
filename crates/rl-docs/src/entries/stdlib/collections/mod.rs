use crate::entry::{FnEntry, StdEntry};

mod map_clear;
mod map_contains;
mod map_get;
mod map_is_empty;
mod map_keys;
mod map_len;
mod map_merge;
mod map_remove;
mod map_to_array;
mod map_values;
mod set_add;
mod set_contains;
mod set_is_empty;
mod set_len;
mod set_remove;
mod set_to_array;

use map_clear::MAP_CLEAR;
use map_contains::MAP_CONTAINS;
use map_get::MAP_GET;
use map_is_empty::MAP_IS_EMPTY;
use map_keys::MAP_KEYS;
use map_len::MAP_LEN;
use map_merge::MAP_MERGE;
use map_remove::MAP_REMOVE;
use map_to_array::MAP_TO_ARRAY;
use map_values::MAP_VALUES;
use set_add::SET_ADD;
use set_contains::SET_CONTAINS;
use set_is_empty::SET_IS_EMPTY;
use set_len::SET_LEN;
use set_remove::SET_REMOVE;
use set_to_array::SET_TO_ARRAY;

pub static COLLECTIONS: StdEntry = StdEntry {
    name: "collections",
    description: "functions for working with set[T] and map[K, V] collections - add, remove, membership, size, lookup, merging, and conversion to an array",
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
    &MAP_CONTAINS,
    &MAP_REMOVE,
    &MAP_LEN,
    &MAP_IS_EMPTY,
    &MAP_TO_ARRAY,
    &MAP_GET,
    &MAP_KEYS,
    &MAP_VALUES,
    &MAP_CLEAR,
    &MAP_MERGE,
];
