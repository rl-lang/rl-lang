use crate::entry::FnEntry;

pub static ARR_FIRST: FnEntry = FnEntry {
    signature: "arr_first(arr)",
    description: "returns the first element of the array",
    example: "get std::array::arr_first\n\narr_first([1, 2, 3])?",
    expected_output: Some("1"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["arr_last"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
