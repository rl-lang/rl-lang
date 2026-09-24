use crate::entry::FnEntry;

pub static READ_ALL: FnEntry = FnEntry {
    signature: "read_all(handle)",
    description: "reads all remaining bytes from a file handle until EOF",
    example: r#"get std::io::open
get std::io::read_all
get std::io::close

dec file = open("data.txt", "r")?
dec string content = read_all(file)?
close(file)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the handle is invalid or not open for reading"),
    see_also: &["read_handle", "readline"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::read_all"),
    updated: Some("v2.1.0"),
};
