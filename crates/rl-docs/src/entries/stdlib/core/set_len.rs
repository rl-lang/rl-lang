use crate::entry::FnEntry;

pub static SET_LEN: FnEntry = FnEntry {
    signature: "__set_len(set)",
    description: "intrinsic: element count of a set",
    example: r#"get __set_new, __set_add, __set_len from core

dec s = __set_new()
__set_add(s, 1)
dec int n = __set_len(s)"#,
    expected_output: None,
    returns: "int",
    errors: Some("non-set aborts"),
    see_also: &["__set_has", "__set_add"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
