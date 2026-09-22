use crate::entry::FnEntry;

pub static SECURE_TOKEN_HEX: FnEntry = FnEntry {
    signature: "secure_token_hex(n)",
    description: "n cryptographically secure random bytes, hex-encoded for display or storage",
    example: r#"get secure_token_hex from std::crypto

dec string tok = secure_token_hex(16)"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["secure_token", "secure_token_urlsafe", "hex_encode"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
