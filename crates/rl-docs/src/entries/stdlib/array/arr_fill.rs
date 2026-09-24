use crate::entry::FnEntry;

pub static ARR_FILL: FnEntry = FnEntry {
    signature: "arr_fill(value, count)",
    description: "creates an array filled with value repeated count times",
    example: "get std::array::arr_fill\n\narr_fill(0, 3)",
    expected_output: Some("[0, 0, 0]"),
    returns: "arr[T]",
    errors: Some(
        "Not validated: a negative `count` casts to a very large length rather\nthan erroring, and will attempt a huge allocation instead of failing\ncleanly.",
    ),
    see_also: &["arr_range"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
