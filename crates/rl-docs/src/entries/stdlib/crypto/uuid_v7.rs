use crate::entry::FnEntry;

pub static UUID_V7: FnEntry = FnEntry {
    signature: "uuid_v7()",
    description: "a time-ordered UUID (version 7) as a hyphenated string. sorts by creation time",
    example: r#"get uuid_v7 from std::crypto

dec string id = uuid_v7()"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["uuid_v4", "uuid_parse", "secure_token_hex"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
