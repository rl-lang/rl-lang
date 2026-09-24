use crate::entry::FnEntry;

pub static CLOSE: FnEntry = FnEntry {
    signature: "close(handle)",
    description: "closes a file handle, releasing the resource",
    example: r#"get std::fs::open
get std::fs::close

dec file = open("data.txt", "r")?
close(file)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the handle is invalid"),
    see_also: &["open"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
