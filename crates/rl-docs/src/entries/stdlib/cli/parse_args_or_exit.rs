use crate::entry::FnEntry;

pub static PARSE_ARGS_OR_EXIT: FnEntry = FnEntry {
    signature: "parse_args_or_exit(spec)",
    description: "same as parse_args, but prints the error plus usage text to stderr and exits the process with code 2 on failure instead of returning err",
    example: r#"get parse_args_or_exit from std::cli

dec args = parse_args_or_exit([
    {"name": "verbose", "flag": "true"},
])"#,
    expected_output: None,
    returns: "map",
    errors: None,
    see_also: &["parse_args", "usage_string"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
