use crate::entry::FnEntry;

pub static DATE_STR: FnEntry = FnEntry {
    signature: "format_date_str(timestamp)",
    description: "shorthand for format_time with \"%Y-%m-%d\", returns the date portion of a unix timestamp",
    example: r#"get std::time::format_date_str

format_date_str(1784305315)?"#,
    expected_output: Some("2026-07-17"),
    returns: "result[string]",
    errors: Some("Will return error on negative timestamp"),
    see_also: &[],
    since: Some("v0.1.5"),
};
