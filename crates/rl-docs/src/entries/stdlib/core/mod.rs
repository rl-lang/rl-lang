use crate::entry::{FnEntry, StdEntry};

mod abort;
mod arr_get;
mod arr_len;
mod arr_new;
mod arr_push;
mod arr_remove;
mod arr_set;
mod map_get;
mod map_has;
mod map_keys;
mod map_len;
mod map_new;
mod map_remove;
mod map_set;
mod result_err_value;
mod result_ok_value;
mod set_add;
mod set_has;
mod set_len;
mod set_new;
mod set_remove;
mod str_concat;
mod str_get_byte;
mod str_len;
mod str_slice;
mod buf_new;
mod buf_len;
mod buf_push_byte;
mod buf_get_byte;
mod buf_set_byte;
mod buf_append;
mod buf_slice;
mod buf_clear;
mod buf_to_string;
mod buf_free;
mod buf_addr;
mod buf_resize;
mod syscall6;
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
    &arr_remove::ARR_REMOVE,
    &arr_len::ARR_LEN,
    &map_new::MAP_NEW,
    &map_get::MAP_GET,
    &map_has::MAP_HAS,
    &map_set::MAP_SET,
    &map_remove::MAP_REMOVE,
    &map_keys::MAP_KEYS,
    &map_len::MAP_LEN,
    &set_new::SET_NEW,
    &set_add::SET_ADD,
    &set_has::SET_HAS,
    &set_remove::SET_REMOVE,
    &set_len::SET_LEN,
    &str_len::STR_LEN,
    &str_get_byte::STR_GET_BYTE,
    &str_slice::STR_SLICE,
    &str_concat::STR_CONCAT,
    &buf_new::BUF_NEW,
    &buf_len::BUF_LEN,
    &buf_push_byte::BUF_PUSH_BYTE,
    &buf_get_byte::BUF_GET_BYTE,
    &buf_set_byte::BUF_SET_BYTE,
    &buf_append::BUF_APPEND,
    &buf_slice::BUF_SLICE,
    &buf_clear::BUF_CLEAR,
    &buf_to_string::BUF_TO_STRING,
    &buf_free::BUF_FREE,
    &buf_addr::BUF_ADDR,
    &buf_resize::BUF_RESIZE,
    &syscall6::SYSCALL6,
    &abort::ABORT,
    &type_of::TYPE_OF,
    &result_ok_value::RESULT_OK_VALUE,
    &result_err_value::RESULT_ERR_VALUE,
];
