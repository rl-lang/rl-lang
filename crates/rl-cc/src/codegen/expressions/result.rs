use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_result_wrap(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "ok" => {
            cc.writer.write("rl_ok(");
            cc.compile_expr(args[0])?;
            cc.writer.write(")");
        }
        "err" => {
            cc.writer.write("rl_err(");
            cc.compile_expr(args[0])?;
            cc.writer.write(")");
        }
        "error" => {
            cc.writer.write("rl_error(");
            cc.compile_expr(args[0])?;
            cc.writer.write(")");
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub(super) fn compile_is_ok_err(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "is_ok" => {
            cc.compile_expr(args[0])?;
            cc.writer.write(".is_ok");
        }
        "is_err" => {
            cc.writer.write("!");
            cc.compile_expr(args[0])?;
            cc.writer.write(".is_ok");
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub(super) fn compile_unwrap(cc: &mut CCodegen, _func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let unwrap_fn = cc.unwrap_fn_for_result(args[0]);
    cc.writer.write(&format!("{unwrap_fn}("));
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_unwrap_or(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    let unwrap_fn = cc.unwrap_fn_for_result(args[0]);
    cc.writer.write("(");
    cc.compile_expr(args[0])?;
    cc.writer.write(".is_ok ? ");
    cc.writer.write(&format!("{unwrap_fn}("));
    cc.compile_expr(args[0])?;
    cc.writer.write(") : ");
    cc.compile_expr(args[1])?;
    cc.writer.write(")");
    Ok(())
}
