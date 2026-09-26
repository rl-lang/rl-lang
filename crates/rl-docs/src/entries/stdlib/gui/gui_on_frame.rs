use crate::entry::FnEntry;

pub static GUI_ON_FRAME: FnEntry = FnEntry {
    signature: "gui_on_frame(window, function)",
    description: "registers `function` as a per-frame callback for `window`: called with no arguments every frame while set, after widgets render. Enables polling patterns (worker threads, animations). Implies continuous repaint while set; pass `null` instead of a function to unset",
    example: r#"get __spawn, __poll from core
get gui_on_frame from std::gui

dec int job = __spawn("get __emit from core\n__emit(\"half\")\n\"done\"")
gui_on_frame(main, fn() {
    dec r = __poll(job)
    if (!is_err(r)) {
        gui_set_text(status, result_unwrap(r))?
    }
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda/null, if `window` is an unknown handle, or if `window` isn't a window",
    ),
    see_also: &["gui_on_close", "gui_on_key", "__spawn", "__poll"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
