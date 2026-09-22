use crate::entry::FnEntry;

pub static JSON_IS_VALID: FnEntry = FnEntry {
    signature: "json_is_valid(s)",
    description: "cheap syntax check: true when s parses as JSON, false otherwise. use json_parse when you need the value or the error",
    example: r#"get json_is_valid from std::serialize

dec bool ok = json_is_valid("{\"a\": 1}")"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["json_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
