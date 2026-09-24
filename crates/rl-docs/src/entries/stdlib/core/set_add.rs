use crate::entry::FnEntry;

pub static SET_ADD: FnEntry = FnEntry {
    signature: "__set_add(set, val)",
    description: "intrinsic: inserts val into the set. non-sets and unhashable values abort",
    example: r#"get __set_new, __set_add from core

dec s = __set_new()
__set_add(s, 1)"#,
    expected_output: None,
    returns: "set[T]",
    errors: Some("non-set or unhashable value aborts"),
    see_also: &["__set_new", "__set_has"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
