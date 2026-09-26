use crate::entry::FnEntry;

pub static GUI_IS_SELECTED: FnEntry = FnEntry {
    signature: "gui_is_selected(selectable)",
    description: "whether a selectable row is currently highlighted",
    example: r#"dec bool on = result_unwrap(gui_is_selected(row))"#,
    expected_output: None,
    returns: "result[bool]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_selectable", "gui_set_selected"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
