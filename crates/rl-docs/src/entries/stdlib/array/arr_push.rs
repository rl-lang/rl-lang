use crate::entry::FnEntry;

pub static ARR_PUSH: FnEntry = FnEntry {
    signature: "arr_push(arr, value)",
    description: "appends value to the end of the array and returns the updated array",
    example: "get std::array::arr_push\n\narr_push([1, 2], 3)?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `value`'s type does not match `arr`'s element type",
    ),
    see_also: &["arr_pop", "arr_insert"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
