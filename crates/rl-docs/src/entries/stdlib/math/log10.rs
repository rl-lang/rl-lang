use crate::entry::FnEntry;

pub static LOG10: FnEntry = FnEntry {
    signature: "log10(x)",
    description: "base-10 logarithm of x",
    example: r#"get std::math::log10

log10(1000.0)?"#,
    expected_output: Some("3.0"),
    returns: "result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` is not an int or float"#,
    ),
    see_also: &["log", "log2"],
    since: Some("v0.1.5"),
};
