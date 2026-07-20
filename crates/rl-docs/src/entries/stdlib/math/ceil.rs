use crate::entry::FnEntry;

pub static CEIL: FnEntry = FnEntry {
    signature: "ceil(x)",
    description: "smallest integer greater than or equal to x",
    example: r#"get std::math::ceil

ceil(2.12)?"#,
    expected_output: Some("3.0"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` is not an int or float"#,
    ),
    see_also: &["floor", "round"],
    since: Some("v0.1.5"),
};
