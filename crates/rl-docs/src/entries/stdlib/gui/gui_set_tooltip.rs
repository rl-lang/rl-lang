use crate::entry::FnEntry;

pub static GUI_SET_TOOLTIP: FnEntry = FnEntry {
    signature: "gui_set_tooltip(widget, text)",
    description: "sets a hover tooltip that appears when the mouse rests on a widget",
    example: r#"get std::gui::gui_set_tooltip

dec handle btn = gui_button(win, "OK", 10, 10)?
gui_set_tooltip(btn, "Click to confirm")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Returns err if the handle is not a valid widget"),
    see_also: &["gui_set_text", "gui_on_click"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
