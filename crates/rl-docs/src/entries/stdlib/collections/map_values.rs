use crate::entry::FnEntry;

pub static MAP_VALUES: FnEntry = FnEntry {
    signature: "map_values(map)",
    description: "returns the map's values as an array",
    example: "get map_values from std::collections\nget arr_sort from std::array\n\ndec map[string, int] m = {\"a\": 2, \"b\": 1}\ndec arr[int] values = map_values(m)?\narr_sort(values)?",
    expected_output: Some("[1, 2]"),
    returns: "result[arr[V]]",
    errors: Some(
        "Will return error if `map` is not a map.\n\nNote: the returned array's element order is not guaranteed - a map is\nbacked by a hash map internally, so the same map can produce arrays in\ndifferent orders across runs. Sort the result with `arr_sort` (from\n`std::array`) if a stable order matters.",
    ),
    see_also: &["map_keys", "map_to_array"],
    since: Some("v0.4.0"),
};
