use crate::entry::FnEntry;

pub static BUF_APPEND: FnEntry = FnEntry {
    signature: "__buf_append(b, s)",
    description: "intrinsic: append a whole string's UTF-8 bytes at once (renderer fast path)",
    example: r#"get __buf_new, __buf_append from core

dec b = __buf_new()
__buf_append(b, "hi")"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["__buf_push_byte", "__buf_to_string"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
