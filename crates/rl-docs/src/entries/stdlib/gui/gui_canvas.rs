use crate::entry::FnEntry;

pub static GUI_CANVAS: FnEntry = FnEntry {
    signature: "gui_canvas(window, x, y, width, height)",
    description: "creates a drawable region at (x, y) of the given size. Issue `gui_draw_*` calls every frame (typically from `on_frame`); the command list clears after each render, so static scenes redraw too. Coordinates are local to the canvas origin",
    example: r#"dec handle cv = result_unwrap(gui_canvas(main, 20, 20, 600, 400))"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_draw_line", "gui_draw_rect", "gui_draw_circle", "gui_draw_text", "gui_on_frame"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
