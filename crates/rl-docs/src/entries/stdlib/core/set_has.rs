use crate::entry::FnEntry;

pub static SET_HAS: FnEntry = FnEntry {
    signature: "__set_has(set, val)",
    description: "intrinsic: true when val is in the set. non-sets and unhashable values abort",
    example: r#"get __set_new, __set_add, __set_has from core

dec s = __set_new()
__set_add(s, 1)
dec bool hit = __set_has(s, 1)"#,
    expected_output: None,
    returns: "bool",
    errors: Some("non-set or unhashable value aborts"),
    see_also: &["__set_new", "__set_add"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
