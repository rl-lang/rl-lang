use crate::entry::FnEntry;

pub static STR_SLICE: FnEntry = FnEntry {
    signature: "__str_slice(s, start, end)",
    description: "intrinsic: bytes from start up to (not including) end. bad ranges and codepoint splits abort",
    example: r#"get __str_slice from core

dec string sub = __str_slice("hello", 1, 4)"#,
    expected_output: None,
    returns: "string",
    errors: Some("bad range or codepoint split aborts"),
    see_also: &["__str_len", "__str_concat"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
