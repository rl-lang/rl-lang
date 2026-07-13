use crate::entry::FnEntry;

pub static MOD: FnEntry = FnEntry {
    signature: "mod(a, b)",
    description: "remainder of a divided by b",
    example: "get std::math::mod\n\nmod(10, 3) // 1",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
