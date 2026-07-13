use crate::entry::FnEntry;

pub static LERP: FnEntry = FnEntry {
    signature: "lerp(x, y, t)",
    description: "linear interpolation between x and y by factor t",
    example: "get std::math::lerp\n\nlerp(0.0, 10.0, 0.5) // 5.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
