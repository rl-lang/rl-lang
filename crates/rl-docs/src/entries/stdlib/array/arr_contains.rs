use crate::entry::FnEntry;

pub static ARR_CONTAINS: FnEntry = FnEntry {
    signature: "arr_contains(arr, value)",
    description: "true if the array contains the given value",
    example: "get std::array::arr_contains\n\narr_contains([1, 2, 3], 2)?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_index_of", "arr_find"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
