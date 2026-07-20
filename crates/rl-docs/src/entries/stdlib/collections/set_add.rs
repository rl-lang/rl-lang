use crate::entry::FnEntry;

pub static SET_ADD: FnEntry = FnEntry {
    signature: "set_add(set, value)",
    description: "adds value to the set and returns the updated set - sets deduplicate automatically, so adding a value that's already present is a no-op",
    example: "get std::collections::set_add\n\ndec set[int] s = {}\nset_add(s, 42)?",
    expected_output: Some("{42}"),
    returns: "result[set[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `set` is not a set\n- `value`'s type can't be used as a set element (e.g. a function, closure, or native function)",
    ),
    see_also: &["set_remove", "set_contains"],
    since: Some("v0.4.0"),
};
