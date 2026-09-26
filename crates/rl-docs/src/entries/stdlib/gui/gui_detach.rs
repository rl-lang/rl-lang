use crate::entry::FnEntry;

pub static GUI_DETACH: FnEntry = FnEntry {
    signature: "gui_detach(widget)",
    description: "moves a widget out of its container back to the window top level, keeping its last laid-out position. Errs when the widget is in no container",
    example: r#"gui_detach(title)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_add", "gui_remove"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
