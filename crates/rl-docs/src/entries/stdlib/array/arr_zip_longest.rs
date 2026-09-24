use crate::entry::FnEntry;

pub static ARR_ZIP_LONGEST: FnEntry = FnEntry {
    signature: "arr_zip_longest(arr1, arr2, default1, default2)",
    description: "zips two arrays into tuples, using default values when one array is shorter",
    example: "get std::array::arr_zip_longest\n\narr_zip_longest([1, 2], [10, 20, 30], 0, 0)?",
    expected_output: Some("[(1, 10), (2, 20), (0, 30)]"),
    returns: "result[arr[tuple[T, T]]]",
    errors: None,
    see_also: &["arr_zip"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
