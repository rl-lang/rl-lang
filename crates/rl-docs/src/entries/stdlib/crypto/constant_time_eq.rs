use crate::entry::FnEntry;

pub static CONSTANT_TIME_EQ: FnEntry = FnEntry {
    signature: "constant_time_eq(a, b)",
    description: "timing-safe equality of two byte arrays. always use this (never ==) when comparing secrets, MACs or hashes",
    example: r#"get constant_time_eq from std::crypto

dec bool same = constant_time_eq([1, 2, 3], [1, 2, 3])"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["hmac_sha256", "sha256"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
