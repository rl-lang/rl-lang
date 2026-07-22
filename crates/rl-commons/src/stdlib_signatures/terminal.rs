//! Typed signatures for `std::terminal`. Mirrors
//! `rl-interpreter/src/stdlib/terminal/*.rs`.
//!
//! `term_print` stays untyped: it stringifies its argument via
//! `Value::to_string`, so any scalar type is accepted - same reasoning as
//! `io::print`/`io::println`. `term_read_key` also stays untyped: depending
//! on the event read, it resolves to `Result[string]` (key press,
//! `FocusGained`/`FocusLost`) or `Result[array[string]]` (mouse events,
//! resize events) - a fixed signature would either reject or silently
//! accept the wrong branch.

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("term")
        .with_functions(&["term_print"])
        .with_typed_function(term_read_key())
        .with_typed_function(no_arg_result_null("term_enter"))
        .with_typed_function(no_arg_result_null("term_leave"))
        .with_typed_function(no_arg_result_null("term_clear"))
        .with_typed_function(no_arg_result_null("term_clear_line"))
        .with_typed_function(term_move())
        .with_typed_function(one_int_arg_result_null("term_move_to_col"))
        .with_typed_function(one_int_arg_result_null("term_move_to_row"))
        .with_typed_function(one_int_arg_result_null("term_move_up"))
        .with_typed_function(one_int_arg_result_null("term_move_down"))
        .with_typed_function(one_int_arg_result_null("term_move_left"))
        .with_typed_function(one_int_arg_result_null("term_move_right"))
        .with_typed_function(one_int_arg_result_null("term_next_line"))
        .with_typed_function(one_int_arg_result_null("term_prev_line"))
        .with_typed_function(no_arg_result_null("term_save_cursor"))
        .with_typed_function(no_arg_result_null("term_restore_cursor"))
        .with_typed_function(no_arg_result_null("term_hide_cursor"))
        .with_typed_function(no_arg_result_null("term_show_cursor"))
        .with_typed_function(term_get_size())
        .with_typed_function(term_set_size())
        .with_typed_function(term_set_title())
        .with_typed_function(one_int_arg_result_null("term_scroll_up"))
        .with_typed_function(one_int_arg_result_null("term_scroll_down"))
        .with_typed_function(no_arg_result_null("term_flush"))
        .with_typed_function(rgb_fn("term_set_fg"))
        .with_typed_function(rgb_fn("term_set_bg"))
        .with_typed_function(no_arg_result_null("term_reset_color"))
        .with_typed_function(term_named_color("term_fg"))
        .with_typed_function(term_named_color("term_bg"))
        .with_typed_function(no_arg_result_null("term_bold"))
        .with_typed_function(no_arg_result_null("term_dim"))
        .with_typed_function(no_arg_result_null("term_italic"))
        .with_typed_function(no_arg_result_null("term_underline"))
        .with_typed_function(no_arg_result_null("term_blink"))
        .with_typed_function(no_arg_result_null("term_reverse"))
        .with_typed_function(no_arg_result_null("term_crossed_out"))
        .with_typed_function(no_arg_result_null("term_reset_attr"))
        .with_typed_function(no_arg_result_null("term_enable_wrap"))
        .with_typed_function(no_arg_result_null("term_disable_wrap"))
        .with_typed_function(no_arg_result_null("term_begin_sync"))
        .with_typed_function(no_arg_result_null("term_end_sync"))
        .with_typed_function(no_arg_result_null("term_enable_mouse"))
        .with_typed_function(no_arg_result_null("term_disable_mouse"))
        .with_typed_function(term_poll())
}

/// Shared shape for the many `term_*()` functions that take no arguments
/// and either succeed with `null` or raise a runtime error (e.g.
/// `term_enter`, `term_clear`, `term_show_cursor`, the attribute setters).
fn no_arg_result_null(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![]), result(T::Null))])
}

/// Shared shape for the `term_move_*`/`term_scroll_*`/`term_*_line`
/// functions that take a single count/position argument via
/// `terminal::common::extract_u16`, which only accepts `int` (see
/// `extract_int`, not `extract_number`).
fn one_int_arg_result_null(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Int]), result(T::Null))])
}

/// `term_move(x, y) -> Result[null]` - both coordinates go through
/// `extract_u16`, so `int` only.
fn term_move() -> StdFn {
    StdFn::typed(
        "term_move",
        vec![(params(vec![T::Int, T::Int]), result(T::Null))],
    )
}

/// `term_get_size() -> Result[array[int]]` - `(cols, rows)` packed into a
/// `Value::Values` with `items_type: Int`. See
/// `terminal/size.rs::std_term_get_size`.
fn term_get_size() -> StdFn {
    StdFn::typed(
        "term_get_size",
        vec![(params(vec![]), result(T::Array(Box::new(T::Int))))],
    )
}

/// `term_set_size(cols, rows) -> Result[null]` - both go through
/// `extract_u16`, so `int` only.
fn term_set_size() -> StdFn {
    StdFn::typed(
        "term_set_size",
        vec![(params(vec![T::Int, T::Int]), result(T::Null))],
    )
}

/// `term_set_title(title) -> Result[null]`. Note: the implementation reads
/// the title with `terminal::common::extract_byte` (not `extract_string`),
/// so despite the name it accepts `int`/`byte`, not `string` - see
/// `terminal/set_title.rs::func`.
fn term_set_title() -> StdFn {
    StdFn::typed(
        "term_set_title",
        vec![
            (params(vec![T::Int]), result(T::Null)),
            (params(vec![T::Byte]), result(T::Null)),
        ],
    )
}

/// `term_fg(name)` / `term_bg(name) -> Result[null]` - a named color
/// string (`"red"`, `"dark_blue"`, ...). See
/// `terminal/named_color.rs::std_term_fg`/`std_term_bg`.
fn term_named_color(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), result(T::Null))])
}

/// `term_set_fg(r, g, b)` / `term_set_bg(r, g, b) -> Result[null]` - each
/// RGB component independently goes through
/// `terminal::common::extract_byte`, which accepts `int` or `byte` (see
/// `extract_number`), so all eight `int`/`byte` combinations are valid.
fn rgb_fn(name: &'static str) -> StdFn {
    let component = [T::Byte, T::Int];
    let mut signatures = Vec::with_capacity(8);
    for r in &component {
        for g in &component {
            for b in &component {
                signatures.push((
                    params(vec![r.clone(), g.clone(), b.clone()]),
                    result(T::Null),
                ));
            }
        }
    }
    StdFn::typed(name, signatures)
}

/// `term_poll(ms) -> Result[bool]` - `ms` goes through
/// `stdlib::common::extract_number`, which accepts `int` or `byte`. See
/// `terminal/poll.rs::func`.
fn term_poll() -> StdFn {
    StdFn::typed(
        "term_poll",
        vec![
            (params(vec![T::Int]), result(T::Bool)),
            (params(vec![T::Byte]), result(T::Bool)),
        ],
    )
}

/// `term_read_key() -> Result[array[string]]` - every branch (key press,
/// mouse event, resize, focus gained/lost) packs its fields into a
/// `Value::Values` of strings.
fn term_read_key() -> StdFn {
    StdFn::typed(
        "term_read_key",
        vec![(params(vec![]), result(T::Array(Box::new(T::String))))],
    )
}
