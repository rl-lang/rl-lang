use crate::entry::FnEntry;

pub static ATAN: FnEntry = FnEntry {
    signature: "atan(x)",
    description: "arc tangent of x in radians",
    example: "get std::math::atan\n\natan(1.0) // 0.7853...",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
