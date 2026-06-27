use crate::interpreter::stdlib::common::{check_arity, extract_string};
use crate::interpreter::{evaluator::Evaluator, values::Value};
use crate::utils::{errors::Error, span::Span};
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

pub fn std_term_fg(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "term_fg", span)?;

    let name = extract_string(args[0].clone(), "term_fg", span)?;

    let color = parse_color(&name)
        .ok_or_else(|| eval.err(format!("term_fg(): unknown color \"{}\"", name), span))?;
    execute!(stdout(), SetForegroundColor(color))
        .map_err(|e| eval.err(format!("term_fg(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}

pub fn std_term_bg(eval: &mut Evaluator, args: Vec<Value>, span: Span) -> Result<Value, Error> {
    check_arity(&args, 1, "term_bg", span)?;

    let name = extract_string(args[0].clone(), "term_bg", span)?;

    let color = parse_color(&name)
        .ok_or_else(|| eval.err(format!("term_bg(): unknown color \"{}\"", name), span))?;
    execute!(stdout(), SetBackgroundColor(color))
        .map_err(|e| eval.err(format!("term_bg(): {}", e), span))?;
    Ok(Value::Ok(Box::new(Value::Null)))
}
