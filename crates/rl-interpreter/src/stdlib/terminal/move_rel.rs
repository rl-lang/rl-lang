use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{try_fn, verr, vnl, vok, vs},
        terminal::common::extract_u16,
    },
    values::Value,
};
use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveToNextLine, MoveToPreviousLine, MoveUp},
    execute,
};
use std::io::stdout;

pub fn std_term_move_up(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    try_fn!("term_move_up", execute!(stdout(), MoveUp(n)));

    vok!(vnl!())
}

pub fn std_term_move_down(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    try_fn!("term_move_down", execute!(stdout(), MoveDown(n)));

    vok!(vnl!())
}

pub fn std_term_move_left(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    try_fn!("term_move_left", execute!(stdout(), MoveLeft(n)));

    vok!(vnl!())
}

pub fn std_term_move_right(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    try_fn!("term_move_right", execute!(stdout(), MoveRight(n)));

    vok!(vnl!())
}

pub fn std_term_next_line(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    try_fn!("term_next_line", execute!(stdout(), MoveToNextLine(n)));

    vok!(vnl!())
}

pub fn std_term_prev_line(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!("term_prev_line", execute!(stdout(), MoveToPreviousLine(n)));

    vok!(vnl!())
}
