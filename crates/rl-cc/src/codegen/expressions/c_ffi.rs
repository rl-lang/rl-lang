use crate::codegen::CCodegen;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_utils::errors::Error;

pub(super) fn compile_c_compile(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_c_compile(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_c_load(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_c_load(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_c_has_symbol(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_c_has_symbol(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_c_close(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_c_close(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_c_clear_cache(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_c_clear_cache()");
    Ok(())
}

pub(super) fn compile_c_call(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_c_call(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write(", ");

    let tuple_expr = cc.ast.exprs.get(args[2]);
    match &tuple_expr.kind {
        ExpressionKind::TupleLiteral(elems) => {
            cc.writer.write(&format!("{}, ", elems.len()));
            cc.writer.write("(void*[]){");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 { cc.writer.write(", "); }
                cc.writer.write("(void*)");
                cc.compile_expr(*elem)?;
            }
            cc.writer.write("}");
        }
        _ => {
            cc.compile_expr(args[2])?;
        }
    }

    cc.writer.write(", ");

    let types_expr = cc.ast.exprs.get(args[3]);
    match &types_expr.kind {
        ExpressionKind::ArrayLiteral(elems) => {
            cc.writer.write("(const char*[]){");
            for (i, elem) in elems.iter().enumerate() {
                if i > 0 { cc.writer.write(", "); }
                let expr = cc.ast.exprs.get(*elem);
                if let ExpressionKind::String(s) = &expr.kind {
                    cc.writer.write(&format!("\"{}\"", s));
                } else {
                    cc.writer.write("\"int64_t\"");
                }
            }
            cc.writer.write("}");
        }
        _ => {
            cc.compile_expr(args[3])?;
        }
    }

    cc.writer.write(", ");
    cc.compile_expr(args[4])?;
    cc.writer.write(")");
    Ok(())
}
