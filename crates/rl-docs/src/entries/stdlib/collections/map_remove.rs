use crate::entry::FnEntry;

pub static MAP_REMOVE: FnEntry = FnEntry {
    signature: "map_remove(map, key)",
    description: "removes key (and its value) from the map and returns the updated map; removing a key that isn't present (including from an empty map) is a no-op, not an error",
    example: "get map_remove, map_len from std::collections\n\ndec map[string, int] m = {\"a\": 1}\nm = map_remove(m, \"a\")?\nmap_len(m)?",
    expected_output: Some("0"),
    returns: "result[map[K, V]]",
    errors: Some(
        "Will return error on the following:\n\n- `map` is not a map\n- `key`'s type can't be used as a map key (e.g. a function, closure, or native function)",
    ),
    see_also: &["map_contains", "map_clear"],
    since: Some("v0.4.0"),
};
