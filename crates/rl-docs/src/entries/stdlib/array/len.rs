use crate::entry::FnEntry;

pub static LEN: FnEntry = FnEntry {
    signature: "len(x)",
    description: "length of a string, array, or tuple",
    example: "get std::array::len\n\nlen(\"hello\")",
    expected_output: Some("5"),
    returns: "int",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) if `x` is not a\nstring, array, or tuple.",
    ),
    see_also: &["arr_count", "arr_is_empty"],
    since: Some("v0.1.5"),
    deprecated: Some("moved to std::len"),
    updated: Some("v0.1.5"),
};
