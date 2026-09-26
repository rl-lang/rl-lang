use crate::entry::FnEntry;

pub static BUF_SLICE: FnEntry = FnEntry {
    signature: "__buf_slice(b, start, end)",
    description: "intrinsic: bytes from start to end as a string (lossy); bad ranges and split codepoints abort",
    example: r#"get __buf_new, __buf_append, __buf_slice from core

dec b = __buf_new()
__buf_append(b, "hi")
__buf_slice(b, 0, 1)"#,
    expected_output: None,
    returns: "string",
    errors: Some("bad range or split codepoint aborts"),
    see_also: &["__buf_to_string", "__buf_get_byte"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
