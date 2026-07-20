use crate::entry::FnEntry;

pub static SET_REMOVE: FnEntry = FnEntry {
    signature: "set_remove(set, value)",
    description: "removes value from the set and returns the updated set; removing a value that isn't present (including from an empty set) is a no-op, not an error",
    example: "get std::collections::set_remove\n\ndec set[int] s = {5}\nset_remove(s, 5)?",
    expected_output: Some("{}"),
    returns: "result[set[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `set` is not a set\n- `value`'s type can't be used as a set element (e.g. a function, closure, or native function)",
    ),
    see_also: &["set_add", "set_contains"],
    since: Some("v0.4.0"),
};
