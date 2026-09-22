use crate::entry::FnEntry;

pub static USAGE_STRING: FnEntry = FnEntry {
    signature: "usage_string(spec)",
    description: "generates --help text from a parse_args spec array, including short aliases, flags, defaults and help strings",
    example: r#"get usage_string from std::cli
get result_unwrap from std::res
get println from std::io

dec spec = [{"name": "verbose", "short": "v", "flag": "true", "help": "turn on verbose output"}]
println(result_unwrap(usage_string(spec)))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("malformed spec entry"),
    see_also: &["parse_args", "parse_args_or_exit"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
