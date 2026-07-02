//! `std::term` - full terminal control via crossterm.

mod attr;
mod clear;
mod clear_line;
mod common;
mod cursor_col;
mod cursor_row;
mod enter;
mod flush;
mod hide_cursor;
mod leave;
mod mouse;
mod move_cursor;
mod move_rel;
mod named_color;
mod poll;
mod print;
mod read_key;
mod reset_color;
mod restore_cursor;
mod save_cursor;
mod scroll;
mod set_bg;
mod set_fg;
mod set_title;
mod show_cursor;
mod size;
mod sync_output;
mod wrap;

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
    "term_scroll_up",
    "term_scroll_down",
    "term_print",
    "term_flush",
    "term_set_fg",
    "term_set_bg",
    "term_reset_color",
    "term_fg",
    "term_bg",
    "term_bold",
    "term_dim",
    "term_italic",
    "term_underline",
    "term_blink",
    "term_reverse",
    "term_crossed_out",
    "term_reset_attr",
    "term_enable_wrap",
    "term_disable_wrap",
    "term_begin_sync",
    "term_end_sync",
    "term_enable_mouse",
    "term_disable_mouse",
    "term_read_key",
    "term_poll",
];

pub fn module() -> Module {
    Module::new("term")
        // enter / leave
        .with_function("term_enter", enter::func)
        .with_function("term_leave", leave::func)
        // clear
        .with_function("term_clear", clear::func)
        .with_function("term_clear_line", clear_line::func)
        // absolute cursor
        .with_function("term_move", move_cursor::func)
        .with_function("term_move_to_col", cursor_col::func)
        .with_function("term_move_to_row", cursor_row::func)
        // relative cursor
        .with_function("term_move_up", move_rel::std_term_move_up)
        .with_function("term_move_down", move_rel::std_term_move_down)
        .with_function("term_move_left", move_rel::std_term_move_left)
        .with_function("term_move_right", move_rel::std_term_move_right)
        .with_function("term_next_line", move_rel::std_term_next_line)
        .with_function("term_prev_line", move_rel::std_term_prev_line)
        // save / restore
        .with_raw_function("term_save_cursor", save_cursor::func)
        .with_raw_function("term_restore_cursor", restore_cursor::func)
        // show / hide
        .with_function("term_hide_cursor", hide_cursor::func)
        .with_raw_function("term_show_cursor", show_cursor::func)
        // size / title
        .with_raw_function("term_get_size", size::std_term_get_size)
        .with_raw_function("term_set_size", size::std_term_set_size)
        .with_raw_function("term_set_title", set_title::func)
        // scroll
        .with_raw_function("term_scroll_up", scroll::std_term_scroll_up)
        .with_raw_function("term_scroll_down", scroll::std_term_scroll_down)
        // output
        .with_raw_function("term_print", print::func)
        .with_function("term_flush", flush::func)
        // rgb color
        .with_raw_function("term_set_fg", set_fg::func)
        .with_raw_function("term_set_bg", set_bg::func)
        .with_raw_function("term_reset_color", reset_color::func)
        // named color
        .with_raw_function("term_fg", named_color::std_term_fg)
        .with_raw_function("term_bg", named_color::std_term_bg)
        // attributes
        .with_function("term_bold", attr::std_term_bold)
        .with_function("term_dim", attr::std_term_dim)
        .with_function("term_italic", attr::std_term_italic)
        .with_function("term_underline", attr::std_term_underline)
        .with_function("term_blink", attr::std_term_blink)
        .with_function("term_reverse", attr::std_term_reverse)
        .with_function("term_crossed_out", attr::std_term_crossed_out)
        .with_function("term_reset_attr", attr::std_term_reset_attr)
        // line wrap
        .with_raw_function("term_enable_wrap", wrap::std_term_enable_wrap)
        .with_raw_function("term_disable_wrap", wrap::std_term_disable_wrap)
        // synchronized output
        .with_raw_function("term_begin_sync", sync_output::std_term_begin_sync)
        .with_raw_function("term_end_sync", sync_output::std_term_end_sync)
        // mouse
        .with_function("term_enable_mouse", mouse::std_term_enable_mouse)
        .with_function("term_disable_mouse", mouse::std_term_disable_mouse)
        // input
        .with_raw_function("term_read_key", read_key::func)
        .with_raw_function("term_poll", poll::func)
}
