use crate::entry::FnEntry;

pub static HYPOT: FnEntry = FnEntry {
    signature: "hypot(a, b)",
    description: "length of the hypotenuse given two sides: √(a² + b²)",
    example: "get std::math::hypot\n\nhypot(3.0, 4.0) // 5.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
