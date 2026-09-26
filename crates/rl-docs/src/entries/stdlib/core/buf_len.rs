use crate::entry::FnEntry;

pub static BUF_LEN: FnEntry = FnEntry {
    signature: "__buf_len(b)",
    description: "intrinsic: the number of bytes currently stored",
    example: r#"get __buf_new, __buf_len from core

dec b = __buf_new()
__buf_len(b)"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["__buf_new", "__buf_resize"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
