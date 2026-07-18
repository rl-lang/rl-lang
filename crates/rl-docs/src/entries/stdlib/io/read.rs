use crate::entry::FnEntry;

pub static READ: FnEntry = FnEntry {
    signature: "read()",
    description: "reads a line from stdin",
    example: "get std::io::read\n\ndec string name = read()?",
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if reading from stdin fails"),
    see_also: &["read_int", "read_float"],
    since: Some("v0.1.5"),
};

pub static READ_PROMPT: FnEntry = FnEntry {
    signature: "read(prompt)",
    description: "prints prompt and reads a line from stdin",
    example: "get std::io::read\n\ndec string name = read(\"enter your name: \")?",
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if reading from stdin fails"),
    see_also: &["read_int", "read_float"],
    since: Some("v0.1.5"),
};
