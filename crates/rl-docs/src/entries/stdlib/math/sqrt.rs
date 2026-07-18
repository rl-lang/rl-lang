use crate::entry::FnEntry;

pub static SQRT: FnEntry = FnEntry {
    signature: "sqrt(x)",
    description: "square root of x",
    example: r#"get std::math::sqrt

sqrt(4)?"#,
    expected_output: Some("2.0"),
    returns: "result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` is not an int or float

Note: a negative `x` does not error - it returns `NaN`."#,
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
