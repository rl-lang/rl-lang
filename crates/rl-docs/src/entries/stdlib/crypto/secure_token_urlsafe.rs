use crate::entry::FnEntry;

pub static SECURE_TOKEN_URLSAFE: FnEntry = FnEntry {
    signature: "secure_token_urlsafe(n)",
    description: "n cryptographically secure random bytes, base64url-encoded without padding. safe to embed in URLs and filenames",
    example: r#"get secure_token_urlsafe from std::crypto

dec string tok = secure_token_urlsafe(16)"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["secure_token", "secure_token_hex", "base64_url_encode"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
