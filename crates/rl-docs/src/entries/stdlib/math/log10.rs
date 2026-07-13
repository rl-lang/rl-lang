use crate::entry::FnEntry;

pub static LOG10: FnEntry = FnEntry {
    signature: "log10(x)",
    description: "base-10 logarithm of x",
    example: "get std::math::log10\n\nlog10(1000.0) // 3.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
