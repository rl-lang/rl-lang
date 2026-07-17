use crate::entry::FnEntry;

pub static TIME_STR: FnEntry = FnEntry {
    signature: "format_time_str(timestamp)",
    description: "shorthand for format_time with \"%H:%M:%S\", returns the time portion of a unix timestamp",
    example: r#"get std::time::format_time_str

format_time_str(1784305564)?"#,
    expected_output: Some("16:26:04"),
    returns: "result[string]",
    errors: Some("Will return error on negative timestamp"),
    see_also: &[],
    since: Some("v0.1.5"),
};
