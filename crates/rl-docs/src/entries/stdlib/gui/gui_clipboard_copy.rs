use crate::entry::FnEntry;

pub static GUI_CLIPBOARD_COPY: FnEntry = FnEntry {
    signature: "gui_clipboard_copy(text)",
    description: "copies `text` to the system clipboard. Needs no window; works outside the event loop. Not supported on android (always errs there). Paste can read anything you copied, including passwords - only paste into programs you trust",
    example: r#"result_unwrap(gui_clipboard_copy("hello"))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) when there is no display server, on android, or the clipboard is unavailable"),
    see_also: &["gui_clipboard_paste"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
