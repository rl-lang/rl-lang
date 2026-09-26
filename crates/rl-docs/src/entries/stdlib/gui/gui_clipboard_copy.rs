use crate::entry::FnEntry;

pub static GUI_CLIPBOARD_COPY: FnEntry = FnEntry {
    signature: "gui_clipboard_copy(text)",
    description: "copies `text` to the system clipboard. Needs no window; works outside the event loop",
    example: r#"result_unwrap(gui_clipboard_copy("hello"))"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_clipboard_paste"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
