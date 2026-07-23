use crate::entry::FnEntry;

pub static MAP_LEN: FnEntry = FnEntry {
    signature: "map_len(map)",
    description: "returns the number of key-value pairs in the map",
    example: "get map_len from std::collections\n\ndec map[string, int] m = {\"a\": 1, \"b\": 2, \"c\": 3}\nmap_len(m)?",
    expected_output: Some("3"),
    returns: "result[int]",
    errors: Some("Will return error if `map` is not a map"),
    see_also: &["map_is_empty", "map_to_array"],
    since: Some("v0.4.0"),
};
