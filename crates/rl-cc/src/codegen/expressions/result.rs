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

/// `result_unwrap_or_else(v, f)`: the ok payload, else `f(err)` unwrapped
/// to the payload type. A non-callable fallback aborts loudly (the VM
/// raises there); a callable one runs through the closure machinery.
pub(super) fn compile_unwrap_or_else(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    use rl_ast::nodes::ExpressionKind;
    let unwrap_fn = cc.unwrap_fn_for_result(args[0]);
    cc.writer.write("(");
    cc.compile_expr(args[0])?;
    cc.writer.write(&format!(".is_ok ? {unwrap_fn}("));
    cc.compile_expr(args[0])?;
    cc.writer.write(") : ");
    let callable = if args.len() >= 2 {
        let f_expr = cc.ast.exprs.get(args[1]);
        match &f_expr.kind {
            ExpressionKind::ResolvedLambda { .. } => true,
            ExpressionKind::ResolvedIdentifier { name, .. } => matches!(
                cc.var_types.get(name),
                Some(rl_ast::statements::TypeAnnotation::Fn)
                    | Some(rl_ast::statements::TypeAnnotation::Callback(_, _))
            ),
            _ => false,
        }
    } else {
        false
    };
    if callable {
        cc.writer.write(&format!("{unwrap_fn}(rl_closure_call("));
        cc.compile_expr(args[1])?;
        cc.writer.write(", (rl_result[]){ ");
        cc.compile_expr(args[0])?;
        cc.writer.write(" }, 1))");
    } else {
        // Static fallback value where a callback belongs: the VM raises
        // when it tries to call it, so abort with the same loudness.
        cc.writer.write(&format!("{unwrap_fn}(rl_err(-1))"));
    }
    cc.writer.write(")");
    Ok(())
}

/// `result_and_then(v, f)`: errors pass through, ok values run through
/// `f` (which itself returns a result).
pub(super) fn compile_and_then(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    use rl_utils::errors::{Error, Reason};
    use rl_utils::span::Span;
    if args.len() < 2 {
        return Err(Error::at(
            Reason::Compile,
            "result_and_then needs a result and a callback",
            Span::dummy(),
        ));
    }
    cc.writer.write("(");
    cc.compile_expr(args[0])?;
    cc.writer.write(").is_ok ? rl_closure_call(");
    cc.compile_expr(args[1])?;
    cc.writer.write(", (rl_result[]){ ");
    cc.compile_expr(args[0])?;
    cc.writer.write(" }, 1) : (");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}
