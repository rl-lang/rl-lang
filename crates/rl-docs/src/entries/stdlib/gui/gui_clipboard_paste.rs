use crate::entry::FnEntry;

pub static GUI_CLIPBOARD_PASTE: FnEntry = FnEntry {
    signature: "gui_clipboard_paste()",
    description: "reads the system clipboard, or an err when it is empty or unavailable. Needs no window. Reads anything you copied, including passwords - only run scripts you trust. Not supported on android (always errs there)",
    example: r#"dec string s = result_unwrap(gui_clipboard_paste())"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("err(string) when the clipboard is empty, there is no display server, or on android"),
    see_also: &["gui_clipboard_copy"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
