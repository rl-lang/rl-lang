use crate::entry::FnEntry;

pub static LOG2: FnEntry = FnEntry {
    signature: "log2(x)",
    description: "base-2 logarithm of x",
    example: r#"get std::math::log2

log2(8.0)?"#,
    expected_output: Some("3.0"),
    returns: "result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` is not an int or float"#,
    ),
    see_also: &["log", "log10"],
    since: Some("v0.1.5"),
};
