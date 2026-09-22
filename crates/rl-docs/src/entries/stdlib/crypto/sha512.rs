use crate::entry::FnEntry;

pub static SHA512: FnEntry = FnEntry {
    signature: "sha512(data)",
    description: "SHA-512 hash of a byte array, returned as raw bytes. use hex_encode or base64_encode to display it",
    example: r#"get sha512, hex_encode from std::crypto

dec string hex = hex_encode(sha512([104, 105]))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["sha256", "sha1", "md5", "hex_encode"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
