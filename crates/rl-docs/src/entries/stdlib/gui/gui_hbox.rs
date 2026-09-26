use crate::entry::FnEntry;

pub static GUI_HBOX: FnEntry = FnEntry {
    signature: "gui_hbox(window, x, y, spacing)",
    description: "creates a horizontal layout container at (x, y): children stack left-to-right with `spacing` pixels between them",
    example: r#"dec handle row = result_unwrap(gui_hbox(main, 0, 0, 8))
gui_add(row, ok_btn)?
gui_add(row, cancel_btn)?"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_vbox", "gui_add", "gui_set_align"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
