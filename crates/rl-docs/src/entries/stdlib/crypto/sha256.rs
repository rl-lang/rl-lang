use crate::entry::FnEntry;

pub static SHA256: FnEntry = FnEntry {
    signature: "sha256(data)",
    description: "SHA-256 hash of a byte array, returned as raw bytes. use hex_encode or base64_encode to display it",
    example: r#"get sha256, hex_encode from std::crypto

dec string hex = hex_encode(sha256([104, 105]))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["sha512", "sha1", "md5", "hex_encode"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
