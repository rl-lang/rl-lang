use crate::entry::FnEntry;

pub static GUI_GET_WINDOW_SIZE: FnEntry = FnEntry {
    signature: "gui_get_window_size(window)",
    description: "returns the window dimensions as [width, height] in logical pixels",
    example: r#"get std::gui::gui_get_window_size

dec handle win = gui_window("App", 800, 600)?
dec arr[float] size = result_unwrap(gui_get_window_size(win))?"#,
    expected_output: Some("[800.0, 600.0]"),
    returns: "result[array[float]]",
    errors: Some("Returns err if the handle is not a window"),
    see_also: &["gui_window", "gui_window_set_size", "gui_get_window_pos"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
