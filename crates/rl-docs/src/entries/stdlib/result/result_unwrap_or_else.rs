use crate::entry::FnEntry;

pub static RESULT_UNWRAP_OR_ELSE: FnEntry = FnEntry {
    signature: "result_unwrap_or_else(r, fn)",
    description: "if r is ok, returns the ok value; if err, calls fn with the error value and returns its result",
    example: r#"get std::res::result_unwrap_or_else

dec result[int] r = err("file not found")
result_unwrap_or_else(r, fn(e) { 0 })"#,
    expected_output: Some("0"),
    returns: "T",
    errors: None,
    see_also: &["result_and_then", "result_unwrap", "result_unwrap_or"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
