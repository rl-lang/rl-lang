use crate::entry::FnEntry;

pub static BISECT_RIGHT: FnEntry = FnEntry {
    signature: "bisect_right(arr, val)",
    description: "finds the rightmost insertion point for val in a sorted array, such that all elements to the left are less than or equal to val",
    example: "get bisect_right from std::collections\n\ndec arr[int] a = [1, 2, 4, 4, 5]\nbisect_right(a, 4)",
    expected_output: Some("4"),
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not sorted in ascending order\n- `val`'s type is not comparable to the array element type",
    ),
    see_also: &["bisect_left", "sorted_insert"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
