use crate::entry::FnEntry;

pub static GUI_WINDOW_MINIMIZE: FnEntry = FnEntry {
    signature: "gui_window_minimize(window, minimized)",
    description: "minimizes (`true`) on the next frame, or restores (`false`). One-shot request, applied then cleared",
    example: r#"gui_window_minimize(main, true)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_window", "gui_window_fullscreen", "gui_window_maximize"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
