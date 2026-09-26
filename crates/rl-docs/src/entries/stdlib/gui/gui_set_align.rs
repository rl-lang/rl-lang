use crate::entry::FnEntry;

pub static GUI_SET_ALIGN: FnEntry = FnEntry {
    signature: "gui_set_align(box, align)",
    description: "sets cross-axis alignment: start/left/top, center/middle, or end/right/bottom. Alignment measures against the widest/tallest child from last frame's sizes",
    example: r#"gui_set_align(col, "center")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_vbox", "gui_hbox"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
