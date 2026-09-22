use crate::entry::FnEntry;

pub static UUID_V4: FnEntry = FnEntry {
    signature: "uuid_v4()",
    description: "a random UUID (version 4) as a hyphenated string",
    example: r#"get uuid_v4 from std::crypto

dec string id = uuid_v4()"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["uuid_v7", "uuid_parse", "secure_token_hex"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
