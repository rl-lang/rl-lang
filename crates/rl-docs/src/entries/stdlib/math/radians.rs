use crate::entry::FnEntry;

pub static RADIANS: FnEntry = FnEntry {
    signature: "radians(x)",
    description: "convert degrees to radians",
    example: "get std::math::radians\n\nradians(180.0)",
    expected_output: Some("3.141592653589793"),
    returns: "float",
    errors: None,
    see_also: &["degrees"],
    since: Some("v0.1.5"),
};
