use crate::entry::FnEntry;

pub static ARR_LAST: FnEntry = FnEntry {
    signature: "arr_last(arr)",
    description: "returns the last element of the array",
    example: "get std::array::arr_last\n\narr_last([1, 2, 3])?",
    expected_output: Some("3"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["arr_first"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
