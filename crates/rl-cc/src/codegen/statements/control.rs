use super::propagate::{emit_propagate_assign, emit_propagate_guard};
use super::result_field_access;
use crate::codegen::CCodegen;
use crate::name_mangle::mangle;
use crate::types::type_to_c;
use rl_ast::nodes::ExpressionKind;
use rl_ast::statements::{Statement, StatementKind};
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// A bare expression used as a statement. A `?expr` gets the
/// early-return guard; otherwise the value is simply discarded.
pub(super) fn compile_expr_stmt(cc: &mut CCodegen, expr_id: ExprId) -> Result<(), Error> {
    let expr = cc.ast.exprs.get(expr_id);
    if let ExpressionKind::Propagate(inner) = &expr.kind {
        let temp = emit_propagate_assign(cc, *inner)?;
        emit_propagate_guard(cc, &temp, true)?;
    } else {
        cc.writer.write_indent();
        cc.compile_expr(expr_id)?;
        cc.writer.write(";\n");
    }
    Ok(())
}

/// `return expr` / bare `return`. A `return ?expr` always returns the
/// `rl_result` on failure (never the script-mode exit path), then
/// unwraps the success value for the caller.
pub(super) fn compile_return(cc: &mut CCodegen, ret: Option<ExprId>) -> Result<(), Error> {
    match ret {
        Some(expr_id) => {
            let expr = cc.ast.exprs.get(expr_id);
            if let ExpressionKind::Propagate(inner) = &expr.kind {
                let temp = emit_propagate_assign(cc, *inner)?;
                emit_propagate_guard(cc, &temp, false)?;
                cc.writer.write_indent();
                cc.writer.write(&format!(
                    "return {};\n",
                    result_field_access("rl_result", &temp)
                ));
            } else {
                cc.writer.write_indent();
                cc.writer.write("return ");
                cc.compile_expr(expr_id)?;
                cc.writer.write(";\n");
            }
        }
        None => {
            cc.writer.writeln("return;");
        }
    }
    Ok(())
}

/// `while cond { body }` — maps directly onto a C `while` loop.
pub(super) fn compile_while(
    cc: &mut CCodegen,
    condition: ExprId,
    body: &[Statement],
) -> Result<(), Error> {
    cc.writer.write_indent();
    cc.writer.write("while (");
    cc.compile_expr(condition)?;
    cc.writer.write(") {\n");
    cc.writer.indent();
    for s in body {
        cc.compile_statement(s)?;
    }
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n");
    Ok(())
}

/// `for [init, cond, incr] { body }` — maps onto a C `for` loop.
pub(super) fn compile_for(
    cc: &mut CCodegen,
    initializer: &Statement,
    condition: ExprId,
    increment: ExprId,
    body: &[Statement],
) -> Result<(), Error> {
    cc.writer.write_indent();
    cc.writer.write("for (");
    compile_for_init(cc, initializer)?;
    cc.writer.write("; ");
    cc.compile_expr(condition)?;
    cc.writer.write("; ");
    cc.compile_expr(increment)?;
    cc.writer.write(") {\n");
    cc.writer.indent();
    for s in body {
        cc.compile_statement(s)?;
    }
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n");
    Ok(())
}

/// Compiles a C-style `for` initializer (`int64_t i = 0`) without the
/// trailing semicolon or newline, so it fits inside `for (...; ...; ...)`.
fn compile_for_init(cc: &mut CCodegen, stmt: &Statement) -> Result<(), Error> {
    if let StatementKind::ResolvedVariableDeclaration {
        name,
        type_annotation,
        value,
        ..
    } = &stmt.kind
    {
        let c_type = type_to_c(type_annotation);
        let c_name = mangle(name);
        cc.declare(name, &c_name);
        cc.writer.write(&format!("{} {} = ", c_type, c_name));
        cc.compile_expr(*value)?;
    }
    Ok(())
}

/// `loop { body }` — an infinite `while (1)` loop, exited via `break`.
pub(super) fn compile_loop(cc: &mut CCodegen, body: &[Statement]) -> Result<(), Error> {
    cc.writer.write_indent();
    cc.writer.write("while (1) {\n");
    cc.writer.indent();
    for s in body {
        cc.compile_statement(s)?;
    }
    cc.writer.dedent();
    cc.writer.write_indent();
    cc.writer.write("}\n");
    Ok(())
}
