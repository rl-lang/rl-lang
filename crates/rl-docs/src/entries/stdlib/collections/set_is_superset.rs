use crate::entry::FnEntry;

pub static SET_IS_SUPERSET: FnEntry = FnEntry {
    signature: "set_is_superset(a, b)",
    description: "returns true if a contains every element in b",
    example: "get set_is_superset from std::collections\n\ndec set[int] a = {1, 2, 3}\ndec set[int] b = {1, 2}\nset_is_superset(a, b)",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error on the following:\n\n- `a` or `b` is not a set\n- `a` and `b` have different element types",
    ),
    see_also: &["set_is_subset"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
