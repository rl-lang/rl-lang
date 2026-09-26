use crate::entry::FnEntry;

pub static GUI_SCROLL: FnEntry = FnEntry {
    signature: "gui_scroll(window, x, y, width, height)",
    description: "creates a scroll viewport at (x, y) of the given size: children stack vertically, but only rows intersecting the visible slice render each frame (virtualized paging, no pixel scrolling). Scroll with `gui_scroll_to`, usually from `on_scroll`",
    example: r#"dec handle list = result_unwrap(gui_scroll(main, 330, 44, 280, 400))"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_scroll_to", "gui_scroll_pos", "gui_add", "gui_on_scroll"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
