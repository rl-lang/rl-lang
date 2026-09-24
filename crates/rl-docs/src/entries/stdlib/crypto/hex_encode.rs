use crate::entry::FnEntry;

pub static HEX_ENCODE: FnEntry = FnEntry {
    signature: "hex_encode(data)",
    description: "lowercase hex encoding of a byte array. the usual way to display digests",
    example: r#"get hex_encode, sha256 from std::crypto

dec string hex = hex_encode(sha256([104, 105]))"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["hex_decode", "base64_encode", "sha256"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
