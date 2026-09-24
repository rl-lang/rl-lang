use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_ast::statements::TypeAnnotation;
use rl_utils::errors::Error;

// Emitters for `std::cli`. Every call maps onto one `rl_cli_*` runtime
// function with the same value shapes as the VM: spec/option arrays travel
// as `rl_array`, messages as `rl_string`, results as `rl_result`.

fn one_arg(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_parse_args(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_cli_parse_args", args)
}

pub(super) fn compile_parse_args_or_exit(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_cli_parse_args_or_exit", args)
}

pub(super) fn compile_usage_string(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_cli_usage", args)
}

pub(super) fn compile_prompt(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_cli_prompt", args)
}

pub(super) fn compile_prompt_password(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_cli_prompt_password", args)
}

pub(super) fn compile_prompt_confirm(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_cli_prompt_confirm", args)
}

pub(super) fn compile_prompt_choice(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_cli_prompt_choice(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_shell_split(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_cli_shell_split", args)
}

pub(super) fn compile_shell_join(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // Runtime returns a bare string, like the VM.
    one_arg(cc, "rl_cli_shell_join", args)
}

pub(super) fn compile_read_line_editable(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_cli_read_line_editable", args)
}

pub(super) fn compile_read_line_with_history(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    // Bare tuple like the VM: deref the single-element array payload into
    // the program tuple struct (same convention as result_unwrap of
    // tuples). Field order matches `_rl_tuple_sarr` in the runtime.
    let tname = cc.ensure_tuple_type(vec![
        TypeAnnotation::String,
        TypeAnnotation::Array(Box::new(TypeAnnotation::String)),
    ]);
    cc.writer
        .write(&format!("(({tname}*)rl_result_unwrap_arr(rl_cli_read_line_with_history("));
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(")).data)[0]");
    Ok(())
}

pub(super) fn compile_progress_bar(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_cli_progress_bar(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(", ");
    if args.len() >= 3 {
        cc.compile_expr(args[2])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_spinner_tick(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_cli_spinner_tick", args)
}
