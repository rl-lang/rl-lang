use crate::entry::FnEntry;

pub static GUI_VBOX: FnEntry = FnEntry {
    signature: "gui_vbox(window, x, y, spacing)",
    description: "creates a vertical layout container at (x, y): children stack top-down with `spacing` pixels between them. Position children with `gui_add`, never absolute coordinates",
    example: r#"dec handle col = result_unwrap(gui_vbox(main, 40, 30, 12))
gui_add(col, title)?"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_hbox", "gui_add", "gui_set_align"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
