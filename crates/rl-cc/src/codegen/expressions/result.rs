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
            // String messages wrap with rl_err_msg, codes with rl_err.
            let is_string = matches!(
                cc.inferred_expr_type(args[0]).as_ref(),
                Some(rl_ast::statements::TypeAnnotation::String)
                    | Some(rl_ast::statements::TypeAnnotation::CString)
            );
            cc.writer.write(if is_string { "rl_err_msg(" } else { "rl_err(" });
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
        // Cast to bool so generic printing says `true`, not `1`.
        "is_err" => {
            cc.writer.write("(bool)(!");
            cc.compile_expr(args[0])?;
            cc.writer.write(".is_ok)");
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub(super) fn compile_unwrap(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    // result_unwrap_err extracts the error payload and aborts on ok
    // values (mirroring the VM); result_unwrap does the inverse.
    // Tuple payloads travel as one element arrays; deref element zero.
    if func_name == "result_unwrap" && !args.is_empty()
        && let Some(fields) = cc.tuple_payload_fields(args[0]) {
            let tname = cc.ensure_tuple_type(fields);
            cc.writer.write(&format!("(({0}*)rl_result_unwrap_arr(", tname));
            cc.compile_expr(args[0])?;
            cc.writer.write(").data)[0]");
            return Ok(());
        }
    let unwrap_fn = if func_name == "result_unwrap_err" {
        match cc.unwrap_fn_for_result(args[0]) {
            "rl_result_unwrap_str" => "rl_result_unwrap_err_str",
            "rl_result_unwrap_f64" => "rl_result_unwrap_err_f64",
            "rl_result_unwrap_bool" => "rl_result_unwrap_err_bool",
            _ => "rl_result_unwrap_err_i64",
        }
    } else {
        cc.unwrap_fn_for_result(args[0])
    };
    cc.writer.write(&format!("{unwrap_fn}("));
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_unwrap_or(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    if !args.is_empty()
        && let Some(fields) = cc.tuple_payload_fields(args[0]) {
            let tname = cc.ensure_tuple_type(fields);
            cc.writer.write("(");
            cc.compile_expr(args[0])?;
            cc.writer.write(&format!(".is_ok ? (({0}*)rl_result_unwrap_arr(", tname));
            cc.compile_expr(args[0])?;
            cc.writer.write(").data)[0] : ");
            cc.compile_expr(args[1])?;
            cc.writer.write(")");
            return Ok(());
        }
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
    let tuple_fields = if !args.is_empty() {
        cc.tuple_payload_fields(args[0])
    } else {
        None
    };
    if let Some(fields) = tuple_fields {
        let tname = cc.ensure_tuple_type(fields);
        cc.writer.write("(");
        cc.compile_expr(args[0])?;
        cc.writer.write(&format!(".is_ok ? (({0}*)rl_result_unwrap_arr(", tname));
        cc.compile_expr(args[0])?;
        cc.writer.write(").data)[0] : ");
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
            cc.writer.write(&format!("(({0}*)rl_result_unwrap_arr(rl_closure_call(", tname));
            cc.compile_expr(args[1])?;
            cc.writer.write(", (rl_result[]){ ");
            cc.compile_expr(args[0])?;
            cc.writer.write(" }, 1)).data)[0]");
        } else {
            cc.writer.write(&format!("(({0}*)rl_result_unwrap_arr(rl_err(-1)).data)[0]", tname));
        }
        cc.writer.write(")");
        return Ok(());
    }
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
