use crate::codegen::CCodegen;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_ast::statements::TypeAnnotation;
use rl_utils::errors::Error;

pub(super) fn compile_print(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let is_ln = func_name == "println";
    if args.is_empty() {
        if is_ln {
            cc.writer.write("rl_println(\"\")");
        } else {
            cc.writer.write("rl_print(\"\")");
        }
    } else {
        let arg = &args[0];
        let expr = cc.ast.exprs.get(*arg);
        if let ExpressionKind::ResolvedIdentifier { name, .. } = &expr.kind {
            if let Some(ta) = cc.var_types.get(name) {
                match ta {
                    TypeAnnotation::Tuple(elems) => {
                        let c_name = cc.lookup(name);
                        let tuple_name = cc.lookup_tuple_name(elems).to_string();
                        let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                        cc.writer.write(&format!("{}_{}({})", print_fn, tuple_name, c_name));
                    }
                    TypeAnnotation::Record(rname) => {
                        let c_name = cc.lookup(name);
                        let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                        cc.writer.write(&format!("{}_rl_Record_{}({})", print_fn, rname, c_name));
                    }
                    TypeAnnotation::Enum(ename) | TypeAnnotation::CEnum(ename) => {
                        let c_name = cc.lookup(name);
                        let print_fn = if is_ln { "rl_println" } else { "rl_print" };
                        cc.writer.write(&format!("{}_Enum_{}({})", print_fn, ename, c_name));
                    }
                    _ if cc.nullable_vars.contains(name) => {
                        let c_fn = if is_ln { "rl_println_raw" } else { "rl_print_raw" };
                        let c_name = cc.lookup(name);
                        cc.writer.write(&format!("{}({})", c_fn, c_name));
                    }
                    _ => {
                        let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                        cc.writer.write(&format!("{}(", c_fn));
                        cc.compile_expr(*arg)?;
                        cc.writer.write(")");
                    }
                }
            } else {
                let c_fn = if is_ln { "rl_println" } else { "rl_print" };
                cc.writer.write(&format!("{}(", c_fn));
                cc.compile_expr(*arg)?;
                cc.writer.write(")");
            }
        } else {
            let c_fn = if is_ln { "rl_println" } else { "rl_print" };
            cc.writer.write(&format!("{}(", c_fn));
            cc.compile_expr(*arg)?;
            cc.writer.write(")");
        }
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
    cc.writer.write("rl_ok(rl_io_delete_file(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
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
