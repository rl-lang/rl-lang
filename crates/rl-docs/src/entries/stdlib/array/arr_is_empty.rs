use crate::entry::FnEntry;

pub static ARR_IS_EMPTY: FnEntry = FnEntry {
    signature: "arr_is_empty(arr)",
    description: "true if the array has no elements",
    example: "get std::array::arr_is_empty\n\narr_is_empty([])?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_count"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
