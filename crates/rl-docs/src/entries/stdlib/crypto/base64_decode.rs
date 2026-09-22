use crate::entry::FnEntry;

pub static BASE64_DECODE: FnEntry = FnEntry {
    signature: "base64_decode(s)",
    description: "decodes standard base64 text back into bytes. invalid input is an error",
    example: r#"get base64_decode from std::crypto
get result_unwrap from std::res

dec bytes = result_unwrap(base64_decode("aGk="))"#,
    expected_output: None,
    returns: "result[array[byte]]",
    errors: Some("invalid base64"),
    see_also: &["base64_encode", "base64_url_decode", "hex_decode"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
