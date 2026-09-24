use crate::entry::{FnEntry, StdEntry};

mod is_array;
mod is_audio_handle;
mod is_bbyte;
mod is_bool;
mod is_bsbyte;
mod is_byte;
mod is_char;
mod is_c_handle;
mod is_error;
mod is_file_handle;
mod is_float;
mod is_function;
mod is_gui_handle;
mod is_http_handle;
mod is_int;
mod is_map;
mod is_net_handle;
mod is_null;
mod is_sbyte;
mod is_set;
mod is_sfloat;
mod is_sint;
mod is_suint;
mod is_string;
mod is_tuple;
mod is_uint;
mod to_bin;
mod to_bool;
mod to_byte;
mod to_char;
mod to_float;
mod to_hex;
mod to_int;
mod to_oct;
mod to_string;
mod unwrap_error;

pub static TYPES: StdEntry = StdEntry {
    name: "types",
    description: "functions for type checking and conversion",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &is_array::IS_ARRAY,
    &is_audio_handle::IS_AUDIO_HANDLE,
    &is_bbyte::IS_BBYTE,
    &is_bool::IS_BOOL,
    &is_bsbyte::IS_BSBYTE,
    &is_byte::IS_BYTE,
    &is_char::IS_CHAR,
    &is_c_handle::IS_C_HANDLE,
    &is_error::IS_ERROR,
    &is_file_handle::IS_FILE_HANDLE,
    &is_float::IS_FLOAT,
    &is_function::IS_FUNCTION,
    &is_gui_handle::IS_GUI_HANDLE,
    &is_http_handle::IS_HTTP_HANDLE,
    &is_int::IS_INT,
    &is_map::IS_MAP,
    &is_net_handle::IS_NET_HANDLE,
    &is_null::IS_NULL,
    &is_sbyte::IS_SBYTE,
    &is_set::IS_SET,
    &is_sfloat::IS_SFLOAT,
    &is_sint::IS_SINT,
    &is_suint::IS_SUINT,
    &is_string::IS_STRING,
    &is_tuple::IS_TUPLE,
    &is_uint::IS_UINT,
    &to_bin::TO_BIN,
    &to_bool::TO_BOOL,
    &to_byte::TO_BYTE,
    &to_char::TO_CHAR,
    &to_float::TO_FLOAT,
    &to_hex::TO_HEX,
    &to_int::TO_INT,
    &to_oct::TO_OCT,
    &to_string::TO_STRING,
    &unwrap_error::UNWRAP_ERROR,
];
