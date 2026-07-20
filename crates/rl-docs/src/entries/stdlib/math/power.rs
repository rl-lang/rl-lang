use crate::entry::FnEntry;

pub static POW: FnEntry = FnEntry {
    signature: "pow(a, b)",
    description: "raises a to the power of b",
    example: r#"get std::math::pow

pow(2, 2)?"#,
    expected_output: Some("4"),
    returns: "result[int] or result[float]",
    errors: Some(
        r#"Will return error on the following:

- `a` or `b` is not an int or float

Note: an int `a` raised to a negative int `b` will panic rather than return
an error, since the exponent is cast to an unsigned integer internally."#,
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
