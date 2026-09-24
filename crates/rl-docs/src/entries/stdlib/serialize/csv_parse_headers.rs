use crate::entry::FnEntry;

pub static CSV_PARSE_HEADERS: FnEntry = FnEntry {
    signature: "csv_parse_headers(s)",
    description: "parses CSV text using the first row as header keys, returning one map per data row. short rows leave keys absent",
    example: r#"get csv_parse_headers from std::serialize
get result_unwrap from std::res

dec rows = result_unwrap(csv_parse_headers("name,age\\nbob,30"))"#,
    expected_output: None,
    returns: "result[array[map[string, string]]]",
    errors: Some("malformed CSV"),
    see_also: &["csv_parse", "csv_stringify"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
