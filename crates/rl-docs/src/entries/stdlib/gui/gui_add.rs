use crate::entry::FnEntry;

pub static GUI_ADD: FnEntry = FnEntry {
    signature: "gui_add(box, widget)",
    description: "moves a widget into a container: it leaves the window top level and is positioned by the layout pass from then on. Containers nest freely; adding a window, or a cycle (a container into itself or its own descendant), is an err",
    example: r#"gui_add(col, title)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_vbox", "gui_hbox", "gui_detach"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
