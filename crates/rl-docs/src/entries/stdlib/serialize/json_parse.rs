use crate::entry::FnEntry;

pub static JSON_PARSE: FnEntry = FnEntry {
    signature: "json_parse(s)",
    description: "parses JSON text into maps, arrays and scalars. numbers without fraction or exponent become int, the rest float, null becomes null. errors include the position",
    example: r#"get json_parse from std::serialize
get result_unwrap from std::res
get map_get from std::collections

dec data = result_unwrap(json_parse("{\"host\": \"x\", \"port\": 80}"))
dec string host = map_get(data, "host").result_unwrap()"#,
    expected_output: None,
    returns: "result[T]",
    errors: Some("malformed JSON with line/column"),
    see_also: &["json_stringify", "json_is_valid", "json_get"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
