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
/// In script mode the error is printed and the program exits with code 1,
/// otherwise the `rl_result` is returned to the caller.
/// When `allow_script_exit` is false (e.g. inside `return ?expr`),
/// the error is always returned, even in script mode.
pub(super) fn emit_propagate_guard(
    cc: &mut CCodegen,
    temp: &str,
    allow_script_exit: bool,
) -> Result<(), Error> {
    cc.writer.write_indent();
    cc.writer.write(&format!("if (!{}.is_ok) {{\n", temp));
    cc.writer.indent();
    cc.writer.write_indent();
    if allow_script_exit && cc.is_script_mode {
        cc.writer.write(&format!("rl_println_result({});\n", temp));
        cc.writer.write_indent();
        cc.writer.write("return 1;\n");
    } else {
        cc.writer.write(&format!("return {};\n", temp));
    }
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n");
    Ok(())
}
