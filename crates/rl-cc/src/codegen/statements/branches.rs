use crate::codegen::CCodegen;
use rl_ast::statements::Statement;
use rl_utils::errors::Error;

/// `if cond { body } [else ...]` — emits the `if` head and opening brace,
/// leaving the `}` dance to the caller so `else` can chain on one line.
pub(super) fn write_conditional(
    cc: &mut CCodegen,
    if_branch: &Statement,
    else_branch: &Option<Box<Statement>>,
) -> Result<(), Error> {
    use rl_ast::statements::StatementKind;
    if let StatementKind::ConditionalBranch {
        condition, body, ..
    } = &if_branch.kind
    {
        cc.writer.write_indent();
        if let Some(cond) = condition {
            cc.writer.write("if (");
            cc.compile_expr(*cond)?;
            cc.writer.write(") {\n");
        } else {
            cc.writer.write("{\n");
        }
        cc.writer.indent();
        cc.push_scope();
        for s in body {
            cc.compile_statement(s)?;
        }
        cc.pop_scope();
        cc.writer.dedent();
        cc.writer.write_indent();
        if else_branch.is_some() {
            cc.writer.write("} ");
        } else {
            cc.writer.write("}\n");
        }
    }

    if let Some(else_stmt) = else_branch {
        write_else_branch(cc, else_stmt)?;
    }

    Ok(())
}

/// Emits an `else if (...)`, a chained `else` conditional, or a plain
/// `else { ... }` block following an `if`.
fn write_else_branch(cc: &mut CCodegen, else_stmt: &Statement) -> Result<(), Error> {
    use rl_ast::statements::StatementKind;
    match &else_stmt.kind {
        StatementKind::ConditionalBranch {
            condition, body, ..
        } => {
            if let Some(cond) = condition {
                cc.writer.write("else if (");
                cc.compile_expr(*cond)?;
                cc.writer.write(") {\n");
            } else {
                cc.writer.write("else {\n");
            }
            cc.writer.indent();
            cc.push_scope();
            for s in body {
                cc.compile_statement(s)?;
            }
            cc.pop_scope();
            cc.writer.dedent();
            cc.writer.write_indent();
            cc.writer.write("}\n");
        }
        StatementKind::Conditional {
            if_branch,
            else_branch,
        } => {
            cc.writer.write("else ");
            // Recurse but skip the indent since we're already on the } line
            if let StatementKind::ConditionalBranch {
                condition, body, ..
            } = &if_branch.kind
            {
                if let Some(cond) = condition {
                    cc.writer.write("if (");
                    cc.compile_expr(*cond)?;
                    cc.writer.write(") {\n");
                } else {
                    cc.writer.write("{\n");
                }
                cc.writer.indent();
                cc.push_scope();
                for s in body {
                    cc.compile_statement(s)?;
                }
                cc.pop_scope();
                cc.writer.dedent();
                cc.writer.write_indent();
                if else_branch.is_some() {
                    cc.writer.write("} ");
                } else {
                    cc.writer.write("}\n");
                }
                if let Some(inner_else) = else_branch {
                    write_else_branch(cc, inner_else)?;
                }
            }
        }
        _ => {
            cc.writer.write("else {\n");
            cc.writer.indent();
            cc.push_scope();
            cc.compile_statement(else_stmt)?;
            cc.pop_scope();
            cc.writer.dedent();
            cc.writer.write_indent();
            cc.writer.write("}\n");
        }
    }
    Ok(())
}
