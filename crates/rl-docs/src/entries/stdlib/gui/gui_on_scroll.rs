use crate::entry::FnEntry;

pub static GUI_ON_SCROLL: FnEntry = FnEntry {
    signature: "gui_on_scroll(window, function)",
    description: "registers `function` for mouse-wheel movement over `window`: called with `(dx, dy)` scroll delta while focused",
    example: r#"gui_on_scroll(main, fn(int dx, int dy) {
    gui_set_text(scroll_label, format("scroll: {},{}", dx, dy))?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_window", "gui_on_key", "gui_on_file_drop"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
