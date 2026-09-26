use crate::entry::FnEntry;

pub static BUF_CLEAR: FnEntry = FnEntry {
    signature: "__buf_clear(b)",
    description: "intrinsic: drop all bytes but keep capacity for reuse",
    example: r#"get __buf_new, __buf_append, __buf_clear, __buf_len from core

dec b = __buf_new()
__buf_append(b, "hi")
__buf_clear(b)
__buf_len(b)"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["__buf_free", "__buf_resize"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
