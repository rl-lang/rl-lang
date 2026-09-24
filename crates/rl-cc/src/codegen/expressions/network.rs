use crate::codegen::CCodegen;
use rl_ast::ExprId;
use rl_utils::errors::Error;

pub(super) fn compile_tcp_listen(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_listen(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_accept(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_accept(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_connect(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_connect(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_read(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_read(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_write(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_write(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_peer_addr(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_peer_addr(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_local_addr(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_local_addr(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_set_timeout(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_set_timeout(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_set_nonblocking(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_set_nonblocking(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_shutdown(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_shutdown(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_tcp_close(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_tcp_close(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_bind(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_bind(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_connect(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_connect(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_send(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_send(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_send_to(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_send_to(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(", ");
    if args.len() >= 3 { cc.compile_expr(args[2])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_recv(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_recv(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_recv_from(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_recv_from(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(", ");
    if args.len() >= 2 { cc.compile_expr(args[1])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_udp_close(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_udp_close(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}

pub(super) fn compile_resolve(cc: &mut CCodegen, args: &[ExprId]) -> Result<(), Error> {
    cc.writer.write("rl_net_resolve(");
    if !args.is_empty() { cc.compile_expr(args[0])?; }
    cc.writer.write(")");
    Ok(())
}
