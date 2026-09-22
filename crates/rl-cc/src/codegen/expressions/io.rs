use crate::codegen::CCodegen;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_ast::statements::TypeAnnotation;
use rl_utils::errors::Error;

/// Emits one print argument with a type-directed printer: tuples and
/// records use their shape functions, nullable values print raw, and
/// everything else goes through the generic macro.
fn print_one(cc: &mut CCodegen, is_ln: bool, arg: ExprId) -> Result<(), Error> {
    let expr = cc.ast.exprs.get(arg);
    // Nullable identifiers print their payload raw.
    if let ExpressionKind::ResolvedIdentifier { name, .. } = &expr.kind
        && cc.nullable_vars.contains(name)
    {
        let c_fn = if is_ln { "rl_println_raw" } else { "rl_print_raw" };
        let c_name = cc.lookup(name);
        cc.writer.write(&format!("{}({})", c_fn, c_name));
        return Ok(());
    }
    match cc.inferred_expr_type(arg).as_ref() {
        Some(TypeAnnotation::Result(inner)) | Some(TypeAnnotation::CResult(inner)) => {
            match inner.as_ref() {
                TypeAnnotation::Array(arr_inner) | TypeAnnotation::CArray(arr_inner) => {
                    match arr_inner.as_ref() {
                        TypeAnnotation::Tuple(elems) | TypeAnnotation::CTuple(elems) => {
                            let elems: Vec<TypeAnnotation> = elems.as_ref().clone();
                            if elems.iter().any(|t| crate::codegen::CCodegen::needs_inference(t)) {
                                let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                                cc.writer.write(&format!("{}(", c_fn));
                                cc.compile_expr(arg)?;
                                cc.writer.write(")");
                                return Ok(());
                            }
                            let arr_fn = cc.ensure_tuple_array_printer(elems);
                            let print_fn = if is_ln { arr_fn.1 } else { arr_fn.0 };
                            cc.writer.write(&format!("{}(rl_result_unwrap_arr(", print_fn));
                            cc.compile_expr(arg)?;
                            cc.writer.write("))");
                            return Ok(());
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            let c_fn = if is_ln { "rl_println" } else { "rl_print" };
            cc.writer.write(&format!("{}(", c_fn));
            cc.compile_expr(arg)?;
            cc.writer.write(")");
        }
        Some(TypeAnnotation::Tuple(elems)) | Some(TypeAnnotation::CTuple(elems)) => {
            let elems: Vec<TypeAnnotation> = elems.as_ref().clone();
            let tuple_name = cc.ensure_tuple_type(elems);
            let print_fn = if is_ln { "rl_println" } else { "rl_print" };
            cc.writer.write(&format!("{}_{}(", print_fn, tuple_name));
            cc.compile_expr(arg)?;
            cc.writer.write(")");
        }
        Some(TypeAnnotation::Array(inner)) | Some(TypeAnnotation::CArray(inner)) => {
            match inner.as_ref() {
                TypeAnnotation::Tuple(elems) | TypeAnnotation::CTuple(elems) => {
                    let elems: Vec<TypeAnnotation> = elems.as_ref().clone();
                    // Generic fields cannot shape a printer; fall back.
                    if elems.iter().any(|t| crate::codegen::CCodegen::needs_inference(t)) {
                        let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                        cc.writer.write(&format!("{}(", c_fn));
                        cc.compile_expr(arg)?;
                        cc.writer.write(")");
                        return Ok(());
                    }
                    let arr_fn = cc.ensure_tuple_array_printer(elems);
                    let print_fn = if is_ln { arr_fn.1 } else { arr_fn.0 };
                    cc.writer.write(&format!("{}(", print_fn));
                    cc.compile_expr(arg)?;
                    cc.writer.write(")");
                }
                _ => {
                    let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                    cc.writer.write(&format!("{}(", c_fn));
                    cc.compile_expr(arg)?;
                    cc.writer.write(")");
                }
            }
        }
        Some(TypeAnnotation::Record(rname)) | Some(TypeAnnotation::CRecord(rname)) => {
            let print_fn = if is_ln { "rl_println" } else { "rl_print" };
            cc.writer.write(&format!("{}_rl_Record_{}(", print_fn, rname));
            cc.compile_expr(arg)?;
            cc.writer.write(")");
        }
        Some(TypeAnnotation::Enum(ename)) | Some(TypeAnnotation::CEnum(ename)) => {
            let print_fn = if is_ln { "rl_println" } else { "rl_print" };
            cc.writer.write(&format!("{}_Enum_{}(", print_fn, ename));
            cc.compile_expr(arg)?;
            cc.writer.write(")");
        }
        _ => {
            let c_fn = if is_ln { "rl_println" } else { "rl_print" };
            cc.writer.write(&format!("{}(", c_fn));
            cc.compile_expr(arg)?;
            cc.writer.write(")");
        }
    }
    Ok(())
}

pub(super) fn compile_print(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let is_ln = func_name == "println";
    if args.is_empty() {
        if is_ln {
            cc.writer.write("rl_println(\"\")");
        } else {
            cc.writer.write("rl_print(\"\")");
        }
        return Ok(());
    }
    // Variadic like the VM: every argument prints in order, println adds
    // one trailing newline.
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            cc.writer.write(", ");
        }
        print_one(cc, false, *arg)?;
    }
    if is_ln {
        cc.writer.write(", printf(\"\\n\")");
    }
    Ok(())
}

pub(super) fn compile_read_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_read_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_read_lines(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_read_lines(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_read_bytes(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_read_bytes(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_read(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_io_read())");
    Ok(())
}

pub(super) fn compile_read_int(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_io_read_int())");
    Ok(())
}

pub(super) fn compile_read_float(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_io_read_float())");
    Ok(())
}

pub(super) fn compile_write_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_write_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_append_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_append_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_delete_file(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_delete_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_isatty(cc: &mut CCodegen) -> Result<(), Error> {
    cc.writer.write("rl_io_isatty()");
    Ok(())
}

pub(super) fn compile_eprint(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_eprint(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_eprintln(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_io_eprintln(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}
