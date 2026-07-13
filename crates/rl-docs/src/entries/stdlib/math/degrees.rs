use crate::entry::FnEntry;

pub static DEGREES: FnEntry = FnEntry {
    signature: "degrees(x)",
    description: "convert radians to degrees",
    example: "get std::math::degrees\n\ndegrees(3.14159) // 180.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
