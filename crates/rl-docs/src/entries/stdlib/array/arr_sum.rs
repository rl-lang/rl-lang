use crate::entry::FnEntry;

pub static ARR_SUM: FnEntry = FnEntry {
    signature: "arr_sum(arr)",
    description: "returns the sum of all elements in an int or float array",
    example: "get std::array::arr_sum\n\narr_sum([1, 2, 3, 4])?",
    expected_output: Some("10"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n\nAn empty array is not rejected - it returns `0` (int) or `0.0` (float).",
    ),
    see_also: &["arr_product"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
