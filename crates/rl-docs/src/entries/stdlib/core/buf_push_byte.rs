use crate::entry::FnEntry;

pub static BUF_PUSH_BYTE: FnEntry = FnEntry {
    signature: "__buf_push_byte(b, byte)",
    description: "intrinsic: append one byte in amortized constant time",
    example: r#"get __buf_new, __buf_push_byte from core

dec b = __buf_new()
__buf_push_byte(b, 72 as byte)"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["__buf_append", "__buf_new"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
