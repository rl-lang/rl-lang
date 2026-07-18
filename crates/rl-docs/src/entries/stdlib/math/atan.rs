use crate::entry::FnEntry;

pub static ATAN: FnEntry = FnEntry {
    signature: "atan(x)",
    description: "arc tangent of x in radians",
    example: "get std::math::atan\n\natan(1.0)",
    expected_output: Some("0.7853981633974483"),
    returns: "float",
    errors: None,
    see_also: &["asin", "acos", "atan2"],
    since: Some("v0.1.5"),
};
