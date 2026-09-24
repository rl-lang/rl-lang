use crate::entry::FnEntry;

pub static MAP_GET_OR_INSERT: FnEntry = FnEntry {
    signature: "map_get_or_insert(map, key, value)",
    description: "returns the value for key if it exists; otherwise inserts value at key and returns it",
    example: "get map_get_or_insert, map_contains from std::collections\n\ndec map[string, int] m = {\"a\": 1}\nmap_get_or_insert(m, \"b\", 2)\nmap_contains(m, \"b\")?",
    expected_output: Some("true"),
    returns: "result[V]",
    errors: Some(
        "Will return error on the following:\n\n- `map` is not a map\n- `key`'s type can't be used as a map key\n- `value`'s type does not match the map value type",
    ),
    see_also: &["map_get", "map_get_or"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
