use crate::entry::FnEntry;

pub static BUF_RESIZE: FnEntry = FnEntry {
    signature: "__buf_resize(b, n)",
    description: "intrinsic: grow (zero-filled) or shrink to exactly n bytes; negative aborts",
    example: r#"get __buf_new, __buf_resize, __buf_len from core

dec b = __buf_new()
__buf_resize(b, 4)
__buf_len(b)"#,
    expected_output: None,
    returns: "null",
    errors: Some("negative size aborts"),
    see_also: &["__buf_len", "__buf_addr"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
