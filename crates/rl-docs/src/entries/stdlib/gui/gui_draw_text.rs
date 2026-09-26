use crate::entry::FnEntry;

pub static GUI_DRAW_TEXT: FnEntry = FnEntry {
    signature: "gui_draw_text(canvas, text, x, y, size, r, g, b)",
    description: "queues one text draw for this frame: HUD and score text without label widgets. Size clamps to at least 1",
    example: r#"gui_draw_text(cv, format("fps: {}", fps), 8, 8, 16, 200, 200, 200)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_canvas", "gui_label"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
