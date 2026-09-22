use crate::entry::FnEntry;

pub static BASE64_URL_ENCODE: FnEntry = FnEntry {
    signature: "base64_url_encode(data)",
    description: "URL-safe base64 encoding (no padding) of a byte array. safe for URLs and filenames",
    example: r#"get base64_url_encode from std::crypto

dec string s = base64_url_encode([104, 105])"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["base64_url_decode", "base64_encode", "secure_token_urlsafe"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
