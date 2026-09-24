use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_assert(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("if (!");
    cc.compile_expr(args[0])?;
    cc.writer.write(") { rl_assert_fail_msg(rl_str_literal(\"assert\", 6), rl_str_literal(\"\", 0)); }");
    Ok(())
}

pub(super) fn compile_assert_cmp(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let (op, label, label_len) = match func_name {
        "assert_eq" => ("!=", "assert_eq", 8),
        "assert_ne" => ("==", "assert_ne", 8),
        "assert_lt" => (">=", "assert_lt", 8),
        "assert_le" => (">", "assert_le", 8),
        "assert_gt" => ("<=", "assert_gt", 8),
        "assert_ge" => ("<", "assert_ge", 8),
        _ => unreachable!(),
    };
    cc.writer.write("if ((");
    cc.compile_expr(args[0])?;
    cc.writer.write(&format!(") {op} ("));
    cc.compile_expr(args[1])?;
    cc.writer.write(")) { rl_assert_fail(rl_str_literal(\"");
    cc.writer.write(label);
    cc.writer.write(&format!("\", {label_len}), "));
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write("); }");
    Ok(())
}

pub(super) fn compile_assert_approx_eq(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("if (fabs((double)(");
    cc.compile_expr(args[0])?;
    cc.writer.write(") - (double)(");
    cc.compile_expr(args[1])?;
    cc.writer.write(")) > 1e-9) { rl_assert_fail(rl_str_literal(\"assert_approx_eq\", 15), ");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write("); }");
    Ok(())
}

pub(super) fn compile_panic(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_panic(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_unreachable(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_unreachable()");
    Ok(())
}

pub(super) fn compile_todo(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_todo()");
    Ok(())
}

pub(super) fn compile_warn(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // VM takes a string and returns bare null; the C helper prints to
    // stderr and returns void, matching the statement use.
    cc.writer.write("rl_debug_warn(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_stack_trace(cc: &mut CCodegen) -> Result<(), Error> {
    // VM returns a bare string with the captured backtrace.
    cc.writer.write("rl_debug_stack_trace()");
    Ok(())
}
