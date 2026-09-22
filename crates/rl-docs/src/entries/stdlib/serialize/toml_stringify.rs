use crate::entry::FnEntry;

pub static TOML_STRINGIFY: FnEntry = FnEntry {
    signature: "toml_stringify(v)",
    description: "renders a string-keyed map as TOML. the top level must be a map; null has no TOML form and is an error",
    example: r#"get toml_stringify from std::serialize
get result_unwrap from std::res

dec string s = result_unwrap(toml_stringify({"host": "x"}))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("non-map top level or unrepresentable value"),
    see_also: &["toml_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
