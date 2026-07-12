use crate::docs::entry::{FnEntry, StdEntry};

mod term_begin_sync;
mod term_bg;
mod term_blink;
mod term_bold;
mod term_clear;
mod term_clear_line;
mod term_crossed_out;
mod term_dim;
mod term_disable_mouse;
mod term_disable_wrap;
mod term_enable_mouse;
mod term_enable_wrap;
mod term_end_sync;
mod term_enter;
mod term_fg;
mod term_flush;
mod term_get_size;
mod term_hide_cursor;
mod term_italic;
mod term_leave;
mod term_move;
mod term_move_down;
mod term_move_left;
mod term_move_right;
mod term_move_to_col;
mod term_move_to_row;
mod term_move_up;
mod term_next_line;
mod term_poll;
mod term_prev_line;
mod term_print;
mod term_read_key;
mod term_reset_attr;
mod term_reset_color;
mod term_restore_cursor;
mod term_reverse;
mod term_save_cursor;
mod term_scroll_down;
mod term_scroll_up;
mod term_set_bg;
mod term_set_fg;
mod term_set_size;
mod term_set_title;
mod term_show_cursor;
mod term_underline;

use term_begin_sync::TERM_BEGIN_SYNC;
use term_bg::TERM_BG;
use term_blink::TERM_BLINK;
use term_bold::TERM_BOLD;
use term_clear::TERM_CLEAR;
use term_clear_line::TERM_CLEAR_LINE;
use term_crossed_out::TERM_CROSSED_OUT;
use term_dim::TERM_DIM;
use term_disable_mouse::TERM_DISABLE_MOUSE;
use term_disable_wrap::TERM_DISABLE_WRAP;
use term_enable_mouse::TERM_ENABLE_MOUSE;
use term_enable_wrap::TERM_ENABLE_WRAP;
use term_end_sync::TERM_END_SYNC;
use term_enter::TERM_ENTER;
use term_fg::TERM_FG;
use term_flush::TERM_FLUSH;
use term_get_size::TERM_GET_SIZE;
use term_hide_cursor::TERM_HIDE_CURSOR;
use term_italic::TERM_ITALIC;
use term_leave::TERM_LEAVE;
use term_move::TERM_MOVE;
use term_move_down::TERM_MOVE_DOWN;
use term_move_left::TERM_MOVE_LEFT;
use term_move_right::TERM_MOVE_RIGHT;
use term_move_to_col::TERM_MOVE_TO_COL;
use term_move_to_row::TERM_MOVE_TO_ROW;
use term_move_up::TERM_MOVE_UP;
use term_next_line::TERM_NEXT_LINE;
use term_poll::TERM_POLL;
use term_prev_line::TERM_PREV_LINE;
use term_print::TERM_PRINT;
use term_read_key::TERM_READ_KEY;
use term_reset_attr::TERM_RESET_ATTR;
use term_reset_color::TERM_RESET_COLOR;
use term_restore_cursor::TERM_RESTORE_CURSOR;
use term_reverse::TERM_REVERSE;
use term_save_cursor::TERM_SAVE_CURSOR;
use term_scroll_down::TERM_SCROLL_DOWN;
use term_scroll_up::TERM_SCROLL_UP;
use term_set_bg::TERM_SET_BG;
use term_set_fg::TERM_SET_FG;
use term_set_size::TERM_SET_SIZE;
use term_set_title::TERM_SET_TITLE;
use term_show_cursor::TERM_SHOW_CURSOR;
use term_underline::TERM_UNDERLINE;

pub static TERM: StdEntry = StdEntry {
    name: "term",
    description: "functions for full terminal control (cursor, color, input, screen)",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &TERM_ENTER,
    &TERM_LEAVE,
    &TERM_CLEAR,
    &TERM_CLEAR_LINE,
    &TERM_MOVE,
    &TERM_MOVE_UP,
    &TERM_MOVE_DOWN,
    &TERM_MOVE_LEFT,
    &TERM_MOVE_RIGHT,
    &TERM_MOVE_TO_COL,
    &TERM_MOVE_TO_ROW,
    &TERM_NEXT_LINE,
    &TERM_PREV_LINE,
    &TERM_SAVE_CURSOR,
    &TERM_RESTORE_CURSOR,
    &TERM_HIDE_CURSOR,
    &TERM_SHOW_CURSOR,
    &TERM_GET_SIZE,
    &TERM_SET_SIZE,
    &TERM_SET_TITLE,
    &TERM_SCROLL_UP,
    &TERM_SCROLL_DOWN,
    &TERM_PRINT,
    &TERM_FLUSH,
    &TERM_SET_FG,
    &TERM_SET_BG,
    &TERM_RESET_COLOR,
    &TERM_FG,
    &TERM_BG,
    &TERM_BOLD,
    &TERM_DIM,
    &TERM_ITALIC,
    &TERM_UNDERLINE,
    &TERM_BLINK,
    &TERM_REVERSE,
    &TERM_CROSSED_OUT,
    &TERM_RESET_ATTR,
    &TERM_ENABLE_WRAP,
    &TERM_DISABLE_WRAP,
    &TERM_BEGIN_SYNC,
    &TERM_END_SYNC,
    &TERM_ENABLE_MOUSE,
    &TERM_DISABLE_MOUSE,
    &TERM_READ_KEY,
    &TERM_POLL,
];
