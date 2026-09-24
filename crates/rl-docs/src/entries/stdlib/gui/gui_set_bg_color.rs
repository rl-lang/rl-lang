use crate::entry::FnEntry;

pub static GUI_SET_BG_COLOR: FnEntry = FnEntry {
    signature: "gui_set_bg_color(widget, r, g, b)",
    description: "sets the background color of a widget as RGB bytes (0-255)",
    example: r#"get std::gui::gui_set_bg_color

dec handle btn = gui_button(win, "Click", 10, 10)?
gui_set_bg_color(btn, 50, 50, 50)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Returns err if the handle is not a valid widget"),
    see_also: &["gui_set_color", "gui_window_set_background"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
