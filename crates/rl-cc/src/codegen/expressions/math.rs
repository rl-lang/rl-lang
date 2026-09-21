use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_trig(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(&format!("{}(", func_name));
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_single_ok(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(&format!("rl_ok({}(", func_name));
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_abs(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_math_abs(rl_ok(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_hypot(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("hypot(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_atan2(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("atan2(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_pow(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_math_pow(rl_ok(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write("), rl_ok(");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_log(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok((log(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(") / log(");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")))");
    Ok(())
}

pub(super) fn compile_radians(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" * (M_PI / 180.0))");
    Ok(())
}

pub(super) fn compile_degrees(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" * (180.0 / M_PI))");
    Ok(())
}

pub(super) fn compile_sign(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("((");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" > 0) - (");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(" < 0))");
    Ok(())
}

pub(super) fn compile_lerp(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(" + (");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(" - ");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(") * ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_map_range(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("(((");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(" - ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(") / (");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(" - ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")) * (");
    if args.len() >= 4 { cc.compile_expr(args[3])?; }
    cc.writer.write(" - ");
    if args.len() >= 5 { cc.compile_expr(args[4])?; }
    cc.writer.write(") + ");
    if args.len() >= 5 { cc.compile_expr(args[4])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_mod(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(" % ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_min_max_clamp(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok((");
    match func_name {
        "max" => {
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" > ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(" ? ");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" : ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
        }
        "min" => {
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" < ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(" ? ");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" : ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
        }
        "clamp" => {
            // clamp(value, min, max): value < min ? min : value > max ? max : value
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" < ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(" ? ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(" : ");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" > ");
            if args.len() >= 3 { cc.compile_expr(args[2])?; }
            cc.writer.write(" ? ");
            if args.len() >= 3 { cc.compile_expr(args[2])?; }
            cc.writer.write(" : ");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
        }
        _ => {}
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_math_runtime(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(&format!("rl_math_{}(", func_name));
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { cc.writer.write(", "); }
        cc.compile_expr(*arg)?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_bitwise(cc: &mut CCodegen, func_name: &str, args: &[ExprId]) -> Result<(), Error> {
    match func_name {
        "bit_and" => {
            cc.writer.write("rl_ok(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" & ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "bit_or" => {
            cc.writer.write("rl_ok(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" | ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "bit_xor" => {
            cc.writer.write("rl_ok(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" ^ ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "bit_not" => {
            cc.writer.write("rl_ok(~(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        "bit_shift_left" => {
            cc.writer.write("rl_ok(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" << ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        "bit_shift_right" => {
            cc.writer.write("rl_ok(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(" >> ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write(")");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_count_bits(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(__builtin_popcountll(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_leading_zeros(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(__builtin_clzll(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_trailing_zeros(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(__builtin_ctzll(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_constant(cc: &mut CCodegen, func_name: &str) -> Result<(), Error> {
    let c_val = match func_name {
        "PI" => "M_PI",
        "E" => "M_E",
        "TAU" => "(2.0 * M_PI)",
        "PHI" => "1.618033988749895",
        "INF" => "INFINITY",
        "NAN" => "NAN",
        "FRAC_1_PI" => "(1.0 / M_PI)",
        "FRAC_1_SQRT_2" => "(1.0 / M_SQRT2)",
        "FRAC_2_PI" => "(2.0 / M_PI)",
        "FRAC_2_SQRT_PI" => "(2.0 / sqrt(M_PI))",
        "FRAC_PI_2" => "(M_PI / 2.0)",
        "FRAC_PI_3" => "(M_PI / 3.0)",
        "FRAC_PI_4" => "(M_PI / 4.0)",
        "FRAC_PI_6" => "(M_PI / 6.0)",
        "FRAC_PI_8" => "(M_PI / 8.0)",
        "SQRT_2" => "M_SQRT2",
        "LN_2" => "M_LN2",
        "LN_10" => "M_LN10",
        "LOG2_E" => "M_LOG2E",
        "LOG2_10" => "3.321928094887362",
        "LOG10_2" => "M_LOG10E",
        "LOG10_E" => "0.4342944819032518",
        "EULER_GAMMA" => "0.5772156649015329",
        _ => "0",
    };
    cc.writer.write(c_val);
    Ok(())
}

pub(super) fn compile_is_inf(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("isinf(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_is_nan(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("isnan(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}
