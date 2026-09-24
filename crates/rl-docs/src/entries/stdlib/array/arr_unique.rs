use crate::entry::FnEntry;

pub static ARR_UNIQUE: FnEntry = FnEntry {
    signature: "arr_unique(arr)",
    description: "returns the array with duplicate values removed, preserving order",
    example: "get std::array::arr_unique\n\narr_unique([1, 2, 2, 3, 1])?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &[],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
