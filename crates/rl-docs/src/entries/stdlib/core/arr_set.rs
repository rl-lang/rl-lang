use crate::entry::FnEntry;

pub static ARR_SET: FnEntry = FnEntry {
    signature: "__arr_set(arr, idx, val)",
    description: "intrinsic: returns the array with element idx replaced. out-of-bounds indexes abort",
    example: r#"get __arr_set from core

dec a = __arr_set([10, 20], 0, 99)"#,
    expected_output: None,
    returns: "array[T]",
    errors: Some("index out of bounds aborts"),
    see_also: &["__arr_get", "__arr_push"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
