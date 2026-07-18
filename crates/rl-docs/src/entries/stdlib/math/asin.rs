use crate::entry::FnEntry;

pub static ASIN: FnEntry = FnEntry {
    signature: "asin(x)",
    description: "arc sine of x in radians",
    example: "get std::math::asin\n\nasin(1.0)",
    expected_output: Some("1.5707963267948966"),
    returns: "float",
    errors: None,
    see_also: &["acos", "atan"],
    since: Some("v0.1.5"),
};
