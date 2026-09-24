use crate::entry::FnEntry;

pub static READLINE: FnEntry = FnEntry {
    signature: "readline(handle)",
    description: "reads a single line from a file handle (without the trailing newline)",
    example: r#"get std::io::open
get std::io::readline
get std::io::close

dec file = open("data.txt", "r")?
dec string line = readline(file)?
close(file)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the handle is invalid or not open for reading"),
    see_also: &["read_handle", "read_all"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::readline"),
    updated: Some("v2.1.0"),
};
