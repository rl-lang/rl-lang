use crate::entry::FnEntry;

pub static READ_INT: FnEntry = FnEntry {
    signature: "read_int()",
    description: "reads a line from stdin then parses it to an integer",
    example: "get std::io::read_int\n\ndec int age = read_int()?",
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid integer",
    ),
    see_also: &["read", "read_float"],
    since: Some("v0.1.5"),
};

pub static READ_INT_PROMPT: FnEntry = FnEntry {
    signature: "read_int(prompt)",
    description: "prints prompt and reads a line from stdin then parses it to an integer",
    example: "get std::io::read_int\n\ndec int age = read_int(\"enter your age: \")?",
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid integer",
    ),
    see_also: &["read", "read_float"],
    since: Some("v0.1.5"),
};
