use crate::entry::FnEntry;

pub static YAML_PARSE: FnEntry = FnEntry {
    signature: "yaml_parse(s)",
    description: "parses YAML text into maps, arrays and scalars with the same value mapping as JSON. non-string mapping keys are an error",
    example: r#"get yaml_parse from std::serialize
get result_unwrap from std::res

dec cfg = result_unwrap(yaml_parse("host: x\\nport: 80"))"#,
    expected_output: None,
    returns: "result[T]",
    errors: Some("malformed YAML with position"),
    see_also: &["yaml_stringify", "json_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
