use crate::entry::FnEntry;

pub static ARR_POP: FnEntry = FnEntry {
    signature: "arr_pop(arr)",
    description: "removes the last element and returns the updated array",
    example: "get std::array::arr_pop\n\narr_pop([1, 2, 3])?",
    expected_output: Some("[1, 2]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["arr_push", "arr_remove"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
