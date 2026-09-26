use crate::entry::FnEntry;

pub static GUI_WINDOW_FULLSCREEN: FnEntry = FnEntry {
    signature: "gui_window_fullscreen(window, fullscreen)",
    description: "enters (`true`) or leaves (`false`) fullscreen on the next frame. One-shot request, applied then cleared",
    example: r#"gui_window_fullscreen(main, true)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_window", "gui_window_maximize", "gui_window_minimize"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
