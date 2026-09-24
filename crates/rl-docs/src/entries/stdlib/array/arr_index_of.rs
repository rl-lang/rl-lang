use crate::entry::FnEntry;

pub static ARR_INDEX_OF: FnEntry = FnEntry {
    signature: "arr_index_of(arr, value)",
    description: "returns the index of the first occurrence of value in the array, or -1 if not found",
    example: "get std::array::arr_index_of\n\narr_index_of([10, 20, 30], 20)?",
    expected_output: Some("1"),
    returns: "result[int]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_contains", "arr_find_index"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
