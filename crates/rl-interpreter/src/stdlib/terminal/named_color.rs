use crate::stdlib::common::{extract_string, try_fn, verr, vs};
use crate::{evaluator::Evaluator, values::Value};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
};
use std::io::stdout;

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

pub fn std_term_fg(_: &mut Evaluator, arg: Value) -> Value {
    let name = match extract_string(arg, "term_fg") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    let color = match parse_color(&name) {
        Some(v) => v,
        None => return verr!(vs!(format!("term_fg(): unknown color \"{}\"", name))),
    };

    try_fn!("term_fg", execute!(stdout(), SetForegroundColor(color)));

    Value::Ok(Box::new(Value::Null))
}

pub fn std_term_bg(_: &mut Evaluator, arg: Value) -> Value {
    let name = match extract_string(arg, "term_fg") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    let color = match parse_color(&name) {
        Some(v) => v,
        None => return verr!(vs!(format!("term_fg(): unknown color \"{}\"", name))),
    };

    try_fn!("term_bg", execute!(stdout(), SetBackgroundColor(color)));

    Value::Ok(Box::new(Value::Null))
}
