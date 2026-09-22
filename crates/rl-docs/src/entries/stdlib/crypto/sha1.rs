use crate::entry::FnEntry;

pub static SHA1: FnEntry = FnEntry {
    signature: "sha1(data)",
    description: "SHA-1 hash of a byte array, returned as raw bytes. for legacy checksum verification only, never for security",
    example: r#"get sha1, hex_encode from std::crypto

dec string hex = hex_encode(sha1([104, 105]))"#,
    expected_output: None,
    returns: "array[byte]",
    errors: None,
    see_also: &["sha256", "sha512", "md5", "hex_encode"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
