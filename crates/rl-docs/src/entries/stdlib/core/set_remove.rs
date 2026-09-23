use crate::entry::FnEntry;

pub static SET_REMOVE: FnEntry = FnEntry {
    signature: "__set_remove(set, val)",
    description: "intrinsic: drops val from the set. absent values abort, uniformly with reads",
    example: r#"get __set_new, __set_add, __set_remove from core

dec s = __set_new()
__set_add(s, 1)
__set_remove(s, 1)"#,
    expected_output: None,
    returns: "set[T]",
    errors: Some("absent value aborts"),
    see_also: &["__set_add", "__set_has"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
