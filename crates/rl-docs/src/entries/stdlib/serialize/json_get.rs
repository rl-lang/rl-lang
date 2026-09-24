use crate::entry::FnEntry;

pub static JSON_GET: FnEntry = FnEntry {
    signature: "json_get(v, path)",
    description: "dotted-path lookup into parsed JSON: maps by key, arrays by numeric segment. for dynamic paths; static access should use field and index syntax. missing keys, bad indexes and type mismatches are errors",
    example: r#"get json_get, json_parse from std::serialize
get result_unwrap from std::res

dec data = result_unwrap(json_parse("{\"server\": {\"host\": \"x\"}}"))
dec string host = result_unwrap(json_get(data, "server.host"))"#,
    expected_output: None,
    returns: "result[T]",
    errors: Some("missing key, bad index, or type mismatch"),
    see_also: &["json_parse"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
