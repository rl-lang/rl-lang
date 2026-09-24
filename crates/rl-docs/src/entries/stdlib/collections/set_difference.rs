use crate::entry::FnEntry;

pub static SET_DIFFERENCE: FnEntry = FnEntry {
    signature: "set_difference(a, b)",
    description: "returns a new set containing elements that are in a but not in b",
    example: "get set_difference from std::collections\n\ndec set[int] a = {1, 2, 3}\ndec set[int] b = {2, 3, 4}\nset_difference(a, b)",
    expected_output: Some("{1}"),
    returns: "result[set[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `a` or `b` is not a set\n- `a` and `b` have different element types",
    ),
    see_also: &["set_union", "set_intersection", "set_symmetric_difference"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
