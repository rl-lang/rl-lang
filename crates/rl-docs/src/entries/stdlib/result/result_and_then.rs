use crate::entry::FnEntry;

pub static RESULT_AND_THEN: FnEntry = FnEntry {
    signature: "result_and_then(r, fn)",
    description: "if r is ok, applies fn to the ok value and returns the result; if err, propagates the error",
    example: r#"get std::res::result_and_then

dec result[int] r = ok(10)
result_and_then(r, fn(x) { x * 2 })"#,
    expected_output: Some("ok(20)"),
    returns: "result[T]",
    errors: None,
    see_also: &["result_unwrap_or_else", "result_map", "is_ok"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
