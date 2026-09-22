use crate::entry::FnEntry;

pub static READ_LINE_WITH_HISTORY: FnEntry = FnEntry {
    signature: "read_line_with_history(prompt, history)",
    description: "same as read_line_editable, but preloads the given history array for up-arrow recall. returns a (line, updated-history) tuple; the line is appended to history unless it is empty",
    example: r#"get read_line_with_history from std::cli

dec pair = read_line_with_history("> ", ["help", "quit"])"#,
    expected_output: None,
    returns: "tuple[string, array[string]]",
    errors: None,
    see_also: &["read_line_editable", "prompt"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
