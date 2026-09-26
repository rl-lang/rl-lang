use crate::entry::FnEntry;

pub static GUI_DRAW_LINE: FnEntry = FnEntry {
    signature: "gui_draw_line(canvas, x1, y1, x2, y2, r, g, b, thickness)",
    description: "queues one line segment for this frame. Colors are 0-255, thickness clamps to at least 1",
    example: r#"gui_draw_line(cv, 0, 0, 100, 100, 255, 0, 0, 2)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_canvas", "gui_draw_rect", "gui_draw_circle"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
