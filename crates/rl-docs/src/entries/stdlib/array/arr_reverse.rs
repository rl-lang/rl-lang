use crate::entry::FnEntry;

pub static ARR_REVERSE: FnEntry = FnEntry {
    signature: "arr_reverse(arr)",
    description: "reverses the order of elements in the array",
    example: "get std::array::arr_reverse\n\narr_reverse([1, 2, 3])?",
    expected_output: Some("[3, 2, 1]"),
    returns: "result[arr[T]]",
    errors: Some("Will return error if `arr` is not an array"),
    see_also: &["arr_sort"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
