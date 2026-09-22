use crate::entry::FnEntry;

pub static SPINNER_TICK: FnEntry = FnEntry {
    signature: "spinner_tick(frame)",
    description: "renders one spinner frame (| / - \\) to stderr, cycling every 4 frames. writes to the runtime output buffer when one is set so test harnesses can capture it",
    example: r#"get spinner_tick from std::cli

dec int i = 0
while i < 8 {
    spinner_tick(i)
    i = i + 1
}"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["progress_bar"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
