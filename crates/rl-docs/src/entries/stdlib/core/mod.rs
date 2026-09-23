use crate::entry::{FnEntry, StdEntry};

mod abort;
mod arr_get;
mod arr_new;
mod arr_push;
mod arr_set;
mod map_get;
mod map_keys;
mod map_new;
mod map_set;
mod set_add;
mod set_has;
mod set_new;
mod type_of;

pub static CORE: StdEntry = StdEntry {
    name: "core",
    description: "compiler intrinsics for self-hosting: the primitive floor RL-written stdlib builds on",
    functions: FUNCTIONS,
    since: Some("v2.2.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &arr_new::ARR_NEW,
    &arr_push::ARR_PUSH,
    &arr_get::ARR_GET,
    &arr_set::ARR_SET,
    &map_new::MAP_NEW,
    &map_get::MAP_GET,
    &map_set::MAP_SET,
    &map_keys::MAP_KEYS,
    &set_new::SET_NEW,
    &set_add::SET_ADD,
    &set_has::SET_HAS,
    &abort::ABORT,
    &type_of::TYPE_OF,
];
