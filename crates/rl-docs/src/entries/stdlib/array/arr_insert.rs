use crate::entry::FnEntry;

pub static ARR_INSERT: FnEntry = FnEntry {
    signature: "arr_insert(arr, value, index)",
    description: "inserts value at the given index, shifting elements right; index may equal arr_count(arr) to append",
    example: "get std::array::arr_insert\n\narr_insert([1, 3], 2, 1)?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `index` is negative or greater than `arr_count(arr)`\n- `value`'s type does not match `arr`'s element type",
    ),
    see_also: &["arr_push", "arr_remove"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
