use crate::entry::FnEntry;

pub static SIGN: FnEntry = FnEntry {
    signature: "sign(x)",
    description: "returns -1, 0, or 1 based on the sign of x",
    example: "get std::math::sign\n\nsign(-5.0)",
    expected_output: Some("-1.0"),
    returns: "float",
    errors: None,
    see_also: &["abs"],
    since: Some("v0.1.5"),
};
