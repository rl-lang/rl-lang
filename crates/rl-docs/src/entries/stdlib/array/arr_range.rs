use crate::entry::FnEntry;

pub static ARR_RANGE: FnEntry = FnEntry {
    signature: "arr_range(start, end, step)",
    description: "creates an int array from start to end (exclusive) with the given step",
    example: "get std::array::arr_range\n\narr_range(0, 6, 2)?",
    expected_output: Some("[0, 2, 4]"),
    returns: "result[arr[int]]",
    errors: Some("Will return error if `step` is 0 or negative"),
    see_also: &["arr_fill"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
