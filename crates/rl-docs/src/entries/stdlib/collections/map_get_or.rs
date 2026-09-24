use crate::entry::FnEntry;

pub static MAP_GET_OR: FnEntry = FnEntry {
    signature: "map_get_or(map, key, default)",
    description: "returns the value for key if it exists, otherwise returns default",
    example: "get map_get_or from std::collections\n\ndec map[string, int] m = {\"a\": 1}\nmap_get_or(m, \"b\", 99)",
    expected_output: Some("99"),
    returns: "result[V]",
    errors: Some(
        "Will return error on the following:\n\n- `map` is not a map\n- `key`'s type can't be used as a map key\n- `default`'s type does not match the map value type",
    ),
    see_also: &["map_get", "map_get_or_insert"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
