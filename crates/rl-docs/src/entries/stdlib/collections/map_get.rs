use crate::entry::FnEntry;

pub static MAP_GET: FnEntry = FnEntry {
    signature: "map_get(map, key)",
    description: "returns the value stored at key - unlike `map[key]` indexing, a missing key produces an `err` result instead of a runtime panic, so it can be handled with `?` or a `match`",
    example: "get map_get from std::collections\n\ndec map[string, int] m = {\"a\": 1}\nmap_get(m, \"a\")?",
    expected_output: Some("1"),
    returns: "result[V]",
    errors: Some(
        "Will return error on the following:\n\n- `map` is not a map\n- `key`'s type can't be used as a map key\n- `key` is not present in the map",
    ),
    see_also: &["map_contains", "map_keys", "map_values"],
    since: Some("v0.4.0"),
};
