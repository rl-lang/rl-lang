use crate::entry::FnEntry;

pub static HEX_DECODE: FnEntry = FnEntry {
    signature: "hex_decode(s)",
    description: "decodes hex text back into bytes. odd length or non-hex input is an error",
    example: r#"get hex_decode from std::crypto
get result_unwrap from std::res

dec bytes = result_unwrap(hex_decode("6869"))"#,
    expected_output: None,
    returns: "result[array[byte]]",
    errors: Some("invalid hex"),
    see_also: &["hex_encode", "base64_decode"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
