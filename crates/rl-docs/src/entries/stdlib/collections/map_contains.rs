use crate::entry::FnEntry;

pub static MAP_CONTAINS: FnEntry = FnEntry {
    signature: "map_contains(map, key)",
    description: "true if key is present in the map",
    example: "get map_contains from std::collections\n\ndec map[string, int] m = {\"a\": 1, \"b\": 2}\nmap_contains(m, \"a\")?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error if `map` is not a map.\n\nUnlike `map_remove`/`map_get`, passing a `key` whose type can't be used\nas a map key is not an error here - it just returns `false`.",
    ),
    see_also: &["map_get", "map_len"],
    since: Some("v0.4.0"),
};
