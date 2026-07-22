//! Typed signatures for `std::net`.

use super::{fixed, handle, handle_to_string, overloads, params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;
use std::rc::Rc;

pub fn module() -> ModuleNames {
    ModuleNames::new("net")
        .with_typed_function(tcp_listen())
        .with_typed_function(tcp_accept())
        .with_typed_function(tcp_connect())
        .with_typed_function(tcp_read())
        .with_typed_function(tcp_write())
        .with_typed_function(tcp_peer_addr())
        .with_typed_function(tcp_local_addr())
        .with_typed_function(tcp_set_timeout())
        .with_typed_function(tcp_set_nonblocking())
        .with_typed_function(tcp_shutdown())
        .with_typed_function(tcp_close())
        .with_typed_function(udp_bind())
        .with_typed_function(udp_connect())
        .with_typed_function(udp_send())
        .with_typed_function(udp_send_to())
        .with_typed_function(udp_recv())
        .with_typed_function(udp_recv_from())
        .with_typed_function(udp_close())
        .with_typed_function(resolve())
}

fn tcp_listen() -> StdFn {
    StdFn::typed(
        "tcp_listen",
        vec![(params(vec![T::String]), result(T::Int))],
    )
}

fn tcp_accept() -> StdFn {
    StdFn::typed("tcp_accept", overloads(vec![handle()], result(T::Int)))
}

fn tcp_connect() -> StdFn {
    StdFn::typed(
        "tcp_connect",
        vec![(params(vec![T::String]), result(T::Int))],
    )
}

fn tcp_read() -> StdFn {
    StdFn::typed(
        "tcp_read",
        overloads(vec![handle(), handle()], result(T::String)),
    )
}

fn tcp_write() -> StdFn {
    StdFn::typed(
        "tcp_write",
        overloads(vec![handle(), fixed(T::String)], result(T::Int)),
    )
}

fn tcp_peer_addr() -> StdFn {
    handle_to_string("tcp_peer_addr")
}
fn tcp_local_addr() -> StdFn {
    handle_to_string("tcp_local_addr")
}

fn tcp_set_timeout() -> StdFn {
    StdFn::typed(
        "tcp_set_timeout",
        overloads(vec![handle(), handle()], result(T::Null)),
    )
}

fn tcp_set_nonblocking() -> StdFn {
    StdFn::typed(
        "tcp_set_nonblocking",
        overloads(vec![handle(), fixed(T::Bool)], result(T::Null)),
    )
}

fn tcp_shutdown() -> StdFn {
    StdFn::typed(
        "tcp_shutdown",
        overloads(vec![handle(), fixed(T::String)], result(T::Null)),
    )
}

fn close(name: &'static str) -> StdFn {
    StdFn::typed(name, overloads(vec![handle()], result(T::Null)))
}
fn tcp_close() -> StdFn {
    close("tcp_close")
}
fn udp_close() -> StdFn {
    close("udp_close")
}

fn udp_bind() -> StdFn {
    StdFn::typed("udp_bind", vec![(params(vec![T::String]), result(T::Int))])
}

fn udp_connect() -> StdFn {
    StdFn::typed(
        "udp_connect",
        overloads(vec![handle(), fixed(T::String)], result(T::Null)),
    )
}

fn udp_send() -> StdFn {
    StdFn::typed(
        "udp_send",
        overloads(vec![handle(), fixed(T::String)], result(T::Int)),
    )
}

fn udp_send_to() -> StdFn {
    StdFn::typed(
        "udp_send_to",
        overloads(
            vec![handle(), fixed(T::String), fixed(T::String)],
            result(T::Int),
        ),
    )
}

fn udp_recv() -> StdFn {
    StdFn::typed(
        "udp_recv",
        overloads(vec![handle(), handle()], result(T::String)),
    )
}

fn udp_recv_from() -> StdFn {
    StdFn::typed(
        "udp_recv_from",
        overloads(
            vec![handle(), handle()],
            result(T::Tuple(Rc::new(vec![T::String, T::String]))),
        ),
    )
}

fn resolve() -> StdFn {
    StdFn::typed(
        "resolve",
        vec![(
            params(vec![T::String]),
            result(T::Array(Box::new(T::String))),
        )],
    )
}
