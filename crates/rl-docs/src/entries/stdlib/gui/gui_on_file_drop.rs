use crate::entry::FnEntry;

pub static GUI_ON_FILE_DROP: FnEntry = FnEntry {
    signature: "gui_on_file_drop(window, function)",
    description: "registers `function` for files dropped onto `window`: called once per drop with `arr[string]` paths",
    example: r#"gui_on_file_drop(main, fn(arr[string] paths) {
    gui_set_text(drop_label, paths[0])?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_window", "gui_on_key", "gui_on_scroll"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
