use crate::entry::FnEntry;

pub static GUI_ON_MOUSE_MOVE: FnEntry = FnEntry {
    signature: "gui_on_mouse_move(window, function)",
    description: "registers `function` as the mouse-move callback for `window`, replacing any callback set earlier. `function` is called with `(x, y)` logical pixels on every mouse movement while `window` has focus",
    example: r#"get std::gui::gui_window
get std::gui::gui_on_mouse_move
get std::gui::gui_set_text

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle pos = result_unwrap(gui_label(window, "mouse: -", 20, 20))

gui_on_mouse_move(window, fn(int x, int y) {
    gui_set_text(pos, format("mouse: {},{}", x, y))?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, if `window` is an unknown handle, or if `window` isn't a window",
    ),
    see_also: &["gui_window", "gui_on_key", "gui_on_frame"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
