use crate::entry::FnEntry;

pub static HMAC_SHA256: FnEntry = FnEntry {
    signature: "hmac_sha256(key, data)",
    description: "keyed HMAC-SHA-256 of data, returned as raw bytes. any key length works",
    example: r#"get hmac_sha256, hex_encode from std::crypto

dec string hex = hex_encode(hmac_sha256([107, 101, 121], [104, 105]))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["hmac_sha512", "sha256", "hex_encode"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
