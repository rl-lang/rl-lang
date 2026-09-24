use crate::entry::FnEntry;

pub static READ_LINE_EDITABLE: FnEntry = FnEntry {
    signature: "read_line_editable(prompt)",
    description: "reads one line with arrow-key editing support (rustyline). returns an empty string on EOF, interrupt, or read error",
    example: r#"get read_line_editable from std::cli

dec string cmd = read_line_editable("> ")"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["read_line_with_history", "prompt"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
