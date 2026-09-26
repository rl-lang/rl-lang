use crate::entry::FnEntry;

pub static READ_ALL: FnEntry = FnEntry {
    signature: "read_all(handle)",
    description: "reads all remaining bytes from a file handle until EOF",
    example: r#"get std::fs::open
get std::fs::read_all
get std::fs::close

dec file = open("data.txt", "r")?
dec string content = read_all(file)?
close(file)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the handle is invalid or not open for reading"),
    see_also: &["read_handle", "readline"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
