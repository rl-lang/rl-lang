use crate::entry::FnEntry;

pub static ABS: FnEntry = FnEntry {
    signature: "abs(number)",
    description: "returns the absolute value of number",
    example: r#"
get std::math::abs

dec int x = -1
x.abs()?"#,
    expected_output: Some("1"),
    returns: "result[int] or result[float]",
    errors: Some("Will return error on non-number values"),
    see_also: &[],
    since: Some("v0.1.5"),
};
