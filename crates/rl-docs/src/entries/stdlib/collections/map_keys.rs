use crate::entry::FnEntry;

pub static MAP_KEYS: FnEntry = FnEntry {
    signature: "map_keys(map)",
    description: "returns the map's keys as an array",
    example: "get map_keys from std::collections\nget arr_sort from std::array\n\ndec map[int, string] m = {2: \"b\", 1: \"a\"}\ndec arr[int] keys = map_keys(m)?\narr_sort(keys)?",
    expected_output: Some("[1, 2]"),
    returns: "result[arr[K]]",
    errors: Some(
        "Will return error if `map` is not a map.\n\nNote: the returned array's element order is not guaranteed - a map is\nbacked by a hash map internally, so the same map can produce arrays in\ndifferent orders across runs. Sort the result with `arr_sort` (from\n`std::array`, int/float keys only) if a stable order matters.",
    ),
    see_also: &["map_values", "map_to_array"],
    since: Some("v0.4.0"),
};
