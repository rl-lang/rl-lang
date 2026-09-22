use crate::entry::FnEntry;

pub static PROGRESS_BAR: FnEntry = FnEntry {
    signature: "progress_bar(current, total, label)",
    description: "renders a single-line progress bar to stderr (carriage-return rewrite, newline at 100%). writes to the runtime output buffer when one is set so test harnesses can capture it",
    example: r#"get progress_bar from std::cli

dec int i = 0
while i <= 10 {
    progress_bar(i, 10, "working")
    i = i + 1
}"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["spinner_tick"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
