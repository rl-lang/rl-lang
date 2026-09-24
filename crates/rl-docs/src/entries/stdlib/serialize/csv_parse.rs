use crate::entry::FnEntry;

pub static CSV_PARSE: FnEntry = FnEntry {
    signature: "csv_parse(s)",
    description: "parses CSV text into rows of strings. honors quotes and escapes",
    example: r#"get csv_parse from std::serialize
get result_unwrap from std::res

dec rows = result_unwrap(csv_parse("a,b\\n1,2"))"#,
    expected_output: None,
    returns: "result[array[array[string]]]",
    errors: Some("malformed CSV"),
    see_also: &["csv_parse_with_delimiter", "csv_parse_headers", "csv_stringify"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
