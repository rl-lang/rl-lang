use crate::entry::FnEntry;

pub static ABORT: FnEntry = FnEntry {
    signature: "__abort(msg)",
    description: "intrinsic: fails loudly with msg. returns T so it fits any storage type; it never produces a value",
    example: r#"get __abort from core

dec int x = __abort("unreachable")"#,
    expected_output: None,
    returns: "T",
    errors: None,
    see_also: &["__type_of"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
