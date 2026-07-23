use crate::entry::FnEntry;

pub static MAP_MERGE: FnEntry = FnEntry {
    signature: "map_merge(map1, map2)",
    description: "merges map2 into map1 and returns the updated map - keys from map2 overwrite matching keys already in map1, keys only in either map are kept as-is",
    example: "get map_merge, map_len from std::collections\n\ndec map[string, int] a = {\"x\": 1}\ndec map[string, int] b = {\"x\": 9, \"y\": 2}\na = map_merge(a, b)?\nmap_len(a)?",
    expected_output: Some("2"),
    returns: "result[map[K, V]]",
    errors: Some(
        "Will return error on the following:\n\n- either `map1` or `map2` is not a map\n- `map1` and `map2` have different key or value types",
    ),
    see_also: &["map_to_array", "map_clear"],
    since: Some("v0.4.0"),
};
