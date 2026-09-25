use crate::entry::FnEntry;

pub static BUF_NEW: FnEntry = FnEntry {
    signature: "__buf_new()",
    description: "intrinsic: a fresh empty byte buffer; amortized-O(1) append beats repeated arr_push/str_concat for incremental builds",
    example: r#"get __buf_new from core

dec b = __buf_new()"#,
    expected_output: None,
    returns: "handle(Buffer)",
    errors: None,
    see_also: &["__buf_free", "__buf_push_byte"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
