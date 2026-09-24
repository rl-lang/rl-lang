use crate::entry::FnEntry;

pub static INI_PARSE: FnEntry = FnEntry {
    signature: "ini_parse(s)",
    description: "parses INI text into a map of sections, each a string map. unsectioned keys live under the empty section name",
    example: r#"get ini_parse from std::serialize
get result_unwrap from std::res

dec cfg = result_unwrap(ini_parse("[server]\\nhost = x"))"#,
    expected_output: None,
    returns: "result[map[string, map[string, string]]]",
    errors: Some("malformed INI"),
    see_also: &["ini_stringify", "toml_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
