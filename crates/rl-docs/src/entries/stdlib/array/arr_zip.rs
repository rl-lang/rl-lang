use crate::entry::FnEntry;

pub static ARR_ZIP: FnEntry = FnEntry {
    signature: "arr_zip(arr1, arr2)",
    description: "zips two arrays into an array of tuples; stops at the shorter array",
    example: "get std::array::arr_zip\n\narr_zip([1, 2, 3], [\"a\", \"b\", \"c\"])",
    expected_output: Some("[(1, \"a\"), (2, \"b\"), (3, \"c\")]"),
    returns: "arr[tuple[T, U]]",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the\nfollowing:\n\n- called with a number of arguments other than 2\n- `arr1` or `arr2` is not an array",
    ),
    see_also: &["arr_map", "arr_concat"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
