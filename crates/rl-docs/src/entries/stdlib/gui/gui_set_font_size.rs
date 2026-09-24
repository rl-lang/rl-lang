use crate::entry::FnEntry;

pub static GUI_SET_FONT_SIZE: FnEntry = FnEntry {
    signature: "gui_set_font_size(widget, size)",
    description: "sets the font size in points for a widget's text",
    example: r#"get std::gui::gui_set_font_size

dec handle btn = gui_button(win, "Click", 10, 10)?
gui_set_font_size(btn, 18.0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Returns err if the handle is not a valid widget"),
    see_also: &["gui_set_color", "gui_set_tooltip"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
