use crate::entry::FnEntry;

pub static ARR_PARTITION: FnEntry = FnEntry {
    signature: "arr_partition(arr, fn)",
    description: "splits the array into two arrays: elements where fn returns true, and elements where fn returns false",
    example: "get std::array::arr_partition\n\narr_partition([1, 2, 3, 4], fn(n) { n % 2 == 0 })?",
    expected_output: Some("([2, 4], [1, 3])"),
    returns: "result[arr[arr[T]]]",
    errors: None,
    see_also: &["arr_filter"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
