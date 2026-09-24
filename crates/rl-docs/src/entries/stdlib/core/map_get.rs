use crate::entry::FnEntry;

pub static MAP_GET: FnEntry = FnEntry {
    signature: "__map_get(map, key)",
    description: "intrinsic: the value under key. missing keys abort; RL code builds result-returning wrappers on top",
    example: r#"get __map_get from core

dec int x = __map_get({"a": 1}, "a")"#,
    expected_output: None,
    returns: "V",
    errors: Some("missing key aborts"),
    see_also: &["__map_set", "__map_keys"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
