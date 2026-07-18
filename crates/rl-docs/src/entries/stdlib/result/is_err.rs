use crate::entry::FnEntry;

pub static IS_ERR: FnEntry = FnEntry {
    signature: "is_err(r)",
    description: "true if r is an err value",
    example: "get std::res::is_err\n\ndec result[int] r = err(\"not found\")\nis_err(r)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_ok"],
    since: Some("v0.1.5"),
};
