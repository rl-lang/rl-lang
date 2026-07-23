use crate::entry::FnEntry;

pub static MAP_CLEAR: FnEntry = FnEntry {
    signature: "map_clear(map)",
    description: "removes every entry from the map and returns the (now empty) map",
    example: "get map_clear, map_is_empty from std::collections\n\ndec map[string, int] m = {\"a\": 1, \"b\": 2}\nm = map_clear(m)?\nmap_is_empty(m)?",
    expected_output: Some("true"),
    returns: "result[map[K, V]]",
    errors: Some("Will return error if `map` is not a map"),
    see_also: &["map_remove", "map_is_empty"],
    since: Some("v0.4.0"),
};
