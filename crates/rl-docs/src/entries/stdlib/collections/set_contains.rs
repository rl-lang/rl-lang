use crate::entry::FnEntry;

pub static SET_CONTAINS: FnEntry = FnEntry {
    signature: "set_contains(set, value)",
    description: "true if value is present in the set",
    example: "get std::collections::set_contains\n\ndec set[int] s = {10, 20, 30}\nset_contains(s, 20)?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error if `set` is not a set.\n\nUnlike `set_add`/`set_remove`, passing a `value` whose type can't be used\nas a set element is not an error here - it just returns `false`.",
    ),
    see_also: &["set_add", "set_len"],
    since: Some("v0.4.0"),
};
