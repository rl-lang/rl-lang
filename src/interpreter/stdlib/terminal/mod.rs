//! `std::term` - full terminal control via crossterm.

mod clear;
mod clear_line;
mod common;
mod cursor_col;
mod cursor_row;
mod enter;
mod hide_cursor;
mod leave;
mod move_cursor;
mod move_rel;
mod restore_cursor;
mod save_cursor;
mod set_title;
mod show_cursor;
mod size;

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
    "term_next_line",
    "term_prev_line",
    "term_save_cursor",
    "term_restore_cursor",
    "term_hide_cursor",
    "term_show_cursor",
    "term_get_size",
    "term_set_size",
    "term_set_title",
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
        // save / restore
        .with_raw_function("term_save_cursor", save_cursor::func)
        .with_raw_function("term_restore_cursor", restore_cursor::func)
        // show / hide
        .with_raw_function("term_hide_cursor", hide_cursor::func)
        .with_raw_function("term_show_cursor", show_cursor::func)
        // size / title
        .with_raw_function("term_get_size", size::std_term_get_size)
        .with_raw_function("term_set_size", size::std_term_set_size)
        .with_raw_function("term_set_title", set_title::func)
}
