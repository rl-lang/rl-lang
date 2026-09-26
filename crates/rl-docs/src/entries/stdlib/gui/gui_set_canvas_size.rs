use crate::entry::FnEntry;

pub static GUI_SET_CANVAS_SIZE: FnEntry = FnEntry {
    signature: "gui_set_canvas_size(canvas, width, height)",
    description: "resizes a canvas region, e.g. to track the live window size from `gui_get_window_size` every frame. Must stay positive",
    example: r#"gui_set_canvas_size(cv, 600, 400)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_canvas", "gui_get_canvas_size", "gui_get_window_size"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
