use crate::entry::FnEntry;

pub static ARR_CYCLE_TAKE: FnEntry = FnEntry {
    signature: "arr_cycle_take(arr, n)",
    description: "cycles the array and takes the first n elements",
    example: "get std::array::arr_cycle_take\n\narr_cycle_take([1, 2, 3], 7)?",
    expected_output: Some("[1, 2, 3, 1, 2, 3, 1]"),
    returns: "result[arr[T]]",
    errors: Some("Returns err if n is negative"),
    see_also: &["arr_repeat", "arr_slice"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
