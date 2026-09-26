use crate::entry::FnEntry;

pub static GUI_SPINNER: FnEntry = FnEntry {
    signature: "gui_spinner(window, x, y)",
    description: "creates a loading spinner at (x, y): pure animation, no text, no interaction. Show one while workers run; remove it when done",
    example: r#"dec handle spin = result_unwrap(gui_spinner(main, 590, 16))"#,
    expected_output: None,
    returns: "result[handle(Gui)]",
    errors: Some("err(string) on bad handles or bad arguments"),
    see_also: &["gui_window", "__spawn", "__poll"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
