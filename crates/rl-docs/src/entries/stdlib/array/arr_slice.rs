use crate::entry::FnEntry;

pub static ARR_SLICE: FnEntry = FnEntry {
    signature: "arr_slice(arr, start, end)",
    description: "returns a sub-array from start to end (exclusive)",
    example: "get std::array::arr_slice\n\narr_slice([1, 2, 3, 4], 1, 3)?",
    expected_output: Some("[2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `start` or `end` is out of bounds for `arr` (this also catches\n  negative `start`/`end`, since they cast to a very large index)\n- `start` is greater than `end`",
    ),
    see_also: &["arr_slice"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
