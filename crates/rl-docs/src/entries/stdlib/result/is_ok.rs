use crate::entry::FnEntry;

pub static IS_OK: FnEntry = FnEntry {
    signature: "is_ok(r)",
    description: "true if r is an ok value",
    example: "get std::res::is_ok\n\ndec result[int] r = ok(42)\nis_ok(r)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_err"],
    since: Some("v0.1.5"),
};
