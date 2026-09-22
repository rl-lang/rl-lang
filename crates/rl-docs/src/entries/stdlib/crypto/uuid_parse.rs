use crate::entry::FnEntry;

pub static UUID_PARSE: FnEntry = FnEntry {
    signature: "uuid_parse(s)",
    description: "validates UUID text and returns its normalized form. malformed input is an error",
    example: r#"get uuid_parse, uuid_v4 from std::crypto
get result_unwrap, is_err from std::res

dec string id = result_unwrap(uuid_parse(uuid_v4()))
dec bool bad = is_err(uuid_parse("not-a-uuid"))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("malformed UUID"),
    see_also: &["uuid_v4", "uuid_v7"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
