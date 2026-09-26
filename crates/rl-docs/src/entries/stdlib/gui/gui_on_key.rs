use crate::entry::FnEntry;

pub static GUI_ON_KEY: FnEntry = FnEntry {
    signature: "gui_on_key(window, function)",
    description: "registers `function` as the key callback for `window`, replacing any callback set earlier. `function` is called with the key's name (string, e.g. \"Enter\", \"Escape\", \"A\", \"ArrowUp\") and whether it was pressed (`true`) or released (`false`) for every non-repeat key event while `window` has focus. Track held keys with a flag set on press and cleared on release. This is independent of individual widgets' own key handling (e.g. typing into a focused textbox still updates its text as normal) - it's a way to react to keys like Escape that no widget otherwise handles",
    example: r#"get std::gui::gui_window
get std::gui::gui_on_key
get std::gui::gui_close

dec handle window = result_unwrap(gui_window("My App", 400, 300))

gui_on_key(window, fn(string key, bool pressed) {
    if (key == "Escape" and pressed) {
        gui_close(window)?
    }
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, if `window` is an unknown handle, or if `window` isn't a window",
    ),
    see_also: &["gui_window", "gui_on_submit", "gui_on_close"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
