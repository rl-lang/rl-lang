use crate::entry::FnEntry;

pub static JSON_STRINGIFY_PRETTY: FnEntry = FnEntry {
    signature: "json_stringify_pretty(v)",
    description: "renders a value as indented JSON. same mapping rules as json_stringify",
    example: r#"get json_stringify_pretty from std::serialize
get println from std::io

println(json_stringify_pretty({"a": 1}))"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["json_stringify", "json_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
