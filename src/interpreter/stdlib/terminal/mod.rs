//! `std::term` - full terminal control via crossterm.

mod clear;
mod clear_line;
mod common;
mod cursor_col;
mod cursor_row;
mod enter;
mod leave;
mod move_cursor;
mod move_rel;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "term_enter",
    "term_leave",
    "term_clear",
    "term_clear_line",
    "term_move",
    "term_move_up",
    "term_move_down",
    "term_move_left",
    "term_move_right",
    "term_move_to_col",
    "term_move_to_row",
];

pub fn module() -> Module {
    Module::new("term")
        // enter / leave
        .with_raw_function("term_enter", enter::func)
        .with_raw_function("term_leave", leave::func)
        // clear
        .with_raw_function("term_clear", clear::func)
        .with_raw_function("term_clear_line", clear_line::func)
        // absolute cursor
        .with_raw_function("term_move", move_cursor::func)
        .with_raw_function("term_move_to_col", cursor_col::func)
        .with_raw_function("term_move_to_row", cursor_row::func)
        // relative cursor
        .with_raw_function("term_move_up", move_rel::std_term_move_up)
        .with_raw_function("term_move_down", move_rel::std_term_move_down)
        .with_raw_function("term_move_left", move_rel::std_term_move_left)
        .with_raw_function("term_move_right", move_rel::std_term_move_right)
        .with_raw_function("term_next_line", move_rel::std_term_next_line)
        .with_raw_function("term_prev_line", move_rel::std_term_prev_line)
}
