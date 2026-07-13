use crate::entry::FnEntry;

pub static LOG2: FnEntry = FnEntry {
    signature: "log2(x)",
    description: "base-2 logarithm of x",
    example: "get std::math::log2\n\nlog2(8.0) // 3.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
