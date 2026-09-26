use crate::entry::FnEntry;

pub static GUI_CLIPBOARD_PASTE: FnEntry = FnEntry {
    signature: "gui_clipboard_paste()",
    description: "reads the system clipboard, or an err when it is empty or unavailable. Needs no window",
    example: r#"dec string s = result_unwrap(gui_clipboard_paste())"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_clipboard_copy"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
