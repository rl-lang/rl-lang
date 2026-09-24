use crate::entry::FnEntry;

pub static UNICODE_CATEGORY: FnEntry = FnEntry {
    signature: "unicode_category(ch)",
    description: "returns the Unicode general category of the character",
    example: "get std::str::unicode_category\n\nunicode_category(\"A\")",
    expected_output: Some("\"Lu\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
