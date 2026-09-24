use crate::entry::FnEntry;

pub static READ_HANDLE: FnEntry = FnEntry {
    signature: "read(handle, n)",
    description: "reads up to n bytes from a file handle, returns as a string",
    example: r#"get std::fs::open
get std::fs::read
get std::fs::close

dec file = open("data.txt", "r")?
dec string chunk = read(file, 1024)?
close(file)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the handle is invalid or not open for reading"),
    see_also: &["read_all", "readline", "write_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
