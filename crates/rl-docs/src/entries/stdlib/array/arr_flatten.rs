use crate::entry::FnEntry;

pub static ARR_FLATTEN: FnEntry = FnEntry {
    signature: "arr_flatten(arr)",
    description: "flattens a nested array into a single array",
    example: "get std::array::arr_flatten\n\narr_flatten([[1, 2], [3, 4]])?",
    expected_output: Some("[1, 2, 3, 4]"),
    returns: "result[arr[T]]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_concat", "arr_flat_map"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
