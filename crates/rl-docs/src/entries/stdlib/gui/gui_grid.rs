use crate::entry::FnEntry;

pub static GUI_GRID: FnEntry = FnEntry {
    signature: "gui_grid(window, x, y, columns)",
    description: "creates a grid container at (x, y): children flow left-to-right into `columns` columns. Each row is as tall as its tallest child, each column as wide as its widest; cells are top-left aligned. Position children with `gui_add`",
    example: r#"dec handle form = result_unwrap(gui_grid(main, 20, 44, 2))
gui_add(form, name_label)?
gui_add(form, name_box)?"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_vbox", "gui_hbox", "gui_add", "gui_set_spacing"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
