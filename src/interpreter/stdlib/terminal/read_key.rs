use crate::ast::statements::TypeAnnotation;
use crate::interpreter::stdlib::common::{verr, vok, vs};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind, read};

pub fn func(_: &mut Evaluator) -> Value {
    loop {
        match read() {
            Ok(Event::Key(key)) => {
                let s = match key.code {
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
                    KeyCode::Insert => "Inseeval".into(),
                    KeyCode::F(n) => format!("F{n}"),
                    KeyCode::Null => "Null".into(),
                    _ => "Unknown".into(),
                };
                return vok!(vs!(s));
            }

            // mouse events
            Ok(Event::Mouse(m)) => {
                let kind = match m.kind {
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
                return vok!(Value::Values {
                    items_type: TypeAnnotation::String,
                    items: vec![vs!(kind), vs!(m.column.to_string()), vs!(m.row.to_string()),],
                });
            }
            // other events
            Ok(Event::Resize(cols, rows)) => {
                return vok!(Value::Values {
                    items_type: TypeAnnotation::String,
                    items: vec![
                        vs!("Resize".into()),
                        vs!(cols.to_string()),
                        vs!(rows.to_string()),
                    ],
                });
            }
            Ok(Event::FocusGained) => return vok!(vs!("FocusGained".into())),
            Ok(Event::FocusLost) => return vok!(vs!("FocusLost".into())),

            Err(e) => return verr!(vs!(format!("term_read_key(): {}", e))),
            _ => continue,
        }
    }
}
