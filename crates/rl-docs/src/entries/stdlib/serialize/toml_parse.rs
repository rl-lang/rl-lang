use crate::entry::FnEntry;

pub static TOML_PARSE: FnEntry = FnEntry {
    signature: "toml_parse(s)",
    description: "parses TOML text into nested maps, arrays and scalars. datetimes become strings",
    example: r#"get toml_parse from std::serialize
get result_unwrap from std::res

dec cfg = result_unwrap(toml_parse("[server]\\nhost = \\"x\\""))"#,
    expected_output: None,
    returns: "result[T]",
    errors: Some("malformed TOML with position"),
    see_also: &["toml_stringify", "json_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
