use crate::entry::FnEntry;

pub static GUI_SET_COLOR: FnEntry = FnEntry {
    signature: "gui_set_color(widget, r, g, b)",
    description: "sets the foreground/text color of a widget as RGB bytes (0-255)",
    example: r#"get std::gui::gui_set_color

dec handle lbl = gui_label(win, "Hello", 10, 10)?
gui_set_color(lbl, 255, 100, 50)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Returns err if the handle is not a valid widget"),
    see_also: &["gui_set_bg_color", "gui_set_font_size"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
