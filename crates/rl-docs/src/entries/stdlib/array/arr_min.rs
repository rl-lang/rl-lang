use crate::entry::FnEntry;

pub static ARR_MIN: FnEntry = FnEntry {
    signature: "arr_min(arr)",
    description: "returns the smallest element in an int or float array",
    example: "get std::array::arr_min\n\narr_min([3, 1, 4, 1, 5])?",
    expected_output: Some("1"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n- `arr` is empty",
    ),
    see_also: &["arr_max", "arr_sort"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
