use crate::entry::FnEntry;

pub static SET_UNION: FnEntry = FnEntry {
    signature: "set_union(a, b)",
    description: "returns a new set containing all elements from both sets",
    example: "get set_union from std::collections\n\ndec set[int] a = {1, 2}\ndec set[int] b = {2, 3}\nset_union(a, b)",
    expected_output: Some("{1, 2, 3}"),
    returns: "result[set[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `a` or `b` is not a set\n- `a` and `b` have different element types",
    ),
    see_also: &["set_intersection", "set_difference", "set_symmetric_difference"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
