use crate::entry::FnEntry;

pub static ARR_WINDOWS: FnEntry = FnEntry {
    signature: "arr_windows(arr, size)",
    description: "returns sliding windows of the given size over the array",
    example: "get std::array::arr_windows\n\narr_windows([1, 2, 3, 4], 3)?",
    expected_output: Some("[[1, 2, 3], [2, 3, 4]]"),
    returns: "result[arr[arr[T]]]",
    errors: Some("Returns err if size is less than 1 or greater than array length"),
    see_also: &["arr_chunk", "arr_slice"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
