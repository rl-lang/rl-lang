use crate::entry::FnEntry;

pub static MOD: FnEntry = FnEntry {
    signature: "mod(a, b)",
    description: "remainder of a divided by b",
    example: r#"get std::math::mod

mod(10, 3)?"#,
    expected_output: Some("1"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `a` or `b` is not an int or float"#,
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
