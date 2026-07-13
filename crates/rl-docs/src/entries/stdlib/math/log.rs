use crate::entry::FnEntry;

pub static LOG: FnEntry = FnEntry {
    signature: "log(x, base)",
    description: "logarithm of x in the given base",
    example: "get std::math::log\n\nlog(100.0, 10.0) // 2.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
