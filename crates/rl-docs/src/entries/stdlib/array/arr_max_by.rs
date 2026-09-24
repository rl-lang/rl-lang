use crate::entry::FnEntry;

pub static ARR_MAX_BY: FnEntry = FnEntry {
    signature: "arr_max_by(arr, fn)",
    description: "returns the element for which fn returns the maximum value",
    example: "get std::array::arr_max_by\n\narr_max_by([\"a\", \"bb\", \"ccc\"], fn(s) { len(s) })?",
    expected_output: Some("\"ccc\""),
    returns: "result[T]",
    errors: Some("Returns err if the array is empty"),
    see_also: &["arr_max", "arr_min_by"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
