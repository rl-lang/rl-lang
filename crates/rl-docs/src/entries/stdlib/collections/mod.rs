use crate::entry::{FnEntry, StdEntry};

mod bisect_left;
mod bisect_right;
mod deque_pop_front;
mod deque_push_front;
mod heap_peek;
mod heap_pop;
mod heap_push;
mod map_clear;
mod map_contains;
mod map_get;
mod map_get_or;
mod map_get_or_insert;
mod map_is_empty;
mod map_keys;
mod map_len;
mod map_merge;
mod map_remove;
mod map_to_array;
mod map_values;
mod set_add;
mod set_contains;
mod set_difference;
mod set_intersection;
mod set_is_empty;
mod set_is_subset;
mod set_is_superset;
mod set_len;
mod set_remove;
mod set_symmetric_difference;
mod set_to_array;
mod set_union;
mod sorted_insert;

use bisect_left::BISECT_LEFT;
use bisect_right::BISECT_RIGHT;
use deque_pop_front::DEQUE_POP_FRONT;
use deque_push_front::DEQUE_PUSH_FRONT;
use heap_peek::HEAP_PEEK;
use heap_pop::HEAP_POP;
use heap_push::HEAP_PUSH;
use map_clear::MAP_CLEAR;
use map_contains::MAP_CONTAINS;
use map_get::MAP_GET;
use map_get_or::MAP_GET_OR;
use map_get_or_insert::MAP_GET_OR_INSERT;
use map_is_empty::MAP_IS_EMPTY;
use map_keys::MAP_KEYS;
use map_len::MAP_LEN;
use map_merge::MAP_MERGE;
use map_remove::MAP_REMOVE;
use map_to_array::MAP_TO_ARRAY;
use map_values::MAP_VALUES;
use set_add::SET_ADD;
use set_contains::SET_CONTAINS;
use set_difference::SET_DIFFERENCE;
use set_intersection::SET_INTERSECTION;
use set_is_empty::SET_IS_EMPTY;
use set_is_subset::SET_IS_SUBSET;
use set_is_superset::SET_IS_SUPERSET;
use set_len::SET_LEN;
use set_remove::SET_REMOVE;
use set_symmetric_difference::SET_SYMMETRIC_DIFFERENCE;
use set_to_array::SET_TO_ARRAY;
use set_union::SET_UNION;
use sorted_insert::SORTED_INSERT;

pub static COLLECTIONS: StdEntry = StdEntry {
    name: "collections",
    description: "functions for working with set[T] and map[K, V] collections - add, remove, membership, size, lookup, merging, and conversion to an array",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &BISECT_LEFT,
    &BISECT_RIGHT,
    &DEQUE_POP_FRONT,
    &DEQUE_PUSH_FRONT,
    &HEAP_PEEK,
    &HEAP_POP,
    &HEAP_PUSH,
    &MAP_CLEAR,
    &MAP_CONTAINS,
    &MAP_GET,
    &MAP_GET_OR,
    &MAP_GET_OR_INSERT,
    &MAP_IS_EMPTY,
    &MAP_KEYS,
    &MAP_LEN,
    &MAP_MERGE,
    &MAP_REMOVE,
    &MAP_TO_ARRAY,
    &MAP_VALUES,
    &SET_ADD,
    &SET_CONTAINS,
    &SET_DIFFERENCE,
    &SET_INTERSECTION,
    &SET_IS_EMPTY,
    &SET_IS_SUBSET,
    &SET_IS_SUPERSET,
    &SET_LEN,
    &SET_REMOVE,
    &SET_SYMMETRIC_DIFFERENCE,
    &SET_TO_ARRAY,
    &SET_UNION,
    &SORTED_INSERT,
];
