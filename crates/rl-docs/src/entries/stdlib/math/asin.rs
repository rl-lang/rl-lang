use crate::entry::FnEntry;

pub static ASIN: FnEntry = FnEntry {
    signature: "asin(x)",
    description: "arc sine of x in radians",
    example: "get std::math::asin\n\nasin(1.0) // 1.5707...",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
