use crate::entry::FnEntry;

pub static MIN: FnEntry = FnEntry {
    signature: "min(a, b)",
    description: "returns the smaller of a and b",
    example: r#"get std::math::min

min(4, 6)?"#,
    expected_output: Some("4"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `a` or `b` is not an int or float
- `a` and `b` are not the same type (e.g. mixing int and float)"#,
    ),
    see_also: &["max"],
    since: Some("v0.1.5"),
};
