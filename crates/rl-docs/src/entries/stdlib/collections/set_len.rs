use crate::entry::FnEntry;

pub static SET_LEN: FnEntry = FnEntry {
    signature: "set_len(set)",
    description: "returns the number of items in the set",
    example: "get std::collections::set_len\n\ndec set[int] s = {1, 2, 3}\nset_len(s)?",
    expected_output: Some("3"),
    returns: "result[int]",
    errors: Some("Will return error if `set` is not a set"),
    see_also: &["set_is_empty", "set_to_array"],
    since: Some("v0.4.0"),
};
