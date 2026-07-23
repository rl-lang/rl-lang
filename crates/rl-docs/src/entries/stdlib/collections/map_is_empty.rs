use crate::entry::FnEntry;

pub static MAP_IS_EMPTY: FnEntry = FnEntry {
    signature: "map_is_empty(map)",
    description: "true if the map has no key-value pairs",
    example: "get map_is_empty from std::collections\n\ndec map[string, int] m = {}\nmap_is_empty(m)?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some("Will return error if `map` is not a map"),
    see_also: &["map_len"],
    since: Some("v0.4.0"),
};
