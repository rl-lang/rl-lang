use crate::entry::FnEntry;

pub static MAP_REMOVE: FnEntry = FnEntry {
    signature: "__map_remove(map, key)",
    description: "intrinsic: drops key from the map. missing keys abort, uniformly with reads; check with __map_has for ensure-absent",
    example: r#"get __map_new, __map_set, __map_remove from core

dec m = __map_new()
__map_set(m, "a", 1)
__map_remove(m, "a")"#,
    expected_output: None,
    returns: "map[K, V]",
    errors: Some("missing key aborts"),
    see_also: &["__map_get", "__map_set"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
