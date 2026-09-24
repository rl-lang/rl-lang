use crate::entry::FnEntry;

pub static ARR_NEW: FnEntry = FnEntry {
    signature: "__arr_new()",
    description: "intrinsic: builds an empty array. the primitive behind array literals; RL-written containers start here",
    example: r#"get __arr_new, __arr_push from core

dec a = __arr_new()
__arr_push(a, 1)"#,
    expected_output: None,
    returns: "array[T]",
    errors: None,
    see_also: &["__arr_push", "__arr_get", "__arr_set"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
