use crate::entry::FnEntry;

pub static ROUND: FnEntry = FnEntry {
    signature: "round(x)",
    description: "rounds x to the nearest integer",
    example: r#"get std::math::round

round(2.2)?"#,
    expected_output: Some("2.0"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` is not an int or float"#,
    ),
    see_also: &["ceil", "floor"],
    since: Some("v0.1.5"),
};
