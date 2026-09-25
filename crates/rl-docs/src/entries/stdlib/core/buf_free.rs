use crate::entry::FnEntry;

pub static BUF_FREE: FnEntry = FnEntry {
    signature: "__buf_free(b)",
    description: "intrinsic: release the buffer; using the handle afterwards aborts",
    example: r#"get __buf_new, __buf_free from core

dec b = __buf_new()
__buf_free(b)"#,
    expected_output: None,
    returns: "null",
    errors: Some("use-after-free aborts"),
    see_also: &["__buf_new", "__buf_clear"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
