use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_to_string_bin_hex_oct(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    // The runtime dispatches on the result tag; rl_ok passes results
    // through and wraps bare values.
    let c_func = match func_name {
        "to_string" => "rl_types_to_string(rl_ok(",
        "to_bin" => "rl_types_to_bin(rl_ok(",
        "to_hex" => "rl_types_to_hex(rl_ok(",
        "to_oct" => "rl_types_to_oct(rl_ok(",
        _ => unreachable!(),
    };
    cc.writer.write(c_func);
    cc.compile_expr(args[0])?;
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_to_primitive(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    // The runtime dispatches on the result tag (parsing strings,
    // truncating floats); rl_ok passes results through as-is.
    let c_func = match func_name {
        "to_int" => "rl_to_int(rl_ok(",
        "to_float" => "rl_to_float(rl_ok(",
        "to_bool" => "rl_to_bool(rl_ok(",
        "to_byte" => "rl_types_to_byte(rl_ok(",
        "to_char" => "rl_types_to_char(rl_ok(",
        _ => unreachable!(),
    };
    cc.writer.write(c_func);
    cc.compile_expr(args[0])?;
    cc.writer.write("))");
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
    // Statically known types fold to a string constant; dynamic values
    // dispatch on the result tag at runtime.
    if let Some(name) = cc.static_type_name(args[0]) {
        cc.writer.write(&format!("rl_str_literal(\"{}\", {})", name, name.len()));
    } else {
        cc.writer.write("rl_type_of_result(rl_ok(");
        cc.compile_expr(args[0])?;
        cc.writer.write("))");
    }
    Ok(())
}

pub(super) fn compile_dbg(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // Primitives print through the fast _Generic path and keep their C
    // value; everything else goes through the runtime passthrough.
    let primitive = matches!(
        cc.inferred_expr_type(args[0]).as_ref(),
        Some(
            rl_ast::statements::TypeAnnotation::Int
                | rl_ast::statements::TypeAnnotation::CInt
                | rl_ast::statements::TypeAnnotation::Float
                | rl_ast::statements::TypeAnnotation::CFloat
                | rl_ast::statements::TypeAnnotation::Bool
                | rl_ast::statements::TypeAnnotation::CBool
                | rl_ast::statements::TypeAnnotation::String
                | rl_ast::statements::TypeAnnotation::CString
        )
    );
    if primitive {
        cc.writer.write("_Generic((");
        cc.compile_expr(args[0])?;
        cc.writer.write("), int64_t: rl_dbg_int64, double: rl_dbg_float64, bool: rl_dbg_bool, rl_string: rl_dbg_str, default: rl_dbg_int64)(");
        cc.compile_expr(args[0])?;
        cc.writer.write(")");
    } else {
        cc.writer.write("rl_dbg_value(rl_ok(");
        cc.compile_expr(args[0])?;
        cc.writer.write("))");
    }
    Ok(())
}
