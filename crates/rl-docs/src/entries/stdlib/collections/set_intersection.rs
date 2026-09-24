use crate::entry::FnEntry;

pub static SET_INTERSECTION: FnEntry = FnEntry {
    signature: "set_intersection(a, b)",
    description: "returns a new set containing only elements present in both sets",
    example: "get set_intersection from std::collections\n\ndec set[int] a = {1, 2, 3}\ndec set[int] b = {2, 3, 4}\nset_intersection(a, b)",
    expected_output: Some("{2, 3}"),
    returns: "result[set[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `a` or `b` is not a set\n- `a` and `b` have different element types",
    ),
    see_also: &["set_union", "set_difference", "set_symmetric_difference"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
