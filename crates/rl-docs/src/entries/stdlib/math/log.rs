use crate::entry::FnEntry;

pub static LOG: FnEntry = FnEntry {
    signature: "log(x, base)",
    description: "logarithm of x in the given base",
    example: r#"get std::math::log

log(100.0, 10.0)?"#,
    expected_output: Some("2.0"),
    returns: "result[float]",
    errors: Some(
        r#"Will return error on the following:

- `x` or `base` is not an int or float"#,
    ),
    see_also: &["log2", "log10"],
    since: Some("v0.1.5"),
};
