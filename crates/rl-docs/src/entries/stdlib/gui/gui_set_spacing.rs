use crate::entry::FnEntry;

pub static GUI_SET_SPACING: FnEntry = FnEntry {
    signature: "gui_set_spacing(box, px)",
    description: "sets the pixels between a container's children (negative clamps to zero)",
    example: r#"gui_set_spacing(col, 16)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_vbox", "gui_hbox", "gui_set_padding"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
