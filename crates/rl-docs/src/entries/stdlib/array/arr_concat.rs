use crate::entry::FnEntry;

pub static ARR_CONCAT: FnEntry = FnEntry {
    signature: "arr_concat(arr1, arr2)",
    description: "concatenates two arrays of the same type into one",
    example: "get std::array::arr_concat\n\narr_concat([1, 2], [3, 4])?",
    expected_output: Some("[1, 2, 3, 4]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr1` or `arr2` is not an array\n- `arr1` and `arr2` have different element types",
    ),
    see_also: &["arr_push", "arr_flatten"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
