use crate::codegen::CCodegen;
use rl_ast::{nodes::ExpressionKind, ExprId};
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;

pub(super) fn compile_arr_closure(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let expr = cc.ast.exprs.get(args[1]);
    if let ExpressionKind::ResolvedLambda { .. } = &expr.kind {
        cc.writer.write(&format!("rl_{func_name}_closure("));
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                cc.writer.write(", ");
            }
            cc.compile_expr(*arg)?;
        }
        cc.writer.write(")");
    } else {
        return Err(Error::at(Reason::Compile, "expected lambda for closure", Span::dummy()));
    }
    Ok(())
}

pub(super) fn compile_result_closure(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let expr = cc.ast.exprs.get(args[1]);
    if let ExpressionKind::ResolvedLambda { .. } = &expr.kind {
        let c_func = match func_name {
            "result_map" => "rl_result_map_closure(",
            "result_map_err" => "rl_result_map_err_closure(",
            _ => unreachable!(),
        };
        cc.writer.write(c_func);
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                cc.writer.write(", ");
            }
            cc.compile_expr(*arg)?;
        }
        cc.writer.write(")");
    } else {
        return Err(Error::at(Reason::Compile, "expected lambda for closure", Span::dummy()));
    }
    Ok(())
}

pub(super) fn compile_bench(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_bench_closure(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write(")");
    Ok(())
}
