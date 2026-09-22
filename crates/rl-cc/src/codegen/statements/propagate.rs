use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// Emits `rl_result <temp> = <inner>;` for a `?`-propagated expression and
/// returns the temp variable name.
pub(super) fn emit_propagate_assign(cc: &mut CCodegen, inner: ExprId) -> Result<String, Error> {
    let temp = cc.temp_var();
    cc.writer.write_indent();
    cc.writer.write(&format!("rl_result {} = ", temp));
    cc.compile_expr(inner)?;
    cc.writer.write(";\n");
    Ok(temp)
}

/// Emits the `if (!<temp>.is_ok) { ... }` early-return guard for `?`.
/// Top-level code prints the error and exits the program with code 1;
/// result-returning functions and lambdas return the `rl_result` to the
/// caller (mirroring the VM's propagation). Any other context aborts,
/// which the checker normally prevents by rejecting `?` there.
pub(super) fn emit_propagate_guard(
    cc: &mut CCodegen,
    temp: &str,
    allow_script_exit: bool,
) -> Result<(), Error> {
    use rl_ast::statements::TypeAnnotation;
    cc.writer.write_indent();
    cc.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
    cc.writer.indent();
    cc.writer.write_indent();
    let returns_result = cc.in_lambda_body
        || matches!(
            cc.fn_return.as_ref(),
            Some(TypeAnnotation::Result(_)) | Some(TypeAnnotation::CResult(_))
        );
    if returns_result {
        cc.writer.write(&format!("return {};\n", temp));
    } else if allow_script_exit && cc.fn_return.is_none() {
        cc.writer.write(&format!("rl_println_result({});\n", temp));
        cc.writer.write_indent();
        cc.writer.write("return 1;\n");
    } else {
        cc.writer.write(&format!("rl_println_result({});\n", temp));
        cc.writer.write_indent();
        cc.writer.write("exit(1);\n");
    }
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n");
    Ok(())
}
