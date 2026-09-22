use crate::entry::FnEntry;

pub static WRITE_HANDLE: FnEntry = FnEntry {
    signature: "write(handle, data)",
    description: "writes a string to a file handle, returns bytes written",
    example: r#"get std::io::open
get std::io::write
get std::io::close

dec file = open("out.txt", "w")?
write(file, "hello world")?
close(file)?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if the handle is invalid or not open for writing"),
    see_also: &["read_handle", "flush"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::write_handle"),
    updated: Some("v2.1.0"),
};
