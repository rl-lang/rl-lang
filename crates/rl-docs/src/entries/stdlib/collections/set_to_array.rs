use crate::entry::FnEntry;

pub static SET_TO_ARRAY: FnEntry = FnEntry {
    signature: "set_to_array(set)",
    description: "returns the set's items as an array",
    example: "get std::collections::set_to_array\n\ndec set[int] s = {7}\nset_to_array(s)?",
    expected_output: Some("[7]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error if `set` is not a set.\n\nNote: the returned array's element order is not guaranteed - a set is\nbacked by a hash set internally, so the same set can produce arrays in\ndifferent orders across runs. Sort the result with `arr_sort` (from\n`std::array`) if a stable order matters.",
    ),
    see_also: &["set_len"],
    since: Some("v0.4.0"),
};
