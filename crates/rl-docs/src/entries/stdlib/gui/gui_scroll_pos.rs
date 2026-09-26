use crate::entry::FnEntry;

pub static GUI_SCROLL_POS: FnEntry = FnEntry {
    signature: "gui_scroll_pos(scroll)",
    description: "the current scroll offset in pixels from the top",
    example: r#"dec int cur = result_unwrap(gui_scroll_pos(list))"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_scroll", "gui_scroll_to"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
