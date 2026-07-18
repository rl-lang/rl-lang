use crate::entry::FnEntry;

pub static MAX: FnEntry = FnEntry {
    signature: "max(a, b)",
    description: "returns the larger of a and b",
    example: r#"get std::math::max

max(4, 6)?"#,
    expected_output: Some("6"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `a` or `b` is not an int or float
- `a` and `b` are not the same type (e.g. mixing int and float)"#,
    ),
    see_also: &["min"],
    since: Some("v0.1.5"),
};
