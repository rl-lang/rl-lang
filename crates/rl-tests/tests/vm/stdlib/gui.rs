use rl_vm::VmValue;

use crate::common::compile_and_run;

// Clipboard access depends on the environment: with a display server the
// calls succeed, headless (or on android, where clipboard is unsupported)
// they return err. These tests only assert the calls return a well-shaped
// result instead of aborting.

#[test]
fn clipboard_copy_returns_result() {
    let result = compile_and_run(
        r#"
get gui_clipboard_copy from std::gui
get is_ok from std::res
dec result[null] r = gui_clipboard_copy("hello")
dec bool x = is_ok(r)?
x
"#,
    )
    .unwrap();
    assert!(matches!(result, VmValue::Bool(_)));
}

#[test]
fn clipboard_paste_returns_result() {
    let result = compile_and_run(
        r#"
get gui_clipboard_paste from std::gui
get is_ok from std::res
dec result[string] r = gui_clipboard_paste()
dec bool x = is_ok(r)?
x
"#,
    )
    .unwrap();
    assert!(matches!(result, VmValue::Bool(_)));
}
