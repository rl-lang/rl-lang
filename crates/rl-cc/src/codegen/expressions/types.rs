use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_to_string_bin_hex_oct(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "to_string" => {
            cc.writer.write("rl_ok(rl_types_to_string(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        "to_bin" => {
            cc.writer.write("rl_ok(rl_types_to_bin(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        "to_hex" => {
            cc.writer.write("rl_ok(rl_types_to_hex(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        "to_oct" => {
            cc.writer.write("rl_ok(rl_types_to_oct(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub(super) fn compile_to_primitive(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "to_int" => {
            cc.writer.write("rl_ok((int64_t)");
            cc.compile_expr(args[0])?;
            cc.writer.write(")");
        }
        "to_float" => {
            cc.writer.write("rl_ok((double)");
            cc.compile_expr(args[0])?;
            cc.writer.write(")");
        }
        "to_bool" => {
            cc.writer.write("rl_ok_bool((");
            cc.compile_expr(args[0])?;
            cc.writer.write(") ? true : false)");
        }
        "to_byte" => {
            cc.writer.write("rl_types_to_byte(rl_ok(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        "to_char" => {
            cc.writer.write("rl_types_to_char(rl_ok(");
            cc.compile_expr(args[0])?;
            cc.writer.write("))");
        }
        _ => unreachable!(),
    }
    Ok(())
}

pub(super) fn compile_error_unwrap(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_types_error_unwrap(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_type_check(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    let c_func = match func_name {
        "is_bool" => "rl_is_bool(",
        "is_int" => "rl_is_int(",
        "is_float" => "rl_is_float(",
        "is_string" => "rl_is_string(",
        "is_null" => "rl_is_null(",
        "is_char" => "rl_is_char(",
        "is_byte" => "rl_is_byte(",
        "is_error" => "rl_is_error(",
        _ => unreachable!(),
    };
    cc.writer.write(c_func);
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_type_of(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_type_of(_Generic((");
    cc.compile_expr(args[0])?;
    cc.writer.write("), int64_t: 1, double: 2, bool: 3, rl_string: 4, default: 0))");
    Ok(())
}

pub(super) fn compile_dbg(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("_Generic((");
    cc.compile_expr(args[0])?;
    cc.writer.write("), int64_t: rl_dbg_int64, double: rl_dbg_float64, bool: rl_dbg_bool, rl_string: rl_dbg_str, default: rl_dbg_int64)(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}
