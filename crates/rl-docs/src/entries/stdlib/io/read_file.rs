use crate::entry::FnEntry;

pub static READ_FILE: FnEntry = FnEntry {
    signature: "read_file(path)",
    description: "reads the entire contents of a file as a string",
    example: "get std::io::read_file\n\ndec string data = read_file(\"backup_info.txt\")?",
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- the current process lacks permission to read `path`\n- `path`'s contents are not valid UTF-8",
    ),
    see_also: &["read_lines", "read_bytes"],
    since: Some("v0.1.5"),
};
