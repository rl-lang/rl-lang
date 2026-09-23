use crate::entry::FnEntry;

pub static STR_CONCAT: FnEntry = FnEntry {
    signature: "__str_concat(a, b)",
    description: "intrinsic: the bytes of a followed by the bytes of b. string building needs a primitive because `+` rejects strings",
    example: r#"get __str_concat from core

dec string s = __str_concat("a", "b")"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["__str_slice", "__str_len"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
