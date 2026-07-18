use crate::entry::FnEntry;

pub static ATAN2: FnEntry = FnEntry {
    signature: "atan2(a, b)",
    description: "arc tangent of a/b using signs to determine quadrant",
    example: "get std::math::atan2\n\natan2(1.0, 1.0)",
    expected_output: Some("0.7853981633974483"),
    returns: "float",
    errors: None,
    see_also: &["atan"],
    since: Some("v0.1.5"),
};
