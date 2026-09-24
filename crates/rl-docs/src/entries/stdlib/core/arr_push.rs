use crate::entry::FnEntry;

pub static ARR_PUSH: FnEntry = FnEntry {
    signature: "__arr_push(arr, val)",
    description: "intrinsic: returns the array with val appended. copy-on-write like the rest of the runtime",
    example: r#"get __arr_new, __arr_push from core

dec a = __arr_push(__arr_new(), 1)"#,
    expected_output: None,
    returns: "array[T]",
    errors: None,
    see_also: &["__arr_new", "__arr_get", "__arr_set"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
