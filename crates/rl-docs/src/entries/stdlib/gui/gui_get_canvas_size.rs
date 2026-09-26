use crate::entry::FnEntry;

pub static GUI_GET_CANVAS_SIZE: FnEntry = FnEntry {
    signature: "gui_get_canvas_size(canvas)",
    description: "the canvas region size as [width, height]",
    example: r#"dec arr[float] s = result_unwrap(gui_get_canvas_size(cv))"#,
    expected_output: None,
    returns: "result[array[float]]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_canvas", "gui_set_canvas_size"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
