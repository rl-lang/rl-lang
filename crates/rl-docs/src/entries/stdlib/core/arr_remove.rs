use crate::entry::FnEntry;

pub static ARR_REMOVE: FnEntry = FnEntry {
    signature: "__arr_remove(arr, idx)",
    description: "intrinsic: returns the array without the element at idx. out-of-bounds indexes abort",
    example: r#"get __arr_remove from core

dec a = __arr_remove([10, 20, 30], 1)"#,
    expected_output: None,
    returns: "array[T]",
    errors: Some("index out of bounds aborts"),
    see_also: &["__arr_get", "__arr_set", "__arr_push"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
