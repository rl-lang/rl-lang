use crate::entry::FnEntry;

pub static READ_HANDLE: FnEntry = FnEntry {
    signature: "read(handle, n)",
    description: "reads up to n bytes from a file handle, returns as a string",
    example: r#"get std::io::open
get std::io::read
get std::io::close

dec file = open("data.txt", "r")?
dec string chunk = read(file, 1024)?
close(file)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the handle is invalid or not open for reading"),
    see_also: &["read_all", "readline", "write_handle"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::read_handle"),
    updated: Some("v2.1.0"),
};
