use crate::entry::FnEntry;

pub static ARR_CHUNK: FnEntry = FnEntry {
    signature: "arr_chunk(arr, size)",
    description: "splits the array into chunks of the given size; the last chunk may be smaller",
    example: "get std::array::arr_chunk\n\narr_chunk([1, 2, 3, 4, 5], 2)?",
    expected_output: Some("[[1, 2], [3, 4], [5]]"),
    returns: "result[arr[arr[T]]]",
    errors: Some("Returns err if size is less than 1"),
    see_also: &["arr_windows", "arr_slice"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
