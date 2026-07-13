use crate::entry::FnEntry;

pub static SIGN: FnEntry = FnEntry {
    signature: "sign(x)",
    description: "returns -1, 0, or 1 based on the sign of x",
    example: "get std::math::sign\n\nsign(-5) // -1",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
