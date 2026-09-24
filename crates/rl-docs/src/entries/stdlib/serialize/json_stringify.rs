use crate::entry::FnEntry;

pub static JSON_STRINGIFY: FnEntry = FnEntry {
    signature: "json_stringify(v)",
    description: "renders a value as compact JSON. maps need string keys; null renders as null. exotic leaves (closures, handles) fall back to null",
    example: r#"get json_stringify from std::serialize

dec string s = json_stringify({"a": 1})"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["json_stringify_pretty", "json_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
