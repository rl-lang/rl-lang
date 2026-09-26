use crate::entry::FnEntry;

pub static GUI_SELECTABLE: FnEntry = FnEntry {
    signature: "gui_selectable(window, text, selected, x, y)",
    description: "creates one selectable row at (x, y): highlighted while `selected`, fires `on_click` like a button. Selection state is RL-owned - read with `gui_is_selected`, write with `gui_set_selected`",
    example: r#"dec handle row = result_unwrap(gui_selectable(main, "alpha", false, 20, 100))"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_set_selected", "gui_is_selected", "gui_on_click"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
