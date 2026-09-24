use crate::entry::FnEntry;

pub static IS_FUNCTION: FnEntry = FnEntry {
    signature: "is_function(v)",
    description: "true if v is of type fn",
    example: "get std::types::is_function\n\nis_function(|x| x + 1)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
