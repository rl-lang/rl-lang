use crate::entry::FnEntry;

pub static ATAN2: FnEntry = FnEntry {
    signature: "atan2(a, b)",
    description: "arc tangent of a/b using signs to determine quadrant",
    example: "get std::math::atan2\n\natan2(1.0, 1.0) // 0.7853...",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
