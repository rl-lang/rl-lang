use crate::codegen::CCodegen;
use rl_ast::statements::MatchPattern;
use rl_ast::ExprId;
use rl_utils::errors::Error;

/// `match value { lit => ..., _ => ... }` — lowers to an `if / else if /
/// else` chain comparing the value against each literal arm.
pub(super) fn compile_match(
    cc: &mut CCodegen,
    value: ExprId,
    arms: &[(MatchPattern, Vec<rl_ast::statements::Statement>)],
) -> Result<(), Error> {
    for (i, (pattern, body)) in arms.iter().enumerate() {
        match pattern {
            MatchPattern::Literal(lit_id) => {
                cc.writer.write_indent();
                if i == 0 {
                    cc.writer.write("if (");
                } else {
                    cc.writer.write("else if (");
                }
                cc.compile_expr(value)?;
                cc.writer.write(" == ");
                cc.compile_expr(*lit_id)?;
                cc.writer.write(") {\n");
                cc.writer.indent();
                for s in body {
                    cc.compile_statement(s)?;
                }
                cc.writer.dedent();
                cc.writer.write_indent();
                cc.writer.write("}\n");
            }
            MatchPattern::Wildcard => {
                cc.writer.write_indent();
                cc.writer.write("else {\n");
                cc.writer.indent();
                for s in body {
                    cc.compile_statement(s)?;
                }
                cc.writer.dedent();
                cc.writer.write_indent();
                cc.writer.write("}\n");
            }
        }
    }
    Ok(())
}
