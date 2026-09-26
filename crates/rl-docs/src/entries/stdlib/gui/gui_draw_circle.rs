use crate::entry::FnEntry;

pub static GUI_DRAW_CIRCLE: FnEntry = FnEntry {
    signature: "gui_draw_circle(canvas, x, y, radius, r, g, b, filled)",
    description: "queues one circle for this frame, filled or 1px outline. Radius clamps to at least 1",
    example: r#"gui_draw_circle(cv, 100, 100, 10, 255, 0, 0, true)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_canvas", "gui_draw_line", "gui_draw_rect"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
