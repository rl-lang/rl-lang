use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_rand_simple(cc: &mut CCodegen, func_name: &str) -> Result<(), Error> {
    let c_call = match func_name {
        "rand_int" => "rl_rand_int()",
        "rand_float" => "rl_rand_float()",
        "rand_bool" => "rl_rand_bool()",
        "rand_char" => "rl_rand_char()",
        "rand_byte" => "rl_rand_byte()",
        _ => "rl_rand_int()",
    };
    cc.writer.write(c_call);
    Ok(())
}

pub(super) fn compile_rand_bool_weighted(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_rand_bool_weighted(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_rand_range(
    cc: &mut CCodegen,
    func_name: &str,
    args: &[ExprId],
) -> Result<(), Error> {
    match func_name {
        "rand_int_range" => {
            cc.writer.write("rl_ok(rl_rand_int_range(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        "rand_float_range" => {
            cc.writer.write("rl_ok(rl_rand_float_range(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        "rand_dice" => {
            cc.writer.write("rl_ok(rl_rand_dice(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        "rand_range" => {
            cc.writer.write("rl_ok(rl_rand_range(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_rand_range_step(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_rand_range_step(");
    if args.len() >= 1 { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_rand_string(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_ok(rl_rand_string(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_rand_collection(
    cc: &mut CCodegen,
    func_name: &str,
    args: &[ExprId],
) -> Result<(), Error> {
    match func_name {
        "rand_dices" => {
            cc.writer.write("rl_ok(rl_rand_dices(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        "rand_bytes" => {
            cc.writer.write("rl_ok(rl_rand_bytes(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        "rand_choice" => {
            cc.writer.write("rl_ok(rl_rand_choice(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        "rand_shuffle" => {
            cc.writer.write("rl_ok(rl_rand_shuffle(");
            if !args.is_empty() { cc.compile_expr(args[0])?; }
            cc.writer.write("))");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_rand_multi(
    cc: &mut CCodegen,
    func_name: &str,
    args: &[ExprId],
) -> Result<(), Error> {
    match func_name {
        "rand_choices" => {
            cc.writer.write("rl_ok(rl_rand_choices(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        "rand_sample" => {
            cc.writer.write("rl_ok(rl_rand_sample(");
            if args.len() >= 1 { cc.compile_expr(args[0])?; }
            cc.writer.write(", ");
            if args.len() >= 2 { cc.compile_expr(args[1])?; }
            cc.writer.write("))");
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn compile_rand_seed(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    // VM reseeds its Xoshiro; the C backend reseeds its own RNG so the
    // later sequence is deterministic for a given seed.
    cc.writer.write("rl_rand_seed(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}
