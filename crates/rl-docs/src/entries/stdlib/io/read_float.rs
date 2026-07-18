use crate::entry::FnEntry;

pub static READ_FLOAT: FnEntry = FnEntry {
    signature: "read_float()",
    description: "reads a line from stdin then parses it to a float",
    example: "get std::io::read_float\n\ndec float pi = read_float()?",
    expected_output: None,
    returns: "result[float]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid float",
    ),
    see_also: &["read", "read_int"],
    since: Some("v0.1.5"),
};

pub static READ_FLOAT_PROMPT: FnEntry = FnEntry {
    signature: "read_float(prompt)",
    description: "prints prompt and reads a line from stdin then parses it to a float",
    example: "get std::io::read_float\n\ndec float pi = read_float(\"enter pi: \")?",
    expected_output: None,
    returns: "result[float]",
    errors: Some(
        "Will return error on the following:\n\n- reading from stdin fails\n- the input line is not a valid float",
    ),
    see_also: &["read", "read_int"],
    since: Some("v0.1.5"),
};
