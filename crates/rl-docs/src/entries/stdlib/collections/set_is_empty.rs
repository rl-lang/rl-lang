use crate::entry::FnEntry;

pub static SET_IS_EMPTY: FnEntry = FnEntry {
    signature: "set_is_empty(set)",
    description: "true if the set has no items",
    example: "get std::collections::set_is_empty\n\ndec set[int] s = {}\nset_is_empty(s)?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some("Will return error if `set` is not a set"),
    see_also: &["set_len"],
    since: Some("v0.4.0"),
};
