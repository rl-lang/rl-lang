use crate::entry::FnEntry;

pub static MAP_KEYS: FnEntry = FnEntry {
    signature: "__map_keys(map)",
    description: "intrinsic: the map's keys as an array. the iteration primitive RL-written map code builds on",
    example: r#"get __map_keys from core
get len from std::array
get result_unwrap from std::res

dec int n = result_unwrap(len(__map_keys({"a": 1})))"#,
    expected_output: None,
    returns: "array[K]",
    errors: None,
    see_also: &["__map_get", "__map_new"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
