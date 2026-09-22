use crate::entry::FnEntry;

pub static BASE64_ENCODE: FnEntry = FnEntry {
    signature: "base64_encode(data)",
    description: "standard base64 encoding (with padding) of a byte array",
    example: r#"get base64_encode from std::crypto

dec string s = base64_encode([104, 105])"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["base64_decode", "base64_url_encode", "hex_encode"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
