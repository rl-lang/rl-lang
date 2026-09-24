use crate::entry::FnEntry;

pub static MAP_SET: FnEntry = FnEntry {
    signature: "__map_set(map, key, val)",
    description: "intrinsic: stores val under key, in place like the runtime's shared maps",
    example: r#"get __map_new, __map_set from core

dec m = __map_new()
__map_set(m, "a", 1)"#,
    expected_output: None,
    returns: "map[K, V]",
    errors: None,
    see_also: &["__map_get", "__map_keys"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
