use crate::entry::FnEntry;

pub static BISECT_LEFT: FnEntry = FnEntry {
    signature: "bisect_left(arr, val)",
    description: "finds the leftmost insertion point for val in a sorted array, such that all elements to the left are less than val",
    example: "get bisect_left from std::collections\n\ndec arr[int] a = [1, 2, 4, 4, 5]\nbisect_left(a, 4)",
    expected_output: Some("2"),
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not sorted in ascending order\n- `val`'s type is not comparable to the array element type",
    ),
    see_also: &["bisect_right", "sorted_insert"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
