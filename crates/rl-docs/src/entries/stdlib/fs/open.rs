use crate::entry::FnEntry;

pub static OPEN: FnEntry = FnEntry {
    signature: "open(file, mode)",
    description: "opens a file and returns a file handle",
    example: r#"get std::fs::open
get std::fs::close

dec file = open("data.txt", "r")?
close(file)?"#,
    expected_output: None,
    returns: "result[handle(File)]",
    errors: Some("Will return error on invalid mode or if the file cannot be opened"),
    see_also: &["close", "read_handle", "write_handle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
