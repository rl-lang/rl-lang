use crate::entry::FnEntry;

pub static READ_BYTES: FnEntry = FnEntry {
    signature: "read_bytes(path)",
    description: "reads the entire contents of a file as a byte array",
    example: "get std::io::read_bytes\n\ndec arr[byte] data = read_bytes(\"backup_info.txt\")?",
    expected_output: None,
    returns: "result[array[byte]]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- the current process lacks permission to read `path`\n\nUnlike `read_file`/`read_lines`, this does not require valid UTF-8, since\nit reads raw bytes rather than a string.",
    ),
    see_also: &["read_file", "read_lines"],
    since: Some("v0.1.5"),
};
