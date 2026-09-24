use crate::entry::FnEntry;

pub static ARR_COUNT: FnEntry = FnEntry {
    signature: "arr_count(arr)",
    description: "returns the number of elements in the array",
    example: "get std::array::arr_count\n\narr_count([1, 2, 3])?",
    expected_output: Some("3"),
    returns: "result[int]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["len", "arr_is_empty"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
