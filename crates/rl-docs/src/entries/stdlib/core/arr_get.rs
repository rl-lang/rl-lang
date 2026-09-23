use crate::entry::FnEntry;

pub static ARR_GET: FnEntry = FnEntry {
    signature: "__arr_get(arr, idx)",
    description: "intrinsic: the element at idx. out-of-bounds indexes abort; RL code builds checked wrappers on top",
    example: r#"get __arr_get from core

dec int x = __arr_get([10, 20], 1)"#,
    expected_output: None,
    returns: "T",
    errors: Some("index out of bounds aborts"),
    see_also: &["__arr_set", "__arr_push"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
