use crate::entry::FnEntry;

pub static ARR_MIN_BY: FnEntry = FnEntry {
    signature: "arr_min_by(arr, fn)",
    description: "returns the element for which fn returns the minimum value",
    example: "get std::array::arr_min_by\n\narr_min_by([\"a\", \"bb\", \"ccc\"], fn(s) { len(s) })?",
    expected_output: Some("\"a\""),
    returns: "result[T]",
    errors: Some("Returns err if the array is empty"),
    see_also: &["arr_min", "arr_max_by"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
