use crate::entry::FnEntry;

pub static DELETE_FILE: FnEntry = FnEntry {
    signature: "delete_file(path)",
    description: "deletes a file at the given path",
    example: "get std::io::delete_file\n\ndelete_file(\"info.txt\")?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `path` does not exist\n- `path` is a directory, not a file\n- the current process lacks permission to remove `path`",
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
