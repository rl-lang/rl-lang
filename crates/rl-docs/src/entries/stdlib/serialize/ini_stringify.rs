use crate::entry::FnEntry;

pub static INI_STRINGIFY: FnEntry = FnEntry {
    signature: "ini_stringify(v)",
    description: "renders a map of string-map sections as INI text. the empty section name holds unsectioned keys",
    example: r#"get ini_stringify from std::serialize
get result_unwrap from std::res

dec string s = result_unwrap(ini_stringify({"server": {"host": "x"}}))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("non-string sections, keys or values"),
    see_also: &["ini_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
