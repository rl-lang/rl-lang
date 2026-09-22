use crate::entry::FnEntry;

pub static CSV_PARSE_WITH_DELIMITER: FnEntry = FnEntry {
    signature: "csv_parse_with_delimiter(s, delim)",
    description: "parses delimited text (tabs, semicolons, pipes) into rows of strings. delim is one character",
    example: r#"get csv_parse_with_delimiter from std::serialize
get result_unwrap from std::res

dec rows = result_unwrap(csv_parse_with_delimiter("a;b\\n1;2", ";"))"#,
    expected_output: None,
    returns: "result[array[array[string]]]",
    errors: Some("empty delimiter or malformed input"),
    see_also: &["csv_parse", "csv_parse_headers", "csv_stringify"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
