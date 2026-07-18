use crate::entry::FnEntry;

pub static FLOOR: FnEntry = FnEntry {
    signature: "floor(x)",
    description: "largest integer less than or equal to x",
    example: r#"get std::math::floor

floor(1.23)?"#,
    expected_output: Some("1.0"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` is not an int or float"#,
    ),
    see_also: &["ceil", "round"],
    since: Some("v0.1.5"),
};
