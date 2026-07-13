use crate::entry::FnEntry;

pub static CLAMP: FnEntry = FnEntry {
    signature: "clamp(x, min, max)",
    description: "clamps x between min and max, returning min if x < min, max if x > max",
    example: "get std::math::clamp\n\nclamp(12, 15, 20) // 15",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
