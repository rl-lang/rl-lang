use crate::entry::FnEntry;

pub static DEGREES: FnEntry = FnEntry {
    signature: "degrees(x)",
    description: "convert radians to degrees",
    example: "get std::math::degrees\n\ndegrees(3.14159)",
    expected_output: Some("179.99984796050427"),
    returns: "float",
    errors: None,
    see_also: &["radians"],
    since: Some("v0.1.5"),
};
