use crate::entry::FnEntry;

pub static ARR_LEN: FnEntry = FnEntry {
    signature: "__arr_len(arr)",
    description: "intrinsic: element count of an array",
    example: r#"get __arr_len from core

dec int n = __arr_len([1, 2, 3])"#,
    expected_output: None,
    returns: "int",
    errors: Some("non-array aborts"),
    see_also: &["__arr_get", "__arr_push"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
