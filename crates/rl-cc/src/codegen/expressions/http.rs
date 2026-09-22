use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_server_start(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_server_start(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_server_recv(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_server_recv(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_server_try_recv(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_server_try_recv(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_server_stop(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_server_stop(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_request_method(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_request_method(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_request_url(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_request_url(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_request_header(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_request_header(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_request_body(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_request_body(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_respond(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_respond(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    cc.writer.write(", ");
    cc.compile_expr(args[2])?;
    if args.len() >= 4 {
        cc.writer.write(", ");
        cc.compile_expr(args[3])?;
        cc.writer.write(", 1");
    } else {
        cc.writer.write(", rl_str_literal(\"\", 0), 0");
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_get(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_get(");
    cc.compile_expr(args[0])?;
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_post(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_post(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    if args.len() >= 3 {
        cc.writer.write(", ");
        cc.compile_expr(args[2])?;
        cc.writer.write(", 1");
    } else {
        cc.writer.write(", rl_str_literal(\"text/plain\", 10), 0");
    }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_request(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_http_request(");
    cc.compile_expr(args[0])?;
    cc.writer.write(", ");
    cc.compile_expr(args[1])?;
    if args.len() >= 3 {
        cc.writer.write(", ");
        cc.compile_expr(args[2])?;
        cc.writer.write(", 1");
    } else {
        cc.writer.write(", rl_str_literal(\"\", 0), 0");
    }
    if args.len() >= 4 {
        cc.writer.write(", ");
        cc.compile_expr(args[3])?;
        cc.writer.write(", 1");
    } else {
        cc.writer.write(", rl_str_literal(\"\", 0), 0");
    }
    cc.writer.write(")");
    Ok(())
}
