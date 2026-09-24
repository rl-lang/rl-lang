use crate::entry::FnEntry;

pub static ARR_SORT: FnEntry = FnEntry {
    signature: "arr_sort(arr)",
    description: "returns the array sorted in ascending order, only int or float arrays",
    example: "get std::array::arr_sort\n\narr_sort([3, 1, 2])?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[int]] or result[arr[float]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array",
    ),
    see_also: &["arr_sort_by", "arr_max", "arr_min"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
