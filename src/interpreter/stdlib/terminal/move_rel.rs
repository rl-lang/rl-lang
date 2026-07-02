use crate::interpreter::{
    evaluator::Evaluator,
    stdlib::{
        common::{verr, vnl, vok, vs},
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
    execute!(stdout(), MoveUp(n)).map_err(|e| verr!(vs!(format!("term_move_up(): {}", e))));

    vok!(vnl!())
}

pub fn std_term_move_down(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    execute!(stdout(), MoveDown(n))
        .map_err(|e| verr!(vs!(format!("term_move_down(): {}", e))));

    vok!(vnl!())
}

pub fn std_term_move_left(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    execute!(stdout(), MoveLeft(n))
        .map_err(|e| verr!(vs!(format!("term_move_left(): {}", e))));

    vok!(vnl!())
}

pub fn std_term_move_right(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    execute!(stdout(), MoveRight(n)).map_err(|e| verr!(vs!(format!("term_move_right(): {}", e))));

    vok!(vnl!())
}

pub fn std_term_next_line(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    execute!(stdout(), MoveToNextLine(n))
        .map_err(|e| verr!(vs!(format!("term_next_line(): {}", e))));

    vok!(vnl!())
}

pub fn std_term_prev_line(_: &mut Evaluator, arg: Value) -> Value {
    let n = match extract_u16(arg, "n") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    execute!(stdout(), MoveToPreviousLine(n))
        .map_err(|e| verr!(vs!(format!("term_prev_line(): {}", e))));

    vok!(vnl!())
}
