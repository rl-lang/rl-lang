use crate::entry::FnEntry;

pub static MAP_HAS: FnEntry = FnEntry {
    signature: "__map_has(map, key)",
    description: "intrinsic: true when key is in the map. what check-then-remove is built on",
    example: r#"get __map_new, __map_set, __map_has from core

dec m = __map_new()
__map_set(m, "a", 1)
dec bool hit = __map_has(m, "a")"#,
    expected_output: None,
    returns: "bool",
    errors: Some("non-map aborts"),
    see_also: &["__map_get", "__map_remove", "__set_has"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
