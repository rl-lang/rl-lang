use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_term_no_args(cc: &mut CCodegen, func_name: &str) -> Result<(), Error> {
    match func_name {
        "term_enter" | "term_leave" | "term_clear" | "term_clear_line"
        | "term_save_cursor" | "term_restore_cursor" | "term_hide_cursor"
        | "term_show_cursor" | "term_flush" | "term_reset_color"
        | "term_bold" | "term_dim" | "term_italic" | "term_underline"
        | "term_blink" | "term_reverse" | "term_crossed_out" | "term_reset_attr"
        | "term_enable_wrap" | "term_disable_wrap" | "term_begin_sync"
        | "term_end_sync" | "term_enable_mouse" | "term_disable_mouse" => {
            cc.writer.write(&format!("rl_{}()", func_name));
        }
        _ => {
            cc.writer.write(&format!("/* unknown terminal fn: {} */", func_name));
        }
    }
    Ok(())
}

pub(super) fn compile_term_with_args(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "term_move" | "term_set_fg" | "term_set_bg" | "term_fg" | "term_bg"
        | "term_move_to_col" | "term_move_to_row" | "term_move_up"
        | "term_move_down" | "term_move_left" | "term_move_right"
        | "term_next_line" | "term_prev_line" | "term_scroll_up"
        | "term_scroll_down" | "term_set_size" | "term_poll" => {
            cc.writer.write(&format!("rl_{}(", func_name));
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    cc.writer.write(", ");
                }
                cc.compile_expr(*arg)?;
            }
            cc.writer.write(")");
        }
        _ => {
            cc.writer.write(&format!("/* unknown terminal fn: {} */", func_name));
        }
    }
    Ok(())
}

pub(super) fn compile_term_get_size(cc: &mut CCodegen) -> Result<(), Error> {
    // VM returns ok([cols, rows]); the runtime builds that array or an error.
    cc.writer.write("rl_term_size()");
    Ok(())
}

pub(super) fn compile_term_read_key(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_term_read_key()");
    Ok(())
}

pub(super) fn compile_term_str(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "term_set_title" => {
            cc.writer.write("rl_term_set_title(");
            if !args.is_empty() {
                cc.compile_expr(args[0])?;
            }
            cc.writer.write(")");
        }
        "term_print" => {
            cc.writer.write("rl_term_print_inline((rl_fmt_arg)");
            if !args.is_empty() {
                cc.write_arg_as_fmt(args[0])?;
            } else {
                cc.writer.write("{ rl_ok_null(), true }");
            }
            cc.writer.write(")");
        }
        _ => {
            cc.writer.write(&format!("/* unknown terminal str fn: {} */", func_name));
        }
    }
    Ok(())
}
