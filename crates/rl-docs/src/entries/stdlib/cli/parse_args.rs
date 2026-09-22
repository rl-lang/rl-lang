use crate::entry::FnEntry;

pub static PARSE_ARGS: FnEntry = FnEntry {
    signature: "parse_args(spec)",
    description: "parses command-line arguments against a spec array of option maps. each map supports name (required), flag (\"true\"/\"false\" string - maps are homogeneous at runtime so a real bool cannot appear here), short (single-char alias), default (string) and help (string). supports --name value, --name=value and -s value forms; -- ends flag parsing and positionals land in the \"_\" array. missing non-flag options without defaults are an error; flags default to false",
    example: r#"get parse_args from std::cli
get result_unwrap from std::res

dec spec = [
    {"name": "verbose", "flag": "true"},
    {"name": "output", "short": "o", "default": "out.txt"},
]
dec args = result_unwrap(parse_args(spec))"#,
    expected_output: None,
    returns: "result[map]",
    errors: Some("unknown argument, missing value, or missing required option"),
    see_also: &["parse_args_or_exit", "usage_string"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
