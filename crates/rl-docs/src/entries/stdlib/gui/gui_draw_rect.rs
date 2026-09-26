use crate::entry::FnEntry;

pub static GUI_DRAW_RECT: FnEntry = FnEntry {
    signature: "gui_draw_rect(canvas, x, y, w, h, r, g, b, filled)",
    description: "queues one rectangle for this frame, filled or 1px outline",
    example: r#"gui_draw_rect(cv, 0, 0, 600, 400, 20, 20, 26, true)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_canvas", "gui_draw_line", "gui_draw_circle"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
