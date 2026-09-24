use crate::entry::FnEntry;

pub static CSV_STRINGIFY: FnEntry = FnEntry {
    signature: "csv_stringify(rows)",
    description: "renders rows of strings as CSV text, quoting fields as needed",
    example: r#"get csv_stringify from std::serialize

dec string s = csv_stringify([["a", "b"], ["1", "2"]])"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["csv_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
