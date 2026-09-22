use crate::entry::FnEntry;

pub static SECURE_TOKEN: FnEntry = FnEntry {
    signature: "secure_token(n)",
    description: "n cryptographically secure random bytes for use as a token. same source as secure_random_bytes",
    example: r#"get secure_token, hex_encode from std::crypto

dec string hex = hex_encode(secure_token(16))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["secure_random_bytes", "secure_token_hex", "secure_token_urlsafe"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
