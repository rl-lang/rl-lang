use crate::entry::FnEntry;

pub static SIN: FnEntry = FnEntry {
    signature: "sin(x)",
    description: "sine of x in radians",
    example: "get std::math::sin\n\nsin(0.0) // 0.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
