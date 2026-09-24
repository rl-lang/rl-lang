use crate::entry::FnEntry;

pub static ARR_SWAP: FnEntry = FnEntry {
    signature: "arr_swap(arr, i, j)",
    description: "returns a new array with elements at indices i and j swapped",
    example: "get std::array::arr_swap\n\narr_swap([1, 2, 3], 0, 2)?",
    expected_output: Some("[3, 2, 1]"),
    returns: "result[arr[T]]",
    errors: Some("Returns err if either index is out of bounds"),
    see_also: &["arr_reverse", "arr_sort"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
