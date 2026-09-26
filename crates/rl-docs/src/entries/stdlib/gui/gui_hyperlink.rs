use crate::entry::FnEntry;

pub static GUI_HYPERLINK: FnEntry = FnEntry {
    signature: "gui_hyperlink(window, text, url, x, y)",
    description: "creates a hyperlink at (x, y): blue underlined `text` that opens `url` in the system browser when clicked. No RL callback fires - opening is egui built-in",
    example: r#"dec handle link = result_unwrap(gui_hyperlink(main, "rl-lang", "https://github.com/rl-lang/rl-lang", 20, 16))"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_window", "gui_button", "gui_label"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
