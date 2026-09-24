use crate::entry::FnEntry;

pub static HMAC_SHA512: FnEntry = FnEntry {
    signature: "hmac_sha512(key, data)",
    description: "keyed HMAC-SHA-512 of data, returned as raw bytes. any key length works",
    example: r#"get hmac_sha512, hex_encode from std::crypto

dec string hex = hex_encode(hmac_sha512([107, 101, 121], [104, 105]))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["hmac_sha256", "sha512", "hex_encode"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
