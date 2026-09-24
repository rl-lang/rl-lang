use crate::entry::FnEntry;

pub static STR_LEN: FnEntry = FnEntry {
    signature: "__str_len(s)",
    description: "intrinsic: byte length of a string (bytes, not chars, matching byte indexing)",
    example: r#"get __str_len from core

dec int n = __str_len("hi")"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["__str_get_byte", "__str_slice"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
