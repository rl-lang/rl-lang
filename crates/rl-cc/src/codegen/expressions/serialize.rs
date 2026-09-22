use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

// Emitters for `std::serialize`. Dynamic inputs (T-typed values) travel
// rl_ok-boxed and dispatch on the result tag in the runtime; concrete
// shapes pass directly. Mirrors the VM value mapping exactly.

fn one_arg(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(")");
    Ok(())
}

fn one_boxed(cc: &mut CCodegen, name: &str, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write(name);
    cc.writer.write("(rl_ok(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write("))");
    Ok(())
}

pub(super) fn compile_json_parse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_json_parse", args)
}

pub(super) fn compile_json_stringify(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_boxed(cc, "rl_serialize_json_stringify", args)
}

pub(super) fn compile_json_stringify_pretty(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_boxed(cc, "rl_serialize_json_stringify_pretty", args)
}

pub(super) fn compile_json_is_valid(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_json_is_valid", args)
}

pub(super) fn compile_json_get(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_serialize_json_get(rl_ok(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write("), ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_csv_parse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_csv_parse", args)
}

pub(super) fn compile_csv_parse_with_delimiter(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    cc.writer.write("rl_serialize_csv_parse_with_delimiter(");
    if !args.is_empty() {
        cc.compile_expr(args[0])?;
    }
    cc.writer.write(", ");
    if args.len() >= 2 {
        cc.compile_expr(args[1])?;
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_csv_stringify(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_csv_stringify", args)
}

pub(super) fn compile_csv_parse_headers(
    cc: &mut CCodegen,
    args: &[ExprId],
) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_csv_parse_headers", args)
}

pub(super) fn compile_toml_parse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_toml_parse", args)
}

pub(super) fn compile_toml_stringify(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_boxed(cc, "rl_serialize_toml_stringify", args)
}

pub(super) fn compile_ini_parse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_ini_parse", args)
}

pub(super) fn compile_ini_stringify(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_boxed(cc, "rl_serialize_ini_stringify", args)
}

pub(super) fn compile_yaml_parse(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_arg(cc, "rl_serialize_yaml_parse", args)
}

pub(super) fn compile_yaml_stringify(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    one_boxed(cc, "rl_serialize_yaml_stringify", args)
}
