use crate::entry::FnEntry;

pub static SET_NEW: FnEntry = FnEntry {
    signature: "__set_new()",
    description: "intrinsic: builds an empty set. the primitive behind set literals",
    example: r#"get __set_new, __set_add from core

dec s = __set_new()
__set_add(s, 1)"#,
    expected_output: None,
    returns: "set[T]",
    errors: None,
    see_also: &["__set_add", "__set_has"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
