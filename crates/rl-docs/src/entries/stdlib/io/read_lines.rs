use crate::entry::FnEntry;

pub static READ_LINES: FnEntry = FnEntry {
    signature: "read_lines(path)",
    description: "reads a file and returns its lines as an array of strings",
    example: "get std::io::read_lines\n\ndec arr[string] data = read_lines(\"index.html\")?",
    expected_output: None,
    returns: "result[array[string]]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- the current process lacks permission to read `path`\n- `path`'s contents are not valid UTF-8",
    ),
    see_also: &["read_file", "read_bytes"],
    since: Some("v0.1.5"),
};
