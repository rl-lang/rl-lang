use crate::entry::FnEntry;

pub static YAML_STRINGIFY: FnEntry = FnEntry {
    signature: "yaml_stringify(v)",
    description: "renders a value as block-style YAML. same shape rules as json_stringify",
    example: r#"get yaml_stringify from std::serialize
get result_unwrap from std::res

dec string s = result_unwrap(yaml_stringify({"host": "x"}))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("unrepresentable value"),
    see_also: &["yaml_parse", "json_stringify"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
