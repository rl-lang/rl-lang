use crate::entry::FnEntry;

pub static GUI_SET_PADDING: FnEntry = FnEntry {
    signature: "gui_set_padding(box, px)",
    description: "sets a container's inner margin in pixels (negative clamps to zero)",
    example: r#"gui_set_padding(col, 8)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_vbox", "gui_hbox", "gui_set_spacing"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
