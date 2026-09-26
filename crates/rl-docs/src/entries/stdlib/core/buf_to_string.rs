use crate::entry::FnEntry;

pub static BUF_TO_STRING: FnEntry = FnEntry {
    signature: "__buf_to_string(b)",
    description: "intrinsic: all bytes as a string (lossy, never aborts)",
    example: r#"get __buf_new, __buf_append, __buf_to_string from core

dec b = __buf_new()
__buf_append(b, "hi")
__buf_to_string(b)"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["__buf_slice", "__buf_append"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
