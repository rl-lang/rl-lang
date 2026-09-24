use crate::entry::FnEntry;

pub static SORTED_INSERT: FnEntry = FnEntry {
    signature: "sorted_insert(arr, value)",
    description: "inserts value into a sorted array at the correct position to maintain sorted order and returns the updated array",
    example: "get sorted_insert from std::collections\n\ndec arr[int] a = [1, 3, 5]\nsorted_insert(a, 4)",
    expected_output: Some("[1, 3, 4, 5]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not sorted in ascending order\n- `value`'s type does not match the array element type",
    ),
    see_also: &["bisect_left", "bisect_right"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
