use crate::entry::FnEntry;

pub static READLINE: FnEntry = FnEntry {
    signature: "readline(handle)",
    description: "reads a single line from a file handle (without the trailing newline)",
    example: r#"get std::fs::open
get std::fs::readline
get std::fs::close

dec file = open("data.txt", "r")?
dec string line = readline(file)?
close(file)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the handle is invalid or not open for reading"),
    see_also: &["read_handle", "read_all"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
