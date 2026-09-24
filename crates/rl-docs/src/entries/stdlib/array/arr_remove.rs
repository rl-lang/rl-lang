use crate::entry::FnEntry;

pub static ARR_REMOVE: FnEntry = FnEntry {
    signature: "arr_remove(arr, index)",
    description: "removes the element at the given index and returns the updated array",
    example: "get std::array::arr_remove\n\narr_remove([1, 2, 3], 1)?",
    expected_output: Some("[1, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `index` is out of bounds, or negative",
    ),
    see_also: &["arr_insert", "arr_pop"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
