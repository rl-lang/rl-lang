use crate::entry::FnEntry;

pub static MAP_NEW: FnEntry = FnEntry {
    signature: "__map_new()",
    description: "intrinsic: builds an empty string-keyed map. the primitive behind map literals",
    example: r#"get __map_new, __map_set from core

dec m = __map_new()
__map_set(m, "a", 1)"#,
    expected_output: None,
    returns: "map[string, T]",
    errors: None,
    see_also: &["__map_get", "__map_set", "__map_keys"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
