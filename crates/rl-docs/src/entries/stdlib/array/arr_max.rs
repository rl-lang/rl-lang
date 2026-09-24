use crate::entry::FnEntry;

pub static ARR_MAX: FnEntry = FnEntry {
    signature: "arr_max(arr)",
    description: "returns the largest element in an int or float array",
    example: "get std::array::arr_max\n\narr_max([3, 1, 4, 1, 5])?",
    expected_output: Some("5"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n- `arr` is empty",
    ),
    see_also: &["arr_min", "arr_sort"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
