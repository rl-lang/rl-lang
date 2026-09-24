//! `std::term` - full terminal control via crossterm.
//!
//! Every function drives the terminal by writing escape codes to stdout through
//! crossterm and returns a language `result[...]` value. Most return
//! `result[null]`; `term_poll` returns `result[bool]`; `term_get_size` returns
//! `result[array[int]]`; `term_read_key` returns `result[array[string]]`.
//! `term_print` is untyped (it stringifies any scalar, like `io::print`).
//!
//! Numeric/string arguments are read with local `extract_*` helpers that
//! reproduce the exact acceptance rules and error strings of the former
//! `stdlib::common` / `terminal::common` extractors: `extract_int` accepts only
//! `int`, `extract_number` accepts `int` or `byte`, `extract_u16` accepts `int`
//! (and rejects negatives), `extract_byte` accepts `int` or `byte`. Because these
//! inspect the raw runtime value, every function taking them is generic over `R`,
//! takes `R::Value` parameters, and carries an explicit signature.

#[cfg(feature = "impls")]
use crossterm::{
    cursor::{
        Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToNextLine,
        MoveToPreviousLine, MoveToRow, MoveUp, RestorePosition, SavePosition, Show,
    },
    event::{
        DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseButton,
        MouseEventKind, poll, read,
    },
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{
        BeginSynchronizedUpdate, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, ScrollUp,
        SetSize, SetTitle, disable_raw_mode, enable_raw_mode, size,
    },
};
#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use std::io::{Write, stderr, stdout};
#[cfg(feature = "impls")]
use std::time::Duration;

// ---- argument extraction (reproduces the former extractors verbatim) -------

#[cfg(feature = "impls")]
/// `int` only. Mirrors `stdlib::common::extract_int`.
fn extract_int<R: Runtime>(v: R::Value, name: &str) -> Result<i64, String> {
    match R::as_i64(&v) {
        Some(i) => Ok(i),
        None => Err(format!(
            "{}: expected int type, got {}",
            name,
            R::type_name(&v)
        )),
    }
}

#[cfg(feature = "impls")]
/// `int` or `byte`, coerced to `u64`. Mirrors `stdlib::common::extract_number`.
fn extract_number<R: Runtime>(v: R::Value, name: &str) -> Result<u64, String> {
    if let Some(i) = R::as_i64(&v) {
        Ok(i as u64)
    } else if let Some(b) = R::as_u8(&v) {
        Ok(b as u64)
    } else {
        Err(format!(
            "{}: expected int or byte type, got {}",
            name,
            R::type_name(&v)
        ))
    }
}

#[cfg(feature = "impls")]
/// `int` (rejecting negatives), coerced to `u16`. Mirrors
/// `terminal::common::extract_u16`.
fn extract_u16<R: Runtime>(v: R::Value, name: &str) -> Result<u16, String> {
    let v = extract_int::<R>(v, name)?;
    match v {
        v if v >= 0 => Ok(v as u16),
        v => Err(format!("{} must be >= 0, got {}", name, v)),
    }
}

#[cfg(feature = "impls")]
/// `int` or `byte`, coerced to `u8`. Mirrors `terminal::common::extract_byte`.
fn extract_byte<R: Runtime>(v: R::Value, name: &str) -> Result<u8, String> {
    let byte = extract_number::<R>(v, name)?;
    Ok(byte as u8)
}

#[cfg(feature = "impls")]
/// `string` only. Mirrors `stdlib::common::extract_string`.
fn extract_string<R: Runtime>(v: R::Value, name: &str) -> Result<String, String> {
    match R::as_str(&v) {
        Some(s) => Ok(s.to_owned()),
        None => Err(format!(
            "{}: expected string type, got {}",
            name,
            R::type_name(&v)
        )),
    }
}

// ---- enter / leave ---------------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_enter() -> Result<(), String> {
    if let Err(e) = enable_raw_mode() {
        return Err(format!("term_enter: {}", e));
    }
    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        return Err(format!("term_enter: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_leave() -> Result<(), String> {
    let _ = stdout().flush();
    let _ = stderr().flush();
    if let Err(e) = disable_raw_mode() {
        return Err(format!("term_leave: {}", e));
    }
    if let Err(e) = execute!(stdout(), Show, LeaveAlternateScreen) {
        return Err(format!("term_leave: {}", e));
    }
    let _ = stdout().flush();
    Ok(())
}

// ---- clear -----------------------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_clear() -> Result<(), String> {
    match execute!(stdout(), Clear(ClearType::All)) {
        Err(e) => Err(format!("term_clear(): {}", e)),
        Ok(_) => Ok(()),
    }
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_clear_line() -> Result<(), String> {
    match execute!(stdout(), Clear(ClearType::CurrentLine)) {
        Err(e) => Err(format!("term_clear_line(): {}", e)),
        Ok(_) => Ok(()),
    }
}

// ---- absolute cursor -------------------------------------------------------

#[native_fn(module = "term", sig(int, int -> result[null]))]
pub fn term_move<R: Runtime>(_cx: &mut R::Cx, x: R::Value, y: R::Value) -> Result<(), String> {
    let x = extract_u16::<R>(x, "x")?;
    let y = extract_u16::<R>(y, "y")?;
    if let Err(e) = execute!(stdout(), MoveTo(x, y)) {
        return Err(format!("term_move: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_move_to_col<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let col = extract_u16::<R>(arg, "col")?;
    match execute!(stdout(), MoveToColumn(col)) {
        Err(e) => Err(format!("term_move_to_col(): {}", e)),
        Ok(_) => Ok(()),
    }
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_move_to_row<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let row = extract_u16::<R>(arg, "row")?;
    match execute!(stdout(), MoveToRow(row)) {
        Err(e) => Err(format!("term_move_to_row(): {}", e)),
        Ok(_) => Ok(()),
    }
}

// ---- relative cursor -------------------------------------------------------

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_move_up<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), MoveUp(n)) {
        return Err(format!("term_move_up: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_move_down<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), MoveDown(n)) {
        return Err(format!("term_move_down: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_move_left<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), MoveLeft(n)) {
        return Err(format!("term_move_left: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_move_right<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), MoveRight(n)) {
        return Err(format!("term_move_right: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_next_line<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), MoveToNextLine(n)) {
        return Err(format!("term_next_line: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_prev_line<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), MoveToPreviousLine(n)) {
        return Err(format!("term_prev_line: {}", e));
    }
    Ok(())
}

// ---- save / restore --------------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_save_cursor() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), SavePosition) {
        return Err(format!("term_save_cursor: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_restore_cursor() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), RestorePosition) {
        return Err(format!("term_restore_cursor: {}", e));
    }
    Ok(())
}

// ---- show / hide -----------------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_hide_cursor() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), Hide) {
        return Err(format!("term_hide_cursor: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_show_cursor() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), Show) {
        return Err(format!("term_show_cursor: {}", e));
    }
    Ok(())
}

// ---- size / title ----------------------------------------------------------

#[native_fn(module = "term", sig(-> result[array[int]]))]
pub fn term_get_size() -> Result<Vec<i64>, String> {
    let (cols, rows) = match size() {
        Ok((cols, rows)) => (cols, rows),
        Err(e) => return Err(format!("term_get_size(): {}", e)),
    };
    Ok(vec![cols as i64, rows as i64])
}

#[native_fn(module = "term", sig(-> result[array[int]]))]
pub fn term_get_cursor_pos() -> Result<Vec<i64>, String> {
    match crossterm::cursor::position() {
        Ok((x, y)) => Ok(vec![x as i64, y as i64]),
        Err(e) => Err(format!("term_get_cursor_pos(): {}", e)),
    }
}

#[native_fn(module = "term", sig(int, int -> result[null]))]
pub fn term_set_size<R: Runtime>(
    _cx: &mut R::Cx,
    cols: R::Value,
    rows: R::Value,
) -> Result<(), String> {
    let cols = extract_u16::<R>(cols, "cols")?;
    let rows = extract_u16::<R>(rows, "rows")?;
    if let Err(e) = execute!(stdout(), SetSize(cols, rows)) {
        return Err(format!("term_set_size: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(string -> result[null]))]
pub fn term_set_title<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let title = extract_string::<R>(arg, "term_set_title")?;
    if let Err(e) = execute!(stdout(), SetTitle(title)) {
        return Err(format!("term_set_title: {}", e));
    }
    Ok(())
}

// ---- scroll ----------------------------------------------------------------

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_scroll_up<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), ScrollUp(n)) {
        return Err(format!("term_scroll_up: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(int -> result[null]))]
pub fn term_scroll_down<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let n = extract_u16::<R>(arg, "n")?;
    if let Err(e) = execute!(stdout(), ScrollDown(n)) {
        return Err(format!("term_scroll_down: {}", e));
    }
    Ok(())
}

// ---- output ----------------------------------------------------------------

#[native_fn(module = "term", untyped)]
pub fn term_print<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let text = R::display(&arg);
    if let Err(e) = execute!(stdout(), Print(text)) {
        return Err(format!("term_print: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_flush() -> Result<(), String> {
    if let Err(e) = stdout().flush() {
        return Err(format!("term_flush: {}", e));
    }
    Ok(())
}

// ---- rgb color -------------------------------------------------------------

// NOTE: the `g` component is extracted under the name `"r"`; preserved for
// parity with the original.
#[native_fn(module = "term",
    product([byte, int], [byte, int], [byte, int] -> result[null]))]
pub fn term_set_fg<R: Runtime>(
    _cx: &mut R::Cx,
    r: R::Value,
    g: R::Value,
    b: R::Value,
) -> Result<(), String> {
    let r = extract_byte::<R>(r, "r")?;
    let b = extract_byte::<R>(b, "b")?;
    let g = extract_byte::<R>(g, "g")?;
    if let Err(e) = execute!(stdout(), SetForegroundColor(Color::Rgb { r, g, b })) {
        return Err(format!("term_set_fg: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term",
    product([byte, int], [byte, int], [byte, int] -> result[null]))]
pub fn term_set_bg<R: Runtime>(
    _cx: &mut R::Cx,
    r: R::Value,
    g: R::Value,
    b: R::Value,
) -> Result<(), String> {
    let r = extract_byte::<R>(r, "r")?;
    let b = extract_byte::<R>(b, "b")?;
    let g = extract_byte::<R>(g, "g")?;
    if let Err(e) = execute!(stdout(), SetBackgroundColor(Color::Rgb { r, g, b })) {
        return Err(format!("term_set_bg: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_reset_color() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), ResetColor) {
        return Err(format!("term_reset_color: {}", e));
    }
    Ok(())
}

// ---- named color -----------------------------------------------------------

#[cfg(feature = "impls")]
fn parse_color(s: &str) -> Option<Color> {
    match s {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),
        "dark_black" => Some(Color::DarkGrey),
        "dark_red" => Some(Color::DarkRed),
        "dark_green" => Some(Color::DarkGreen),
        "dark_yellow" => Some(Color::DarkYellow),
        "dark_blue" => Some(Color::DarkBlue),
        "dark_magenta" => Some(Color::DarkMagenta),
        "dark_cyan" => Some(Color::DarkCyan),
        "grey" => Some(Color::Grey),
        _ => None,
    }
}

// NOTE: reads the argument under the name `"term_fg"`; preserved for parity.
#[native_fn(module = "term", sig(string -> result[null]))]
pub fn term_fg<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let name = extract_string::<R>(arg, "term_fg")?;
    let color = match parse_color(&name) {
        Some(v) => v,
        None => return Err(format!("term_fg(): unknown color \"{}\"", name)),
    };
    if let Err(e) = execute!(stdout(), SetForegroundColor(color)) {
        return Err(format!("term_fg: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(string -> result[null]))]
pub fn term_bg<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<(), String> {
    let name = extract_string::<R>(arg, "term_bg")?;
    let color = match parse_color(&name) {
        Some(v) => v,
        None => return Err(format!("term_bg(): unknown color \"{}\"", name)),
    };
    if let Err(e) = execute!(stdout(), SetBackgroundColor(color)) {
        return Err(format!("term_bg: {}", e));
    }
    Ok(())
}

// ---- attributes ------------------------------------------------------------

#[cfg(feature = "impls")]
fn set_attr(attr: Attribute, name: &str) -> Result<(), String> {
    match execute!(stdout(), SetAttribute(attr)) {
        Err(e) => Err(format!("{}(): {}", name, e)),
        Ok(_) => Ok(()),
    }
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_bold() -> Result<(), String> {
    set_attr(Attribute::Bold, "term_bold")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_dim() -> Result<(), String> {
    set_attr(Attribute::Dim, "term_dim")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_italic() -> Result<(), String> {
    set_attr(Attribute::Italic, "term_italic")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_underline() -> Result<(), String> {
    set_attr(Attribute::Underlined, "term_underline")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_blink() -> Result<(), String> {
    set_attr(Attribute::SlowBlink, "term_blink")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_reverse() -> Result<(), String> {
    set_attr(Attribute::Reverse, "term_reverse")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_crossed_out() -> Result<(), String> {
    set_attr(Attribute::CrossedOut, "term_crossed_out")
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_reset_attr() -> Result<(), String> {
    set_attr(Attribute::Reset, "term_reset_attr")
}

// ---- line wrap -------------------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_enable_wrap() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), EnableLineWrap) {
        return Err(format!("term_enable_wrap: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_disable_wrap() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), DisableLineWrap) {
        return Err(format!("term_disable_wrap: {}", e));
    }
    Ok(())
}

// ---- synchronized output ---------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_begin_sync() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), BeginSynchronizedUpdate) {
        return Err(format!("term_begin_sync: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_end_sync() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), EndSynchronizedUpdate) {
        return Err(format!("term_end_sync: {}", e));
    }
    Ok(())
}

// ---- mouse -----------------------------------------------------------------

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_enable_mouse() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), EnableMouseCapture) {
        return Err(format!("term_enable_mouse: {}", e));
    }
    Ok(())
}

#[native_fn(module = "term", sig(-> result[null]))]
pub fn term_disable_mouse() -> Result<(), String> {
    if let Err(e) = execute!(stdout(), DisableMouseCapture) {
        return Err(format!("term_disable_mouse: {}", e));
    }
    Ok(())
}

// ---- input -----------------------------------------------------------------

#[native_fn(module = "term", sig(-> result[array[string]]))]
pub fn term_read_key() -> Result<Vec<String>, String> {
    loop {
        match read() {
            Ok(Event::Key(key)) => {
                let s: String = match key.code {
                    KeyCode::Char(c) => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            format!("Ctrl:{c}")
                        } else {
                            format!("Char:{c}")
                        }
                    }
                    KeyCode::Enter => "Enter".into(),
                    KeyCode::Esc => "Esc".into(),
                    KeyCode::Backspace => "Backspace".into(),
                    KeyCode::Delete => "Delete".into(),
                    KeyCode::Tab => "Tab".into(),
                    KeyCode::BackTab => "BackTab".into(),
                    KeyCode::Up => "Up".into(),
                    KeyCode::Down => "Down".into(),
                    KeyCode::Left => "Left".into(),
                    KeyCode::Right => "Right".into(),
                    KeyCode::Home => "Home".into(),
                    KeyCode::End => "End".into(),
                    KeyCode::PageUp => "PageUp".into(),
                    KeyCode::PageDown => "PageDown".into(),
                    KeyCode::Insert => "Insert".into(),
                    KeyCode::F(n) => format!("F{n}"),
                    KeyCode::Null => "Null".into(),
                    _ => "Unknown".into(),
                };
                return Ok(vec![s]);
            }

            // mouse events
            Ok(Event::Mouse(m)) => {
                let kind: String = match m.kind {
                    MouseEventKind::Down(MouseButton::Left) => "MouseLeft".into(),
                    MouseEventKind::Down(MouseButton::Right) => "MouseRight".into(),
                    MouseEventKind::Down(MouseButton::Middle) => "MouseMiddle".into(),
                    MouseEventKind::Up(_) => "MouseUp".into(),
                    MouseEventKind::Drag(_) => "MouseDrag".into(),
                    MouseEventKind::Moved => "MouseMove".into(),
                    MouseEventKind::ScrollUp => "ScrollUp".into(),
                    MouseEventKind::ScrollDown => "ScrollDown".into(),
                    _ => "MouseUnknown".into(),
                };
                return Ok(vec![kind, m.column.to_string(), m.row.to_string()]);
            }
            // other events
            Ok(Event::Resize(cols, rows)) => {
                return Ok(vec![
                    "Resize".to_string(),
                    cols.to_string(),
                    rows.to_string(),
                ]);
            }
            Ok(Event::FocusGained) => {
                return Ok(vec!["FocusGained".to_string()]);
            }
            Ok(Event::FocusLost) => {
                return Ok(vec!["FocusLost".to_string()]);
            }

            Err(e) => return Err(format!("term_read_key(): {}", e)),
            _ => continue,
        }
    }
}

#[native_fn(module = "term", sig(int -> result[bool]), sig(byte -> result[bool]))]
pub fn term_poll<R: Runtime>(_cx: &mut R::Cx, arg: R::Value) -> Result<bool, String> {
    let ms = extract_number::<R>(arg, "ms")?;
    match poll(Duration::from_millis(ms)) {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("term_poll(): {}", e)),
    }
}

rl_std_core::native_module!("term";
    funcs: [
        // enter / leave
        term_enter, term_leave,
        // clear
        term_clear, term_clear_line,
        // absolute cursor
        term_move, term_move_to_col, term_move_to_row,
        // relative cursor
        term_move_up, term_move_down, term_move_left, term_move_right,
        term_next_line, term_prev_line,
        // save / restore
        term_save_cursor, term_restore_cursor,
        // show / hide
        term_hide_cursor, term_show_cursor,
        // size / title
        term_get_size, term_set_size, term_set_title,
        term_get_cursor_pos,
        // scroll
        term_scroll_up, term_scroll_down,
        // output
        term_print, term_flush,
        // rgb color
        term_set_fg, term_set_bg, term_reset_color,
        // named color
        term_fg, term_bg,
        // attributes
        term_bold, term_dim, term_italic, term_underline, term_blink,
        term_reverse, term_crossed_out, term_reset_attr,
        // line wrap
        term_enable_wrap, term_disable_wrap,
        // synchronized output
        term_begin_sync, term_end_sync,
        // mouse
        term_enable_mouse, term_disable_mouse,
        // input
        term_read_key, term_poll,
    ],
);
