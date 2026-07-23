use crate::entry::FnEntry;

pub static MAP_TO_ARRAY: FnEntry = FnEntry {
    signature: "map_to_array(map)",
    description: "returns the map's entries as an array of (key, value) tuples",
    example: "get map_to_array from std::collections\n\ndec map[string, int] m = {\"a\": 1}\nmap_to_array(m)?",
    expected_output: Some("[(a, 1)]"),
    returns: "result[arr[(K, V)]]",
    errors: Some(
        "Will return error if `map` is not a map.\n\nNote: the returned array's entry order is not guaranteed - a map is\nbacked by a hash map internally, so the same map can produce arrays in\ndifferent orders across runs.",
    ),
    see_also: &["map_keys", "map_values"],
    since: Some("v0.4.0"),
};
