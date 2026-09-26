use crate::entry::FnEntry;

pub static GUI_SCROLL_TO: FnEntry = FnEntry {
    signature: "gui_scroll_to(scroll, px)",
    description: "scrolls a viewport to `px` pixels from the top, clamped into range. Negative clamps to zero",
    example: r#"gui_scroll_to(list, 200)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_scroll", "gui_scroll_pos", "gui_on_scroll"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
